// import { useEffect } from "react";

import type { ExtensionComponentProps } from "../types.ts"
import { initLanguage } from "./Language.ts"
import { CodeEditor } from "@pistonite/intwc";
import { EditorExtension } from "./editor.ts";
import { useApplication } from "application/useApplication.ts";


const FILE = "script.skyb";


export const Component: React.FC<ExtensionComponentProps> = ({
    standalone, connect
}) => {
    const app = useApplication();
    initLanguage();
    // useEffect(() => {
    // }, []);
    return (
    <CodeEditor 
            style={{ height: "100%"  }}
            onCreated={(editor) => {
                const unsubscribeEditor = editor.subscribe((filename) => {
                    if (filename !== FILE) {
                        return;
                    }
                    void app.setScript(editor.getFileContent(FILE));
                });
                const instance = new EditorExtension(FILE, standalone, editor);
                const disconnect = connect(instance);
                console.log("EditorComponent: onCreated");
                return () => {
                    unsubscribeEditor();
                    disconnect();
                };
            }}
        />
    );
}
