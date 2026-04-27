import * as monaco from "monaco-editor";
import { addDarkSubscriber } from "@pistonite/pure/pref";

import { log } from "../internal.ts";

import { DarkTheme, LightTheme, type Theme } from "./colors.ts";
import type { CustomTokenColor, ThemeOptions } from "./options.ts";

export const initThemes = (options: ThemeOptions) => {
    const customTokenColors = options.customTokenColors || [];
    defineTheme("intwc-dark", "vs-dark", DarkTheme, customTokenColors);
    defineTheme("intwc-light", "vs", LightTheme as unknown as Theme, customTokenColors);

    addDarkSubscriber((dark) => {
        monaco.editor.setTheme(dark ? "intwc-dark" : "intwc-light");
    }, true);
};

const defineTheme = (
    name: string,
    base: "vs" | "vs-dark",
    theme: Theme,
    customTokenColors: CustomTokenColor[],
) => {
    const dark = base === "vs-dark";
    const tokenNameToColor = new Map<string, string>();
    theme.tokenColors.forEach(({ token, foreground }) => {
        tokenNameToColor.set(token, foreground);
    });
    const rules = [...theme.tokenColors];
    customTokenColors.forEach(({ token, value }) => {
        if (typeof value === "string") {
            if (value.startsWith("#")) {
                tokenNameToColor.set(token, value);
                rules.push({ token, foreground: value });
            } else {
                const color = tokenNameToColor.get(value);
                if (!color) {
                    log.warn(`unknown token in custom color: ${value}`);
                    return;
                }
                rules.push({ token, foreground: color });
            }
            return;
        }
        const color = value[dark ? 1 : 0];
        tokenNameToColor.set(token, color);
        rules.push({ token, foreground: color });
    });
    monaco.editor.defineTheme(name, {
        base,
        inherit: true,
        colors: theme.editorColors,
        rules,
    });
};
