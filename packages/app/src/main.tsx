import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import { ThemeProvider } from './theme/ThemeProvider.tsx'
import { initDark } from '@pistonite/pure/pref'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { initI18n } from 'skybook-localization'
import { initExtensionManager } from './application/extensionManager.ts'
import { initRuntime } from 'runtime/init.ts'
import { ApplicationApi } from 'application/api.ts'
import { ApplicationProvider } from 'application/ApplicationProvider.tsx'

async function boot() {
    initDark({
        persist: false,
    });
    await initI18n();
    initExtensionManager();
    const queryClient = new QueryClient();

    const runtime = await initRuntime();

    const app = new ApplicationApi();

    createRoot(document.getElementById('-root-')!).render(
        <StrictMode>
            <ApplicationProvider app={app}>
            <QueryClientProvider client={queryClient}>
                <ThemeProvider>
                    <App />
                </ThemeProvider>
            </QueryClientProvider>
            </ApplicationProvider>
        </StrictMode>,
    )
}
void boot()
