import { wxWrapHandler } from "@pistonite/workex";
import type { Result } from "@pistonite/pure/result";

import type {
    Runtime,
    RuntimeWorkerInitArgs,
    RuntimeWorkerInitOutput,
    ScriptEnvImage,
    RuntimeInitParams,
    RuntimeWorkerInitError,
} from "@pistonite/skybook-api";

import type { ParseMgr } from "./parse_mgr.ts";
import type { NativeApi } from "./native_api.ts";
import type { TaskMgr } from "./task_mgr.ts";
import type { RunMgr } from "./run_mgr.ts";
import type { ImageMgr } from "./image_mgr.ts";
import { unwrap, type Pwr, unwrapMaybeAborted } from "./error.ts";
import { getCustomBlueFlameImage } from "./app_call.ts";

export const createRuntimeWorker = <TPtr>(
    napi: NativeApi<TPtr>,
    taskMgr: TaskMgr<TPtr>,
    parseMgr: ParseMgr<TPtr>,
    runMgr: RunMgr<TPtr>,
    imageMgr: ImageMgr,
): Runtime => {
    return {
        initialize: async (args) => {
            return {
                val: unwrap(
                    await initializeRuntimeWorker(napi, imageMgr, args),
                ),
            };
        },
        resolveItemIdent: async (query) => {
            return { val: unwrap(await napi.resolveItemIdent(query)) };
        },
        getParserDiagnostics: async (script) => {
            return { val: unwrap(await parseMgr.getParserDiagnostics(script)) };
        },
        getSemanticTokens: async (script, start, end) => {
            return {
                val: unwrap(await napi.parseScriptSemantic(script, start, end)),
            };
        },
        getStepFromPos: async (script, pos) => {
            return { val: unwrap(await parseMgr.getStepFromPos(script, pos)) };
        },
        getStepBytePositions: async (script) => {
            return { val: unwrap(await parseMgr.getStepBytePositions(script)) };
        },
        abortTask: wxWrapHandler(async (taskId) => {
            await taskMgr.abortTask(taskId);
        }),
        executeScript: wxWrapHandler(async (script, taskId) => {
            await runMgr.triggerFullExecution(script, taskId);
        }),
        getRuntimeDiagnostics: async (script, taskId, bytePos) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getRuntimeDiagnostics(script, taskId, bytePos),
                ),
            };
        },
        getPouchList: async (script, taskId, pos) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getPouchList(script, taskId, pos),
                ),
            };
        },
        getGdtInventory: async (script, taskId, pos) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getGdtInventory(script, taskId, pos),
                ),
            };
        },
        getOverworldItems: async (script, taskId, pos) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getOverworldItems(script, taskId, pos),
                ),
            };
        },
        getCrashInfo: async (script, taskId, pos) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getCrashInfo(script, taskId, pos),
                ),
            };
        },
        getSaveNames: async (script, taskId, pos) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getSaveNames(script, taskId, pos),
                ),
            };
        },
        getSaveInventory: async (script, taskId, pos, name) => {
            return {
                val: unwrapMaybeAborted(
                    await runMgr.getSaveInventory(script, taskId, pos, name),
                ),
            };
        },
    };
};

const initializeRuntimeWorker = async <TPtr>(
    napi: NativeApi<TPtr>,
    imageMgr: ImageMgr,
    args: RuntimeWorkerInitArgs,
): Pwr<Result<RuntimeWorkerInitOutput, RuntimeWorkerInitError>> => {
    if (args.isCustomImage) {
        return await initializeRuntimeWorkerWithCustomImage(
            napi,
            imageMgr,
            args.params,
            args.alwaysAskApp,
        );
    }
    let storedVersion: "not-changed" | "" = "not-changed";
    if (args.deleteCustomImage) {
        await imageMgr.putImage(undefined);
        storedVersion = "";
    }
    const result = await napi.initRuntime(undefined, undefined);
    // IPC error
    if (result.err) {
        return result;
    }
    // runtime init error
    if (result.val.err) {
        return { val: { err: result.val.err } };
    }
    return {
        val: {
            val: {
                version: "",
                storedVersion,
            },
        },
    };
};

const initializeRuntimeWorkerWithCustomImage = async <TPtr>(
    napi: NativeApi<TPtr>,
    imageMgr: ImageMgr,
    params: RuntimeInitParams,
    alwaysAskApp: boolean,
): Pwr<Result<RuntimeWorkerInitOutput, RuntimeWorkerInitError>> => {
    // try reading the image from the database
    let customImage = alwaysAskApp ? undefined : await imageMgr.getImage();
    if (!customImage) {
        // try requesting the image from the app
        const newImage = await getCustomBlueFlameImage();
        if (newImage.err || !newImage.val) {
            return { val: { err: { type: "NoImageFromApp" } } };
        }
        // save the image
        const ok = await imageMgr.putImage(newImage.val);
        if (!ok) {
            // technically we can still use the image in memory,
            // but the state will be inconsistency the next time
            return { val: { err: { type: "SaveImage" } } };
        }
        customImage = newImage.val;
    }

    const result = await napi.initRuntime(customImage, params);
    // IPC error
    if (result.err) {
        return result;
    }
    // runtime init error
    if (result.val.err) {
        return { val: { err: result.val.err } };
    }
    const output = result.val.val;
    const version = output.gameVersion as ScriptEnvImage;
    return {
        val: {
            val: {
                version,
                storedVersion: version,
            },
        },
    };
};
