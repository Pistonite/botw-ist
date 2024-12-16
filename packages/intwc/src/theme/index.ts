import * as monaco from 'monaco-editor';

import { DarkTheme, type Theme } from './themes.gen.ts';

export const initThemes = () => {
    defineTheme("intwc-dark", "vs-dark", DarkTheme);

    monaco.editor.setTheme("intwc-dark");
}

const defineTheme = (name: string, base: "vs" | "vs-dark", theme: Theme) => {
    monaco.editor.defineTheme(name, {
        base,
        inherit: true,
        colors: theme.editorColors,
        rules: theme.tokenColors,
    });
}

