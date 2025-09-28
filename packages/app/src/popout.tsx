import { type PropsWithChildren, StrictMode, useCallback } from "react";
import { createRoot, type Root } from "react-dom/client";
import { addLocaleSubscriber, initDark } from "@pistonite/pure/pref";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "@pistonite/shared-controls";

import { initI18n, translateUI } from "skybook-localization";
import {
    getSheikaBackgroundUrl,
    ItemTooltipProvider,
    PopoutItemDragProvider,
    useItemDrag,
} from "@pistonite/skybook-itemsys";
import {
    readExtensionProperties,
    connectPopoutExtensionWindow,
} from "@pistonite/skybook-api/client";
import type { ExtensionApp } from "@pistonite/skybook-api";

import { type ConnectExtensionFn, getExtension } from "self::extensions";
import { extLog, probeAndRegisterAssetLocation, type FirstPartyExtension } from "self::util";

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

    const rootElement = document.getElementById("-popout-root-") as HTMLDivElement;
    const root = createRoot(rootElement);

    const connect: ConnectExtensionFn = async (_id: string, extension: FirstPartyExtension) => {
        extLog.info("connecting to host window");
        const app = await connectPopoutExtensionWindow(extension, properties);
        extLog.info("connected");
        if (!app) {
            return;
        }
        root.unmount();
        const newRoot = createRoot(rootElement);
        render(newRoot, app, extension);
    };

    const extension = await getExtension(properties.extensionId, true, connect);
    if (!extension) {
        extLog.error(`extension with ID '${properties.extensionId}' not found!`);
        return;
    }

    // render(root, undefined, <extension.Component />);

    addLocaleSubscriber(() => {
        const appTitle = translateUI("title");
        const extensionTitleKey = `extension.${properties.extensionId}.name`;
        const extensionTitle = translateUI(extensionTitleKey);
        document.title = `${extensionTitle} - ${appTitle}`;
    }, true);
}

const render = (root: Root, app: ExtensionApp, extension: FirstPartyExtension) => {
    const queryClient = new QueryClient();
    root.render(
        <StrictMode>
            <QueryClientProvider client={queryClient}>
                <ThemeProvider>
                    <PopoutWrapper app={app} extension={extension} />
                </ThemeProvider>
            </QueryClientProvider>
        </StrictMode>,
    );
};

type PopoutWrapperProps = {
    app: ExtensionApp;
    extension: FirstPartyExtension;
};

// eslint-disable-next-line react-refresh/only-export-components
const PopoutWrapper: React.FC<PopoutWrapperProps> = ({ app, extension }) => {
    const subscribe = useCallback(
        (fn: () => void) => {
            return extension.getItemDragData().subscribe(fn);
        },
        [extension],
    );
    return (
        <PopoutItemDragProvider
            app={app}
            subscribeData={subscribe}
            getData={() => extension.getItemDragData().get()}
        >
            <PopoutTooltipProvider>
                <div
                    style={{
                        width: "100vw",
                        height: "100vh",
                    }}
                >
                    <extension.Component />
                </div>
            </PopoutTooltipProvider>
        </PopoutItemDragProvider>
    );
};
// eslint-disable-next-line react-refresh/only-export-components
const PopoutTooltipProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const { data } = useItemDrag();
    return (
        <ItemTooltipProvider backgroundUrl={getSheikaBackgroundUrl()} suppress={!!data}>
            {children}
        </ItemTooltipProvider>
    );
};

void boot();
