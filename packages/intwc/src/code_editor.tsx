import { useEffect, useState } from "react";

import { type CodeEditorApi, EditorState } from "./editor_state.ts";

export type CodeEditorProps = {
    /**
     * Callback when the editor is first created. You can return
     * a callback to be called when the editor is about to be destroyed.
     *
     * Use this to open initial file(s)
     */
    onCreated?: (api: CodeEditorApi) => (() => void) | undefined;
} & React.HTMLAttributes<HTMLDivElement>;

export const CodeEditor: React.FC<CodeEditorProps> = ({ onCreated, ...props }) => {
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
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [ref]);

    return <div ref={setRef} style={{ height: "100%" }} {...props} />;
};
