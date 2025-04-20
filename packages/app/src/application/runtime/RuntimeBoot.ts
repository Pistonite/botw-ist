import type { Void } from "@pistonite/pure/result";

import type {
    Runtime,
    RuntimeInitArgs,
} from "@pistonite/skybook-api";
import { translateGenericError, } from "skybook-localization";

import { useApplicationStore, useSessionStore } from "self::application/store";

/** Initialize the runtime with the given arguments */
export async function initRuntime(
    runtime: Runtime,
    args: RuntimeInitArgs,
): Promise<Void<string>> {
    const isCustomImage = args.isCustomImage;
    updateLogo(isCustomImage);
    console.log(`[boot] initializing runtime, custom image: ${isCustomImage}`);

    const initWorkerResult = await runtime.initialize(args);

    if (initWorkerResult.err) {
        console.warn("[boot] runtime initialization failed (worker)");
        if (isCustomImage) {
            useApplicationStore.getState().setCustomImageVersion("");
        }
        return { err: translateGenericError(initWorkerResult.err.message) };
    }
    const initResult = initWorkerResult.val;
    if (initResult.err) {
        console.warn("[boot] runtime initialization failed");
        if (isCustomImage) {
            useApplicationStore.getState().setCustomImageVersion("");
        }
        // TODO: worker error structure, tracked by #69
        return { err: "initResult.err" };
    }

    const { version, storedVersion } = initResult.val;
    if (storedVersion !== "not-changed") {
        console.log(`[boot] updating stored image version to: ${version}`);
        useApplicationStore.getState().setCustomImageVersion(version);
    }

    useSessionStore.getState().setRunningCustomImageVersion(version);

    console.log(`[boot] runtime initialized successfully, version: ${version}`);
    return {};
}

/** Update the logo in the boot screen and the favicon */
const updateLogo = (customImage: boolean) => {
    const image = customImage ? "/static/icon-purple.svg" : "/static/icon.svg";
    const linkIconTag = document.head.querySelector("link[rel='icon']");
    if (!linkIconTag) {
        const link = document.createElement("link");
        link.rel = "icon";
        link.type = "image/svg+xml";
        link.href = image;
        document.head.appendChild(link);
    } else {
        (linkIconTag as HTMLLinkElement).href = image;
    }

    const bootLogo = document.querySelector(".-boot-logo- img");
    if (!bootLogo || (bootLogo as HTMLImageElement).src === image) {
        return;
    }
    (bootLogo as HTMLImageElement).src = image;
};
