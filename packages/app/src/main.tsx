import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import { ThemeProvider } from './theme/ThemeProvider.tsx'
import { initDark, isDark, prefersDarkMode } from '@pistonite/pure/pref'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { initI18n } from 'skybook-localization'
// import { initCodeEditor } from '@pistonite/intwc';
// import './index.css'
// import { initCodeEditorService } from '@pistonite/intwc'

// async function initEditor() {
//     // const TypeScriptWorker = (await import("monaco-editor/esm/vs/language/typescript/tsWorker.js?worker")).default;
//     await initCodeEditorService({
//         typescript: {
//             // createTypeScriptWorker: () => new TypeScriptWorker(),
//         }
//     });
// }

async function boot() {
    await initI18n();
    initDark({
        persist: false,
    });
    const queryClient = new QueryClient();
    // await initCodeEditor({
    //     language: {
    //         typescript: {
    //         } }
    // });
    createRoot(document.getElementById('-root-')!).render(
        <StrictMode>
            <QueryClientProvider client={queryClient}>
                <ThemeProvider>
                    <App />
                </ThemeProvider>
            </QueryClientProvider>
        </StrictMode>,
    )
}
void boot()
