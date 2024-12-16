import { Suspense, useEffect, useState } from "react";
// import { serviceReady } from ".";
import { EditorState } from "./EditorState.ts";

export type CodeEditorProps = {
    /** 
     * Callback when the editor is first created
     *
     * Use this to open initial file(s)
     */
    onCreated?: (api: EditorState) => void | Promise<void>;
};

export const CodeEditor: React.FC<CodeEditorProps> = ({onCreated}) => {
    // const [ready, setReady] = useState(false);
    const [ref, setRef] = useState<HTMLDivElement | null>(null);

    useEffect(() => {
        // serviceReady().then(() => {
        //     setReady(true);
        // }).catch((e: unknown) => {
        //     console.error(e);
        // });
    }, [ ]);

    useEffect(() => {
        if (!ref) {
            return;
        }
        // let isActive = true;
        const editor = new EditorState(ref);
        // async function start() {
        //     await editor.start();
        //     if (!isActive) {
        //         return;
        //     }
            onCreated?.(editor);
        // }
        // start();
        return () => {
            // isActive = false;
            editor.dispose();
        };
    }, [ ref]);

    // if (!ready) {
    //     return (<div className="intwc-container-pending"/>);
    // }


    // classname for debugging purposes
    return (<div ref={setRef} style={{width: "100%", height: "100%"}} className="intwc-container"/>);
};
