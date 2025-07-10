import { wxWorkerGlobal } from "@pistonite/workex";

import { skybookRuntimeApp } from "@pistonite/skybook-api/interfaces/RuntimeApp.bus";
import {
    resolveAppPromise,
    TaskMgr,
    ParseMgr,
    RunMgr,
    createRuntimeWorker,
} from "skybook-runtime-worker";

import { WasmApi } from "./wasm_api.ts";
import { IndexedDBImageMgr } from "./idb_image_mgr.ts";

async function boot() {
    const wapi = new WasmApi();
    await wapi.initWasmModule();
    const taskMgr = new TaskMgr(wapi);
    const parseMgr = new ParseMgr(wapi);
    const runMgr = new RunMgr(wapi, parseMgr, taskMgr);

    const api = createRuntimeWorker(
        wapi,
        taskMgr,
        parseMgr,
        runMgr,
        IndexedDBImageMgr,
    );

    await wxWorkerGlobal()({
        app: skybookRuntimeApp(api, resolveAppPromise),
    });
}

void boot();
