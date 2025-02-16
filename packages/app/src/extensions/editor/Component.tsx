// import { useEffect } from "react";

import { CodeEditor } from "@pistonite/intwc";

import { useExtensionApp } from "application/useExtensionApp.ts";

import type { ExtensionComponentProps } from "../types.ts";

import { init } from "./init.ts";
import { EditorExtension } from "./editor.ts";

const FILE = "script.skyb";

export const Component: React.FC<ExtensionComponentProps> = ({
    standalone,
    connect,
}) => {
    const app = useExtensionApp();
    init(app);
    // useEffect(() => {
    // }, []);
    return (
        <CodeEditor
            style={{ height: "100%" }}
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
};
