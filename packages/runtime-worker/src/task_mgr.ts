import { type Emp, idgen } from "@pistonite/pure/memory";

import type { NativeApi, NativeHandle } from "./native_api.ts";
import type { Pwr } from "./error.ts";
import { log } from "./util.ts";

const getNextNativeHandleId = idgen();

/**
 * Manages simulation run tasks
 *
 * A "task" is defined as a job that has the simulation script as input,
 * and need to execute the script to get output. Since simulation is Pure,
 * multiple tasks that depend on the same script can be batched together
 * as the same simulation run, and take output at different steps.
 *
 * TaskMgr is used internally by the RunMgr to manage such batching behavior.
 *
 * A Task in TaskMgr is identified by a string ID, typically a UUID.
 * It can have the following states:
 * - Registered: call `registerTask` to start tracking the task by TaskMgr
 * - Aborted: call `abortTask` to abort a Task
 * - Finished: A task is finished when the simulation has reached the step needed
 *   by the task, or when the simulation run has ended or aborted.
 *   The RunMgr will unregister the task
 *
 * When all tasks are aborted on a native handle, there is a grace period
 * for the native task to abort. This is to accommodate messaging delay
 * from the app.
 */
export class TaskMgr<TPtr> {
    private napi: NativeApi<TPtr>;
    private tasks: Map<string, TaskContainer>;
    /** Map from ID to native handle */
    private nativeHandles: Map<number, NativeHandleContainer<TPtr>>;

    constructor(napi: NativeApi<TPtr>) {
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
            this.napi.makeNativeHandleEmp(raw.val),
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
        taskContainer.nativeHandleId = undefined;
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
            const WAIT_LIMIT = 10;
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

    /** Get a reference to native handle by its id */
    public getNativeHandle(
        nativeHandleId: number,
    ): Emp<NativeHandle, TPtr> | undefined {
        const container = this.nativeHandles.get(nativeHandleId);
        if (!container) {
            return undefined;
        }
        return container.getHandle();
    }

    public addNativeHandleDependency(
        taskId: string,
        nativeHandleId: number,
    ): boolean {
        log.debug(
            `${taskId} NA#${nativeHandleId}\nadding to native handle container`,
        );
        const taskContainer = this.tasks.get(taskId);
        if (!taskContainer) {
            log.error(`${taskId}\nis not registered`);
            return false;
        }
        const container = this.nativeHandles.get(nativeHandleId);
        if (!container) {
            log.error(
                `${taskId} NA#${nativeHandleId}\nnative handle container does not exist`,
            );
            return false;
        }
        taskContainer.nativeHandleId = nativeHandleId;
        container.addTask(taskId);
        return true;
    }

    private removeNativeHandleDependency(
        taskId: string,
        nativeHandleId: number,
    ) {
        const container = this.nativeHandles.get(nativeHandleId);
        if (!container) {
            log.warn(
                `${taskId} NA#${nativeHandleId}\nnative handler container does not exist`,
            );
            return;
        }
        log.debug(
            `${taskId} NA#${nativeHandleId}\nremoving task from native handle container`,
        );
        const shouldNativeTaskAborted = container.abortTask(taskId);
        if (!shouldNativeTaskAborted) {
            return;
        }
        void container.scheduleAbortNativeTask().then((isAborted) => {
            if (isAborted) {
                this.unregisterNativeHandle(nativeHandleId);
            }
        });
    }

    private unregisterNativeHandle(nativeHandleId: number) {
        this.nativeHandles.delete(nativeHandleId);
        const remaining = this.nativeHandles.size;
        log.debug(
            `NA#${nativeHandleId}\nunregistered. ${remaining} container remaining.`,
        );
        if (remaining === 0) {
            log.info("all native handle containers unregistered");
        }
    }
}

type TaskContainer = {
    isAborted: boolean;
    nativeHandleId: number | undefined;
};

class NativeHandleContainer<TPtr> {
    private napi: NativeApi<TPtr>;
    /** Id of this native handle */
    private id: number;
    /** The handle used to abort task running in native code */
    private handle: Emp<NativeHandle, TPtr>;
    /** The tasks that currently depend on the native task */
    private owningTaskIds: string[];
    private isAbortingNativeTaskScheduled: boolean;

    constructor(
        napi: NativeApi<TPtr>,
        id: number,
        handle: Emp<NativeHandle, TPtr>,
    ) {
        this.napi = napi;
        this.id = id;
        this.handle = handle;
        this.owningTaskIds = [];
        this.isAbortingNativeTaskScheduled = false;
    }

    /** Add task dependency on this native task */
    public addTask(id: string) {
        this.owningTaskIds.push(id);
    }

    public getHandle(): Emp<NativeHandle, TPtr> {
        return this.handle;
    }

    /**
     * Remove the task's dependency on this native handle
     * Returns true if all tasks dependencies are removed,
     * which means the native handle can be scheduled to abort
     */
    public abortTask(id: string): boolean {
        if (this.owningTaskIds.length) {
            this.owningTaskIds = this.owningTaskIds.filter((x) => x !== id);
        }
        return !this.owningTaskIds.length;
    }

    /**
     * Abort the native task, if no more tasks are coming in after a grace period
     * Return true after the native task is aborted, or false if no abort
     */
    public async scheduleAbortNativeTask(): Promise<boolean> {
        if (this.isAbortingNativeTaskScheduled) {
            return false; // already in the loop below in some other context
        }
        const WAIT_LIMIT = 3;
        this.isAbortingNativeTaskScheduled = true;
        await new Promise((r) => setTimeout(r, WAIT_LIMIT * 1000));
        this.isAbortingNativeTaskScheduled = false;
        if (this.owningTaskIds.length) {
            log.debug(
                `NA#${this.id}\nnot aborting native handle container - tasks were added`,
            );
            this.isAbortingNativeTaskScheduled = false;
            return false;
        }
        log.debug(`NA#${this.id}\naborting native handle`);
        // "this" is holding a strong ref to handle, so the call is safe
        this.napi.abortTask(this.handle.value);
        return true;
    }
}
