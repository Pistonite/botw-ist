import type { AsyncErc } from "@pistonite/pure/memory";
import type { Result } from "@pistonite/pure/result";

import {
    makeNativeHandleErc,
    type NativeApi,
    type NativeHandle,
} from "./NativeApi.ts";
import type { WorkerError } from "./Error.ts";

export const TaskState = {
    /** Task is just created and pending to be executed */
    Created: 0,
    /** Task is actively being executed on a resource */
    Running: 1,
    /** Task is being awaited on, but not holding on to a resource */
    Pending: 2,
    /** Task is done */
    Finished: 3,
    /** Task has been requested to be aborted */
    Aborted: 4,
} as const;
export type TaskState = (typeof TaskState)[keyof typeof TaskState];

export class TaskHandle {
    private napi: NativeApi;
    private state: TaskState;
    private nativeHandle: AsyncErc<NativeHandle>;

    constructor(napi: NativeApi) {
        this.napi = napi;
        this.state = TaskState.Created;
        this.nativeHandle = makeNativeHandleErc(undefined);
    }

    public isAborted() {
        return this.state === TaskState.Aborted;
    }

    public isDone() {
        return (
            this.state === TaskState.Finished ||
            this.state === TaskState.Aborted
        );
    }

    public isUsingResource() {
        return this.state === TaskState.Running;
    }

    /** Get a strong reference to the native handle (caller is in charge of freeing it) */
    public getNativeHandle(): Promise<AsyncErc<NativeHandle>> {
        return this.nativeHandle.getStrong();
    }

    /** Request the task to be aborted */
    public abort() {
        if (this.state === TaskState.Finished) {
            // task is already finished so it cannot be aborted
            return;
        }
        this.state = TaskState.Aborted;
        // transfer ptr ownership to native
        const ptr = this.nativeHandle.take();
        if (ptr === undefined) {
            return;
        }
        this.napi.abortTask(ptr);
    }

    /**
     * Mark the task as running on the native side with a native resource
     *
     * Returns true if marked as running successfully, which means the ownership
     * of the native handle is transferred to this handle
     */
    public async markRunningWithNativeHandle(nativeHandleRaw: number): Promise<boolean> {
        if (this.state === TaskState.Aborted) {
            // aborted before the task can even start, which is fine
            return false;
        }
        if (this.state !== TaskState.Created) {
            console.warn(
                `cannot mark task as running, task is already in state: ${this.state}. This is a bug!`,
            );
            return false;
        }
        await this.nativeHandle.assign(nativeHandleRaw);
        this.state = TaskState.Running;
        return true;
    }

    public markRunning() {
        if (this.state === TaskState.Aborted) {
            // aborted before the task can even start, which is fine
            return;
        }
        if (this.state !== TaskState.Created) {
            console.warn(
                `cannot mark task as running, task is already in state: ${this.state}. This is a bug!`,
            );
            return;
        }
        this.state = TaskState.Pending;
    }

    /**
     * Mark the task as finished. If the task previously owns a native handle,
     * it will be freed
     */
    public markFinish() {
        if (this.state !== TaskState.Aborted) {
            this.state = TaskState.Finished;
        }
        void this.nativeHandle.free();
    }
}

export class TaskMgr {
    private napi: NativeApi;
    /** The max number of tasks that can hold on to a native resource at a time */
    private nativeConcurrency: number;
    /** Task ID to TaskHandle */
    private tasks: Map<string, TaskHandle>;

    private waiters: (() => void)[] = [];

    constructor(napi: NativeApi, nativeConcurrency: number) {
        this.napi = napi;
        this.nativeConcurrency = Math.max(1, nativeConcurrency);
        this.tasks = new Map();
    }

    /** Make sure a handle exists for the given id */
    public register(id: string) {
        this.getHandle(id);
    }

    /** Abort the task with the given id. This does not delete the handle yet */
    public abort(id: string) {
        const handle = this.tasks.get(id);
        if (!handle) {
            // task is already done or never started
            return;
        }
        const shouldNotify = handle.isUsingResource();
        handle.abort();
        if (shouldNotify) {
            this.notifyWaiters();
        }
    }

    /** Check if the task is aborted */
    public isAborted(id: string): boolean {
        const handle = this.tasks.get(id);
        if (handle) {
            return handle.isAborted();
        }
        // task is not found, so it is aborted previously
        return true;
    }

    /** Mark a task as finished and delete the handle */
    public finish(id: string) {
        const handle = this.tasks.get(id);
        if (!handle) {
            return;
        }
        this.tasks.delete(id);
        const shouldNotify = handle.isUsingResource();
        handle.markFinish();
        if (shouldNotify) {
            this.notifyWaiters();
        }
    }

    public run(id: string) {
        this.getHandle(id).markRunning();
    }

    /**
     * Create a native handle and associate it with the task id.
     *
     * If the max native concurrency is reached, it will wait for a task
     * to finish.
     *
     * If the task is aborted while waiting for a resource, an error is returned
     */
    public async acquireNativeResourceAndRun(
        id: string,
    ): Promise<Result<AsyncErc<NativeHandle>, WorkerError>> {
        const handle = this.getHandle(id);
        if (this.countResourceUsage() >= this.nativeConcurrency) {
            console.log(`[task] ${id} is waiting on a native resource`);
            do {
                await new Promise<void>((resolve) => {
                    this.waiters.push(resolve);
                });
            } while (this.countResourceUsage() >= this.nativeConcurrency);
            // make sure task is not aborted while waiting on a resource
            if (handle.isAborted()) {
                return getAbortError(id);
            }
        }
        const raw = await this.napi.makeTaskHandle();
        if (raw.err) {
            return raw;
        }

        // we need to check for abort at every async step here to ensure
        // we return an error to the caller, otherwise, the abort signal
        // is not sent to the native side, and it will not abort

        if (!(await handle.markRunningWithNativeHandle(raw.val))) {
            // since the task cannot start, we must free the native handle here
            void makeNativeHandleErc(raw.val).free();
            return getAbortError(id);
        }
        console.log(`[task] ${id} acquired a native resource`);
        const nativeHandle = await handle.getNativeHandle();
        if (handle.isAborted()) {
            await nativeHandle.free();
            return getAbortError(id);
        }
        return { val: nativeHandle };
    }

    private notifyWaiters() {
        if (this.countResourceUsage() >= this.nativeConcurrency) {
            return;
        }
        const oldWaiters = this.waiters;
        this.waiters = [];
        for (const waiter of oldWaiters) {
            waiter();
        }
    }

    private countResourceUsage() {
        let count = 0;
        for (const handle of this.tasks.values()) {
            if (handle.isUsingResource()) {
                count++;
            }
        }
        return count;
    }

    /** Get or create a task handle with the id */
    private getHandle(id: string) {
        const existing = this.tasks.get(id);
        if (existing) {
            return existing;
        }
        const handle = new TaskHandle(this.napi);
        this.tasks.set(id, handle);
        return handle;
    }
}

const getAbortError = (id: string) => {
    return {err: {
        type: "Aborted",
        message: `Task ${id} was aborted while waiting for a native resource`,
    }} as const;
}
