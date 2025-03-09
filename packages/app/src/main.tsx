import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { addLocaleSubscriber, initDark } from "@pistonite/pure/pref";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Void } from "@pistonite/pure/result";

import { initI18n, translateUI } from "skybook-localization";
import { ItemTooltipProvider } from "skybook-item-system";
import { extractDirectLoad } from "@pistonite/skybook-api/client";
import { 
    DirectLoad, 
    parseEnvFromScript, 
    getInitParamsFromEnv,
    RuntimeInitParams, ScriptEnvImage } from "@pistonite/skybook-api";

import { 
    initExtensionManager, 
    createExtensionAppHost,
    ExtensionAppContext,
} from "application/extension";
import { initRuntime } from "application/runtime";
import { useApplicationStore, useSessionStore } from "application/store";

import { initNarrow } from "pure-contrib/narrow.ts";
import { isLessProductive } from "pure-contrib/platform.ts";

import { App } from "./App.tsx";
import { ThemeProvider } from "./theme/ThemeProvider.tsx";

import {
    getSheikaBackgroundUrl,
    probeAndRegisterAssetLocation,
} from "ui/asset.ts";
import { BootScreen, BootScreenProps } from "ui/BootScreen.tsx";
import { RuntimeClient } from "@pistonite/skybook-api/interfaces/Runtime.send";
import { translateGenericError } from "../../localization/src/translation/error.ts";

void boot();

/**
 * Application boot flow
 *
 * 1. We start by kicking off loading assets (images, strings)
 * 2. Load the script from store or DirectLoad, and initialize the runtime 
 *    with default or custom image. This will mount a temporary React root
 *    for showing dialogs
 * 3. Mount the main React root with the app
 */
async function boot() {
    initDark({ persist: true, });

    // start initializing the runtime early
    const runtime = initRuntime();

    if (isLessProductive) {
        initNarrow({
            threshold: 800,
            override: (narrow) => {
                if (window.innerWidth < window.innerHeight) {
                    return true;
                }
                if (narrow && window.innerHeight < window.innerWidth) {
                    return false;
                }
                return narrow;
            },
        });
    } else {
        initNarrow({
            threshold: 800,
        });
    }

    const beforeMainUI = async () => {
        const promises = [
            probeAndRegisterAssetLocation(), 
        ];
        await Promise.all(promises);
    };

    const beforeBootUI = async () => {
        await initI18n();
        addLocaleSubscriber(() => {
            const title = translateUI("title");
            // fallback in case translation failed to load
            if (title === "title") {
                document.title = "IST Simulator";
            } else {
                document.title = title;
            }
        }, true);
    };

    const params = new URLSearchParams(window.location.search);

    // Begin boot flow
    const payload = extractDirectLoad();
    const context: BootContext = {
        beforeBootUI,
        beforeMainUI,
        runtime,
        unmountBootUI: undefined,
        setup: !!params.get("setup")
    };

    if (payload) {
        await bootWithDirectLoad(payload, context);
        return;
    }

    await bootWithLocalScript(context);
}

type BootContext = {
    beforeBootUI: () => Promise<void>,
    beforeMainUI: () => Promise<void>,
    unmountBootUI: (() => void) | undefined,
    runtime: Promise<RuntimeClient>,
    /** If the runtime setup dialog should always be displayed */
    setup: boolean,
}

async function bootWithDirectLoad(payload: DirectLoad, context: BootContext) {
    console.log("[boot] found direct load payload");
    const { setMode } = useSessionStore.getState();
    if (payload.edit) {
        setMode("edit-only");
    } else {
        setMode("read-only");
    }
    // for boot purpose, we ignore errors during parsing the env
    const env = parseEnvFromScript(payload.content);

    if (context.setup) {
        console.log("[boot] setup mode requested");
        await continueBootWithDialog({
            initialState: "SetupDialog",
        }, context);
        return
    }
    
    // Does the script require CI?
    if (!env.image || env.image === "default") {
        console.log("[boot] custom image not required, using default image");
        await continueBootWithDefaultImage();
        return;
    }

    const { 
        customImageVersion, 
        isUseCustomImageByDefault,
    } = useApplicationStore.getState();

    console.log(`[boot] direct load requests custom image: ${env.image}`);

    const versionMatch = checkImageVersion(customImageVersion, env.image);
    if (versionMatch === "ok") {
        console.log("[boot] found matching custom image version");
        if (payload.edit) {
            // for edit sessions, allow custom image to be loaded by default
            // without prompts
            if (isUseCustomImageByDefault) {
                await continueBootWithCustomImageWithNoDialog(context, getInitParamsFromEnv(env));
                return;
            }
        }
        await continueBootWithDialog({
            initialState: "UseCustomOrUseDefaultImage",
        }, context);
        return;
    }
    if (versionMatch === "no-image") {
        console.log("[boot] no custom image found");
        await continueBootWithDialog({
            initialState: "OpenSetupOrUseDefaultImage",
            openSetupOrDefaultPromptType: "DirectLoadNoImage",
        }, context);
        return;
    }
    console.log("[boot] custom image version mismatch");
    await continueBootWithDialog({
        initialState: "OpenSetupOrUseDefaultImage",
        openSetupOrDefaultPromptType: "DirectLoadVersionMismatch",
    }, context);
    return;
}

async function bootWithLocalScript(context: BootContext) {
    console.log("[boot] loading local script");
    const { 
        script, 
        customImageVersion, 
        isUseCustomImageByDefault,
        setUseCustomImageByDefault
    } = useApplicationStore.getState();
    // for boot purpose, we ignore errors during parsing the env
    const env = parseEnvFromScript(script);

    if (context.setup) {
        console.log("[boot] setup mode requested");
        await continueBootWithDialog({
            initialState: "SetupDialog",
        }, context);
        return
    }

    // Does the script require CI?
    if (env.image && env.image !== "default") {
        console.log(`[boot] local script requests custom image: ${env.image}`);
        // Check the version required by the script
        const versionMatch = checkImageVersion(customImageVersion, env.image);
        if (versionMatch === "ok") {
            console.log("[boot] found matching custom image version");
            await continueBootWithCustomImage();
        } else if (versionMatch === "no-image") {
            console.log("[boot] no custom image found");
            await continueBootWithDialog({
                initialState: "OpenSetupOrUseDefaultImage",
                openSetupOrDefaultPromptType: "LocalNoImage",
            });
        } else {
            console.log("[boot] custom image version mismatch");
            await continueBootWithDialog({
                initialState: "OpenSetupOrUseDefaultImage",
                openSetupOrDefaultPromptType: "LocalVersionMismatch",
            });
        }
        return;
    }

    // Script doesn't require CI, check if we have should use CI by default
    if (isUseCustomImageByDefault) {
        // check we have a valid stored image version
        if (customImageVersion === "ver1.5" || customImageVersion === "ver1.6") {
            console.log("[boot] loading custom image by default");
            await continueBootWithCustomImageNoDialog();
            return;
        }
        // if the CI version is invalid, clear use custom image by default
        setUseCustomImageByDefault(false);
    }

    console.log("[boot] loading default image");
    await continueBootWithDefaultImage();
}

type CheckImageVersionResult = "no-image" | "mismatch" | "ok";
const checkImageVersion = (storedVersion: string, spec: ScriptEnvImage): CheckImageVersionResult => {
    if (storedVersion !== "ver1.5" && storedVersion !== "ver1.6") {
        return "no-image";
    }
    if (spec.startsWith("custom-anyver")) {
        return "ok";
    }
    if (spec.startsWith("custom-ver1.5") && storedVersion === "ver1.5") {
        return "ok";
    }
    if (spec.startsWith("custom-ver1.6") && storedVersion === "ver1.6") {
        return "ok";
    }
    return "mismatch";
}

async function continueBootWithDialog(bootScreenProps: BootScreenProps, context: BootContext) {
    // if not mounted already, mount temporary root for showing dialog
    if (!context.unmountBootUI) {
        await context.beforeBootUI();
        context.beforeBootUI = async () => {};
        const rootElement = document.getElementById("-boot-root-") as HTMLDivElement;
        const root = createRoot(rootElement);
        context.unmountBootUI = () => {
            root.unmount();
        }
        root.render(<StrictMode>
            <BootScreen {...bootScreenProps} />
        </StrictMode>);
    }
}

async function continueBootWithCustomImageWithNoDialog(
    context: BootContext, 
    params: RuntimeInitParams
) {
    const result = await continueBootWithCustomImage(context, params);
    if (result.err) {
            await continueBootWithDialog({
                initialState: "OpenSetupOrUseDefaultImage",
                openSetupOrDefaultPromptType: "InitializeError",
            }, context);
    }
}

async function continueBootWithCustomImage(
    context: BootContext, 
    params: RuntimeInitParams
): Promise<Void<string>> {
    updateLogo(true);
    const runtime = await context.runtime;
    console.log("[boot] initializing runtime with custom image");
    const initWorkerResult = await runtime.initialize({
        isCustomImage: true,
        params
    });

    if (initWorkerResult.err) {
        console.warn("[boot] custom image initialization failed (worker)");
        return {err: translateGenericError(initWorkerResult.err.message)};
    }
    const initResult = initWorkerResult.val;
    if (initResult.err) {
        console.warn("[boot] custom image initialization failed");
        // TODO: worker error structure, tracked by #69
        return {err: "initResult.err"};
    }

    console.log("[boot] custom image initialized successfully");
    await bootMainUI(context);
    return {};
}

async function continueBootWithDefaultImage() {
    updateLogo(false);
}

/** Update the logo in the boot screen and the favicon */
function updateLogo(customImage: boolean) {
    const image = customImage ? "/static/icon.svg" : "/static/icon-purple.svg";
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
    if (!bootLogo) {
        return;
    }
    (bootLogo as HTMLImageElement).src = image;
}

async function bootMainUI(context: BootContext) {
    console.log("[boot] booting main UI");
    await context.beforeBootUI(); // needed if boot UI is never shown
    await context.beforeMainUI();
    context.unmountBootUI?.();

    const queryClient = new QueryClient();
    const app = createExtensionAppHost(await context.runtime);
    initExtensionManager();

    const root = document.getElementById("-root-") as HTMLDivElement;
    createRoot(root).render(
        <StrictMode>
            <ExtensionAppContext.Provider value={app}>
                <QueryClientProvider client={queryClient}>
                    <ThemeProvider>
                        <ItemTooltipProvider
                            backgroundUrl={getSheikaBackgroundUrl()}
                        >
                            <App />
                        </ItemTooltipProvider>
                    </ThemeProvider>
                </QueryClientProvider>
            </ExtensionAppContext.Provider>
        </StrictMode>,
    );
}
