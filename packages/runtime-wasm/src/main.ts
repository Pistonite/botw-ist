import { wxWorkerGlobal } from "@pistonite/workex";

import { skybookRuntimeApp } from "@pistonite/skybook-api/interfaces/RuntimeApp.bus";
import {
    resolveAppPromise,
    initExternalRefCountTypes,
    TaskMgr,
    ParseMgr,
    RunMgr,
    createRuntimeWorker,
} from "skybook-runtime-worker";

import { WasmApi } from "./WasmApi.ts";
import { IndexedDBImageMgr } from "./IndexedDBImageMgr.ts";

async function boot() {
    const wapi = new WasmApi();
    await wapi.initWasmModule();
    await initExternalRefCountTypes(wapi);
    const taskMgr = new TaskMgr(wapi, 2); // 2 threads for now
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
