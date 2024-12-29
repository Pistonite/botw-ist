import { useEffect, useState } from "react";

import { CodeEditorApi, EditorState } from "./EditorState.ts";

export type CodeEditorProps = {
    /** 
     * Callback when the editor is first created. You can return
     * a callback to be called when the editor is about to be destroyed.
     *
     * Use this to open initial file(s)
     */
    onCreated?: (api: CodeEditorApi) => (() => void) | void;
} & React.HTMLAttributes<HTMLDivElement>;

export const CodeEditor: React.FC<CodeEditorProps> = ({onCreated, ...props}) => {
    const [ref, setRef] = useState<HTMLDivElement | null>(null);

    useEffect(() => {
        if (!ref) {
            return;
        }
        const editor = new EditorState(ref);
        const cleanup = onCreated?.(editor);
        return () => {
            cleanup?.();
            editor.dispose();
        };
    }, [ ref]);

    return (<div ref={setRef} {...props}/>);
};
