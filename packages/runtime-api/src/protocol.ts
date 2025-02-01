import { WorkexPromise } from "@pistonite/workex";

/**
 * API provided by the simulator runtime
 *
 * @workex:send app
 * @workex:recv runtime
 */
export interface RuntimeApi {

    /** 
     * Set the script for the runtime, which starts executing
     * the script immediately
     */
    setScript(script: string): WorkexPromise<string>;
}
