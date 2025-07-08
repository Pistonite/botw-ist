import type { AsyncErc } from "@pistonite/pure/memory";
import { idgen } from "@pistonite/pure/memory";

import {
    makeNativeHandleErc,
    type NativeApi,
    type NativeHandle,
} from "./NativeApi.ts";
import type { Pwr } from "./Error.ts";
import { log } from "./util.ts";

const getNextNativeHandleId = idgen();

export class TaskMgr {
    private napi: NativeApi;
    private tasks: Map<string, TaskContainer>;
    /** Map from ID to native handle */
    private nativeHandles: Map<number, NativeHandleContainer>;

    constructor(napi: NativeApi) {
        this.napi = napi;
        this.tasks = new Map();
        this.nativeHandles = new Map();
    }

    /**
     * Request and register a new native handle.
     * This does not add the requesting task as a dependency to the native handle yet
     */
    public async registerNativeHandle(requestingTaskId: string): Pwr<number> {
        const raw = await this.napi.makeTaskHandle();
        if (raw.err) {
            log.error(`${requestingTaskId}\nmakeTaskHandle failed.`);
            return raw;
        }
        const id = getNextNativeHandleId();
        log.debug(
            `${requestingTaskId}\nacquired native handle container #${id}, ptr=${raw.val}`,
        );
        const container = new NativeHandleContainer(
            this.napi,
            id,
            makeNativeHandleErc(raw.val),
        );
        this.nativeHandles.set(id, container);
        return { val: id };
    }

    public registerTask(taskId: string) {
        log.debug(`${taskId}\nregistering task`);
        if (this.tasks.has(taskId)) {
            log.error(
                `${taskId}\nregistering taskId that already exists! This is not supported!!!`,
            );
            return;
        }
        const container: TaskContainer = {
            isAborted: false,
            nativeHandleId: undefined,
        };
        this.tasks.set(taskId, container);
    }

    /** Try to abort the task, and abort the native task if no other task depends on it */
    public async abortTask(taskId: string) {
        log.debug(`${taskId}\nattempting to abort task`);
        // if we are trying to abort a task that doesn't exist...
        // there is no way to know if this is a task that's about to be scheduled
        // (and app wants it cancelled immediately), or if it's a task that's already
        // finished (in which case we just want to ignore it)
        // So, the simpliest solution, is to wait and see if this task gets registered
        const taskContainer = await this.waitForTaskToExist(taskId);
        if (!taskContainer) {
            log.debug(`${taskId}\nignoring abort request`);
            return;
        }
        log.debug(`${taskId}\naborting task`);
        taskContainer.isAborted = true;
        const nativeHandleId = taskContainer.nativeHandleId;
        if (nativeHandleId !== undefined) {
            this.removeNativeHandleDependency(taskId, nativeHandleId);
        }
    }

    /** Remove a task from the manager */
    public unregisterTask(taskId: string) {
        log.debug(`${taskId}\nunregistering task`);
        // we don't wait here, since unregisterTask is only called internally
        // when task is finished
        const taskContainer = this.tasks.get(taskId);
        if (!taskContainer) {
            log.debug(`${taskId}\ncannot find task to unregister, ignoring`);
            return;
        }
        this.tasks.delete(taskId);
        const nativeHandleId = taskContainer.nativeHandleId;
        if (nativeHandleId !== undefined) {
            this.removeNativeHandleDependency(taskId, nativeHandleId);
        }
    }

    /** Check the task exists and is not aborted */
    public async isTaskActive(taskId: string): Promise<boolean> {
        const taskContainer = await this.waitForTaskToExist(taskId);
        if (!taskContainer) {
            return false;
        }
        return !taskContainer.isAborted;
    }

    private async waitForTaskToExist(
        taskId: string,
    ): Promise<TaskContainer | undefined> {
        let taskContainer = this.tasks.get(taskId);
        if (!taskContainer) {
            log.debug(`${taskId}\nnon-existent task, waiting for it to exist`);
            const WAIT_LIMIT = 20;
            for (let i = 0; i < WAIT_LIMIT && !this.tasks.has(taskId); i++) {
                await new Promise((r) => setTimeout(r, 1000));
            }
            taskContainer = this.tasks.get(taskId);
            if (!taskContainer) {
                log.debug(`${taskId}\nstill does not exist after waiting`);
                return undefined;
            }
        }
        return taskContainer;
    }

    /** Get a strong reference to native handle by its id */
    public async getNativeHandle(
        nativeHandleId: number,
    ): Promise<AsyncErc<NativeHandle>> {
        const container = this.nativeHandles.get(nativeHandleId);
        if (!container) {
            return makeNativeHandleErc(undefined);
        }
        return await container.getHandle();
    }

    public addNativeHandleDependency(taskId: string, nativeHandleId: number) {
        log.debug(
            `${taskId}\nadding to native handle container #${nativeHandleId}`,
        );
        const taskContainer = this.tasks.get(taskId);
        if (!taskContainer) {
            log.error(
                `${taskId}\nis not registered`,
            );
            return;
        }
        const container = this.nativeHandles.get(nativeHandleId);
        if (!container) {
            log.error(
                `${taskId}\nreferences non-existent native container #${nativeHandleId}`,
            );
            return;
        }
        taskContainer.nativeHandleId = nativeHandleId;
        container.addTask(taskId);
    }

    private removeNativeHandleDependency(
        taskId: string,
        nativeHandleId: number,
    ) {
        const container = this.nativeHandles.get(nativeHandleId);
        if (!container) {
            // this is ok if the native container was already deleted
            return;
        }
        log.debug(
            `${taskId}\nremoving native handle container #${nativeHandleId}`,
        );
        const isNativeTaskAborted = container.abortTask(taskId);
        if (isNativeTaskAborted) {
            log.debug(
                `${taskId}\ndisposing native handle container #${nativeHandleId}`,
            );
            this.nativeHandles.delete(nativeHandleId);
        }
    }
}

type TaskContainer = {
    isAborted: boolean;
    nativeHandleId: number | undefined;
};

class NativeHandleContainer {
    private napi: NativeApi;
    /** Id of this native handle */
    private id: number;
    /** The handle used to abort task running in native code */
    private handle: AsyncErc<NativeHandle>;
    /** The tasks that currently depend on the native task */
    private owningTaskIds: string[];

    constructor(napi: NativeApi, id: number, handle: AsyncErc<NativeHandle>) {
        this.napi = napi;
        this.id = id;
        this.handle = handle;
        this.owningTaskIds = [];
    }

    /** Check if this native handle is no longer used */
    public isDisposed(): boolean {
        return this.owningTaskIds.length === 0 && !this.handle.value;
    }

    /** Add task dependency on this native task */
    public addTask(id: string) {
        if (!this.handle.value) {
            log.warn(
                `calling addTask on native handle container #${this.id}, which does not own a native handle pointer`,
            );
        }
        this.owningTaskIds.push(id);
    }

    public getHandle(): Promise<AsyncErc<NativeHandle>> {
        return this.handle.getStrong();
    }

    /**
     * Remove the task's dependency on this native handle
     * Returns true if all tasks dependencies are removed,
     * which will abort the native task
     */
    public abortTask(id: string): boolean {
        if (!this.owningTaskIds.length) {
            log.warn(
                `attempting to abort task ${id}, which does not depend on native handle container #${this.id}`,
            );
            // return false, since the native task might still be useful if new tasks are added.
            return false;
        }
        this.owningTaskIds = this.owningTaskIds.filter((x) => x !== id);
        if (!this.owningTaskIds.length) {
            log.debug(
                `aborting native handle container #${this.id} since all tasks are removed`,
            );
            const ptr = this.handle.take();
            if (ptr) {
                this.napi.abortTask(ptr);
            } else {
                log.warn(
                    `attempt abort on native handle container #${this.id}, which does not own a native handle pointer`,
                );
            }
            return true;
        }

        return false;
    }
}
