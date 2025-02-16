import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./App.tsx";
import { ThemeProvider } from "./theme/ThemeProvider.tsx";
import { initDark } from "@pistonite/pure/pref";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

import { initI18n } from "skybook-localization";
import { ItemTooltipProvider } from "skybook-item-system";

import { initExtensionManager } from "./application/extensionManager.ts";
import { createExtensionAppHost } from "application/ExtensionAppHost.ts";
import { initNarrow } from "pure-contrib/narrow.ts";
import { isLessProductive } from "pure-contrib/platform.ts";

import { initRuntime } from "./runtime.ts";
import { ExtensionAppContext } from "application/useExtensionApp.ts";

async function boot() {
    const root = document.getElementById("-root-") as HTMLDivElement;
    if (isLessProductive) {
        // window.setStatus
        // await new Promise<void>((resolve) => {
        //     const button = document.createElement('button');
        //     button.innerText = 'fullscreen' + window.innerWidth;
        //     button.onclick = async () => {
        //     // document.body.style.height = 'calc ( 100vh + 1px )';
        //     // document.body.style.overflow = 'visible';
        //     // root.style.height = 'calc ( 100vh + 1px )';
        //     // window.scrollTo(0, 100);
        //         await document.body.requestFullscreen({
        //             navigationUI: "hide"});
        //         resolve();
        //     };
        //     root.appendChild(button);
        // });
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
    initDark({
        persist: false,
    });
    await initI18n();
    initExtensionManager();
    const queryClient = new QueryClient();

    const runtime = await initRuntime();
    const app = createExtensionAppHost(runtime);

    createRoot(root).render(
        <StrictMode>
            <ExtensionAppContext.Provider value={app}>
                <QueryClientProvider client={queryClient}>
                    <ThemeProvider>
                        <ItemTooltipProvider>
                            <App />
                        </ItemTooltipProvider>
                    </ThemeProvider>
                </QueryClientProvider>
            </ExtensionAppContext.Provider>
        </StrictMode>,
    );
}
void boot();
