import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { addLocaleSubscriber, initDark } from "@pistonite/pure/pref";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "@pistonite/shared-controls";

import { initI18n, translateUI } from "skybook-localization";
import { ItemTooltipProvider } from "skybook-item-system";
import {
    readExtensionProperties,
    connectPopoutExtensionWindow,
} from "@pistonite/skybook-api/client";

import { type ConnectExtensionFn, getExtension } from "self::extensions";
import {
    extLog,
    getSheikaBackgroundUrl,
    probeAndRegisterAssetLocation,
    type FirstPartyExtension,
} from "self::util";

async function boot() {
    // Initialize preferences, but do not persist settings
    // popout also does not connect to the store at all

    initDark({ persist: false });
    await initI18n(false);
    await probeAndRegisterAssetLocation();

    const properties = readExtensionProperties();
    if (!properties.extensionId) {
        // should not happen, just error and bail
        extLog.error("no extension ID!");
        return;
    }

    const connect: ConnectExtensionFn = async (
        _id: string,
        extension: FirstPartyExtension,
    ) => {
        extLog.error("connecting to host window");
        await connectPopoutExtensionWindow(extension, properties);
        extLog.info("connected");
    };

    const extension = await getExtension(properties.extensionId, true, connect);
    if (!extension) {
        extLog.error(
            `extension with ID '${properties.extensionId}' not found!`,
        );
        return;
    }

    const queryClient = new QueryClient();

    const rootElement = document.getElementById(
        "-popout-root-",
    ) as HTMLDivElement;
    const root = createRoot(rootElement);
    root.render(
        <StrictMode>
            <QueryClientProvider client={queryClient}>
                <ThemeProvider>
                    <ItemTooltipProvider
                        backgroundUrl={getSheikaBackgroundUrl()}
                    >
                        <div
                            style={{
                                width: "100vw",
                                height: "100vh",
                            }}
                        >
                            <extension.Component />
                        </div>
                    </ItemTooltipProvider>
                </ThemeProvider>
            </QueryClientProvider>
        </StrictMode>,
    );
    addLocaleSubscriber(() => {
        const appTitle = translateUI("title");
        const extensionTitleKey = `extension.${properties.extensionId}.name`;
        const extensionTitle = translateUI(extensionTitleKey);
        document.title = `${extensionTitle} - ${appTitle}`;
    }, true);
}

void boot();
