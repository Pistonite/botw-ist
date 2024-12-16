import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import { initCodeEditor } from '@pistonite/intwc';
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
    await initCodeEditor({
        language: {
            typescript: {}
        }
    });
    createRoot(document.getElementById('root')!).render(
        <StrictMode>
            <App />
        </StrictMode>,
    )
}
void boot()
