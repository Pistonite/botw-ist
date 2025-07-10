import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { addLocaleSubscriber, initDark } from "@pistonite/pure/pref";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import type { Void } from "@pistonite/pure/result";
import { ThemeProvider } from "@pistonite/shared-controls";
import { wxWorker } from "@pistonite/workex";

import { type Translator, initI18n, translateUI } from "skybook-localization";
import { ItemTooltipProvider } from "skybook-item-system";
import { extractDirectLoad } from "@pistonite/skybook-api/client";
import {
    type Runtime,
    type DirectLoad,
    parseEnvFromScript,
    type ScriptEnvImage,
    type RuntimeWorkerInitArgs,
    type ScriptEnv,
} from "@pistonite/skybook-api";
import { skybookRuntime } from "@pistonite/skybook-api/interfaces/Runtime.bus";

import {
    initExtensionManager,
    initExtensionAppHost,
    bootstrapAppWithRuntime,
    initRuntime,
    createRuntimeAppHost,
    RuntimeContext,
    loadRecoveryScriptIfNeeded,
    registerCrashHandler,
    usePersistStore,
    useSessionStore,
    isCrashed,
} from "self::application";
import { initNarrow, isLessProductive } from "self::pure-contrib";
import {
    bootLog,
    devLog,
    getSheikaBackgroundUrl,
    probeAndRegisterAssetLocation,
} from "self::util";
import {
    App,
    BootScreen,
    CrashScreen,
    type BootScreenProps,
    CatchCrash,
} from "self::ui/surfaces/root";

const VALID_VERSIONS = ["1.5.0", "1.6.0", "1.8.2"];

const createReactRoot = () => {
    const root = document.getElementById("-root-") as HTMLDivElement;
    return createRoot(root);
};
let ReactRoot: ReturnType<typeof createReactRoot> | undefined = undefined;

const createWasmRuntimeWorker = async (): Promise<Runtime> => {
    const appHost = createRuntimeAppHost();
    // create the runtime worker
    let url: string;
    if (import.meta.env.DEV) {
        devLog.info("using local runtime worker");
        url = "/runtime/worker.js";
    } else {
        const commitShort = import.meta.env.COMMIT.substring(0, 8);
        url = `/runtime/worker-${commitShort}.min.js`;
    }
    const worker = new Worker(url);
    const result = await wxWorker(worker)({
        runtime: skybookRuntime(appHost),
    });

    if (result.err) {
        bootLog.error("failed to connect to runtime worker");
        bootLog.error(result.err);
        throw new Error(
            "fatal boot failure: failed to connect to runtime worker",
        );
    }

    return result.val.protocols.runtime;
};

/**
 * Application boot flow
 *
 * 1. We start by kicking off loading assets (images, strings)
 * 2. Load the script from store or DirectLoad, and initialize the runtime
 *    with default or custom image. This will mount a temporary React root
 *    for showing dialogs
 * 3. Mount the main React root with the app
 */
const boot = async () => {
    let context: BootContext | undefined = undefined;
    registerCrashHandler(() => {
        context?.unmountBootUI?.();
        (ReactRoot || createReactRoot()).render(<CrashScreen />);
        void removeBootCurtain(false);
    });
    initDark({ persist: true });

    // start initializing the runtime early
    const runtime = createWasmRuntimeWorker();
    void runtime.then(bootstrapAppWithRuntime);

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
        if (isCrashed()) {
            return;
        }
        const promises = [probeAndRegisterAssetLocation()];
        await Promise.all(promises);
    };

    const beforeBootUI = async () => {
        if (isCrashed()) {
            return;
        }
        await initI18n(true);
    };

    context = {
        beforeBootUI,
        beforeMainUI,
        runtime,
        unmountBootUI: undefined,
    };

    const payload = extractDirectLoad();

    if (payload) {
        await bootWithDirectLoad(context, payload);
        return;
    }

    await bootWithLocalScript(context);
};

type BootContext = {
    beforeBootUI: () => Promise<void>;
    beforeMainUI: () => Promise<void>;
    unmountBootUI: (() => void) | undefined;
    runtime: Promise<Runtime>;
};

const bootWithDirectLoad = async (
    context: BootContext,
    payload: DirectLoad,
) => {
    bootLog.info("found direct load payload");
    const { setModeToEditOnly, setModeToReadOnly } = useSessionStore.getState();
    if (payload.edit) {
        setModeToEditOnly(payload.content);
    } else {
        setModeToReadOnly(payload.content);
    }
    // for boot purpose, we ignore errors during parsing the env
    const env = parseEnvFromScript(payload.content);

    // Does the script require CI?
    if (!env.image) {
        bootLog.info("custom image not required, using default image");
        await continueBootWithDefaultImage(context, env);
        return;
    }

    const { customImageVersion, isUseCustomImageByDefault } =
        usePersistStore.getState();

    bootLog.info(`direct load requests custom image: ${env.image}`);

    const versionMatch = checkImageVersion(customImageVersion, env.image);
    if (versionMatch === "ok") {
        bootLog.info("[boot] found matching custom image version");
        if (payload.edit) {
            // for edit sessions, allow custom image to be loaded by default
            // without prompts
            if (isUseCustomImageByDefault) {
                await continueBootWithCustomImageWithNoDialog(context, env);
                return;
            }
        }
        await continueBootWithDialog(context, env, {
            initialState: "UseCustomOrUseDefaultImage",
        });
        return;
    }
    if (versionMatch === "no-image") {
        bootLog.info("no custom image found, continuing with dialog");
        await continueBootWithDialog(context, env, {
            initialState: "OpenSetupOrUseDefaultImage",
            openSetupOrDefaultPromptType: "DirectLoadNoImage",
        });
        return;
    }
    bootLog.info("custom image version mismatch, continuing with dialog");
    await continueBootWithDialog(context, env, {
        initialState: "OpenSetupOrUseDefaultImage",
        openSetupOrDefaultPromptType: "DirectLoadVersionMismatch",
    });
    return;
};

const bootWithLocalScript = async (context: BootContext) => {
    bootLog.info("loading local script");
    // if a crash previously happened and the user set a recovery script, load it
    loadRecoveryScriptIfNeeded();
    const {
        savedScript,
        customImageVersion,
        isUseCustomImageByDefault,
        setUseCustomImageByDefault,
    } = usePersistStore.getState();
    // for boot purpose, we ignore errors during parsing the env
    const env = parseEnvFromScript(savedScript);

    // Does the script require CI?
    if (env.image) {
        bootLog.info(`local script requests custom image: ${env.image}`);
        // Check the version required by the script
        const versionMatch = checkImageVersion(customImageVersion, env.image);
        if (versionMatch === "ok") {
            bootLog.info("found matching custom image version");
            await continueBootWithCustomImageWithNoDialog(context, env);
        } else if (versionMatch === "no-image") {
            bootLog.info("no custom image found");
            await continueBootWithDialog(context, env, {
                initialState: "OpenSetupOrUseDefaultImage",
                openSetupOrDefaultPromptType: "LocalNoImage",
            });
        } else {
            bootLog.info("custom image version mismatch");
            await continueBootWithDialog(context, env, {
                initialState: "OpenSetupOrUseDefaultImage",
                openSetupOrDefaultPromptType: "LocalVersionMismatch",
            });
        }
        return;
    }

    // Script doesn't require CI, check if we have should use CI by default
    if (isUseCustomImageByDefault) {
        // check we have a valid stored image version
        if (VALID_VERSIONS.includes(customImageVersion)) {
            bootLog.info(
                `loading custom image by default, version=${customImageVersion}`,
            );
            await continueBootWithCustomImageWithNoDialog(context, env);
            return;
        }
        // if the CI version is invalid, clear use custom image by default
        setUseCustomImageByDefault(false);
    }

    await continueBootWithDefaultImage(context, env);
};

type CheckImageVersionResult = "no-image" | "mismatch" | "ok";
const checkImageVersion = (
    storedVersion: string,
    spec: ScriptEnvImage,
): CheckImageVersionResult => {
    if (!VALID_VERSIONS.includes(storedVersion)) {
        bootLog.debug(
            `stored_version=${storedVersion} is not a valid image version`,
        );
        return "no-image";
    }
    if (spec !== storedVersion) {
        bootLog.debug(`stored_version=${storedVersion}, spec=${spec}`);
        return "mismatch";
    }
    return "ok";
};

const continueBootWithDialog = async (
    context: BootContext,
    env: ScriptEnv,
    props: Omit<
        BootScreenProps,
        "runtime" | "onSuccess" | "scriptImageVersion" | "params"
    >,
) => {
    // if not mounted already, mount temporary root for showing dialog
    if (!context.unmountBootUI) {
        await context.beforeBootUI();

        // Block dialog from showing too early, which startles the user
        // i.e. let them see the loading fade-in first
        const msToWait = Math.ceil(1000 - performance.now());
        if (msToWait > 0) {
            bootLog.debug(
                `waiting for ${msToWait}ms before showing boot dialog`,
            );
            await new Promise((resolve) => setTimeout(resolve, msToWait));
        }
        if (isCrashed()) {
            return;
        }

        context.beforeBootUI = async () => {};
        const rootElement = document.getElementById(
            "-boot-root-",
        ) as HTMLDivElement;
        const root = createRoot(rootElement);
        context.unmountBootUI = () => {
            root.unmount();
        };
        root.render(
            <StrictMode>
                <CatchCrash>
                    <ThemeProvider>
                        <BootScreen
                            runtime={context.runtime}
                            scriptImageVersion={env.image}
                            params={env.params}
                            {...props}
                            onSuccess={() => {
                                void bootMainUI(context);
                            }}
                        />
                    </ThemeProvider>
                </CatchCrash>
            </StrictMode>,
        );
    }
};

const continueBootWithCustomImageWithNoDialog = async (
    context: BootContext,
    env: ScriptEnv,
) => {
    bootLog.info("booting with custom image without dialog");
    const result = await initRuntimeWithArgs(context, {
        isCustomImage: true,
        params: env.params,
        alwaysAskApp: false,
    });
    if (result.err) {
        bootLog.info(
            "failed to initialize runtime with custom image, showing dialog now",
        );
        bootLog.error(result.err);
        await continueBootWithDialog(context, env, {
            initialState: "OpenSetupOrUseDefaultImage",
            openSetupOrDefaultPromptType: "InitializeError",
            initialErrorString: result.err,
        });
        return;
    }
    await bootMainUI(context);
};

const continueBootWithDefaultImage = async (
    context: BootContext,
    env: ScriptEnv,
) => {
    bootLog.info("booting with default image without dialog");
    const result = await initRuntimeWithArgs(context, {
        isCustomImage: false,
        deleteCustomImage: false,
    });
    if (result.err) {
        bootLog.info(
            "failed to initialize runtime with default image, showing dialog now",
        );
        bootLog.error(result.err);
        await continueBootWithDialog(context, env, {
            initialState: "Error",
            initialErrorString: result.err,
        });
        return;
    }
    await bootMainUI(context);
};

const initRuntimeWithArgs = async (
    context: BootContext,
    args: RuntimeWorkerInitArgs,
): Promise<Void<(translator: Translator) => string>> => {
    return await initRuntime(await context.runtime, args);
};

const bootMainUI = async (context: BootContext) => {
    bootLog.info("booting main UI");
    await context.beforeBootUI(); // needed if boot UI is never shown
    await context.beforeMainUI();
    context.unmountBootUI?.();

    const queryClient = new QueryClient();
    initExtensionAppHost(await context.runtime);
    initExtensionManager();

    const runtime = await context.runtime;
    if (isCrashed()) {
        bootLog.warn("crash detected before main UI, not booting");
        return;
    }
    ReactRoot = createReactRoot();
    ReactRoot.render(
        <StrictMode>
            <CatchCrash>
                <RuntimeContext.Provider value={runtime}>
                    <QueryClientProvider client={queryClient}>
                        <ThemeProvider>
                            <ItemTooltipProvider
                                backgroundUrl={getSheikaBackgroundUrl()}
                            >
                                <App />
                            </ItemTooltipProvider>
                        </ThemeProvider>
                    </QueryClientProvider>
                </RuntimeContext.Provider>
            </CatchCrash>
            ,
        </StrictMode>,
    );

    void removeBootCurtain(true);

    // It's OK to set the title at the end,
    // because the title is also server-rendered.
    // This is just for switching between different languages
    addLocaleSubscriber(() => {
        document.title = translateUI("title");
    }, true);
};

const removeBootCurtain = async (animation: boolean) => {
    // Let UI settle before removing boot UI
    if (animation) {
        await new Promise((resolve) => setTimeout(resolve, 1));
    }
    // play fade out animation
    const curtain = document.getElementById("-root-curtain-");
    if (curtain) {
        if (animation) {
            curtain.classList.add("end");
            await new Promise((resolve) => setTimeout(resolve, 200));
        }
        curtain.remove();
    }
    document.querySelectorAll(".-boot-only-").forEach((el) => {
        el.remove();
    });
};

void boot();
