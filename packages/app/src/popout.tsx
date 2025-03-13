import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { initDark } from "@pistonite/pure/pref";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "@pistonite/shared-controls";

import { initI18n } from "skybook-localization";
import { ItemTooltipProvider } from "skybook-item-system";
import {
    readExtensionProperties,
    connectPopoutExtensionWindow,
} from "@pistonite/skybook-api/client";

import {
    type ConnectExtensionFn,
    getExtension,
    type FirstPartyExtension,
} from "self::extensions";

import {
    getSheikaBackgroundUrl,
    probeAndRegisterAssetLocation,
} from "./ui/asset.ts";

async function boot() {
    // Initialize preferences, but do not persist settings
    // popout also does not connect to the store at all

    initDark({ persist: false });
    await initI18n();
    await probeAndRegisterAssetLocation();

    const properties = readExtensionProperties();
    if (!properties.extensionId) {
        // should not happen, just error and bail
        console.error("[popout] No extension ID provided!");
        return;
    }

    const connect: ConnectExtensionFn = async (
        extension: FirstPartyExtension,
    ) => {
        await connectPopoutExtensionWindow(extension, properties);
        return () => {};
    };

    const extension = await getExtension(properties.extensionId, true, connect);
    if (!extension) {
        console.error(
            `[popout] Extension with ID ${properties.extensionId} not found!`,
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
                        <extension.Component />
                    </ItemTooltipProvider>
                </ThemeProvider>
            </QueryClientProvider>
        </StrictMode>,
    );
}

void boot();
