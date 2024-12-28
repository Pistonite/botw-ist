import { WorkexPromise } from "workex";

/**
 * API provided by the simulator runtime
 *
 * @workex:send app
 * @workex:recv runtime
 */
export interface RuntimeApi {

    parseScript(script: string): WorkexPromise<string>;
}
