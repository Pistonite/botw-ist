// To enable maximum tree-shaking, we don't depend on catppuccin at runtime. Instead,
// we generate the themes at build time and only reference the colors we need
//
// Output is written to stdout, so any engine that supports TypeScript can run this script
import { ColorFormat, flavors } from "@catppuccin/palette";

function opacity(color: ColorFormat, alpha: number) {
    const alphaInt = Math.max(Math.min(Math.floor(alpha * 255), 255), 0);
    const alphaHex = alphaInt.toString(16).padStart(2, "0");
    return color.hex + alphaHex;
}
function createTokenStyle(tokens: string[], style: Record<string, unknown>) {
    return tokens.map(token => ({
        token,
        ...style
    }));
}

function createCommentTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle(["comment"], style);
}

function createPunctuationTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "punctuation",
        "delimiter",
        "meta.brace",
    ], style);
}

function createKeywordTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "keyword",
    ], style);
}

function createOperatorTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "keyword.operator",
        "operator",
    ], style);
}

function createVariableTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "variable",
        "variable.parameter",
    ], style);
}

function createVariableLibraryTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "variable.language",
    ], style);
}

function createVariableConstTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "variable.readonly",
    ], style);
}

function createFunctionTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "support.function",
        "function"
    ], style);
}

function createMacroTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "meta.macro",
    ], style);
}

function createTypeTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "class",
        "type",
        "namespace",
        "support.type",
    ], style);
}

function createLiteralConstantTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "constant",
        "number",
    ], style);
}

function createLiteralStringTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "string",
    ], style);
}

function createLiteralRegExpTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "string.regexp",
        "regexp",
    ], style);
}

function createSourceTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "source",
    ], style);
}

function createEscapeTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "constant.character.escape",
        "string.escape",
    ], style);
}

function createTagTokenStyle(style: Record<string, unknown>) {
    return createTokenStyle([
        "tag",
    ], style);
}



function createDarkTheme() {
    const mocha = flavors.mocha.colors;
    const frappe = flavors.frappe.colors;
    return {
        editorColors: {
            "foreground": mocha.text.hex,
            "descriptionForeground": mocha.text.hex,
            "errorForeground": mocha.red.hex,
            "selection.background": opacity(mocha.text, 0.1),
            "focusBorder": mocha.lavender.hex,

            "editor.background": mocha.base.hex,
            "editor.foreground": mocha.text.hex,
            "editorWidget.background": frappe.base.hex,
            "editorWidget.border": frappe.mantle.hex,
            "editorCursor.foreground": mocha.rosewater.hex,
            "editorIndentGuide.background": mocha.surface0.hex,
            "editorBracketMatch.background": opacity(mocha.text, 0.1),
            "editorBracketMatch.border": mocha.peach.hex,
            "editorError.foreground": mocha.red.hex,
            "editorWarning.foreground": mocha.yellow.hex,
            "editorMarkerNagivationError.background": mocha.red.hex,
            "editorLineNumber.foreground": mocha.overlay2.hex,
            "editorLineNumber.activeForeground": mocha.overlay2.hex,
            "editor.lineHighlightBackground": opacity(mocha.text, 0.1),

            "input.background": mocha.mantle.hex,
            "input.foreground": mocha.text.hex,
            "input.placeholderForeground": mocha.surface0.hex,
        },
        tokenColors: [
            ...createCommentTokenStyle({
                foreground: mocha.overlay2.hex,
                fontStyle: "italic"
            }),
            ...createPunctuationTokenStyle({
                foreground: mocha.overlay2.hex
            }),
            ...createKeywordTokenStyle({
                foreground: mocha.mauve.hex
            }),
            ...createOperatorTokenStyle({
                foreground: mocha.sapphire.hex
            }),
            ...createVariableTokenStyle({
                foreground: mocha.lavender.hex
            }),
            ...createVariableLibraryTokenStyle({
                foreground: mocha.red.hex
            }),
            ...createVariableConstTokenStyle({
                foreground: mocha.peach.hex
            }),
            ...createFunctionTokenStyle({
                foreground: mocha.yellow.hex
            }),
            ...createMacroTokenStyle({
                foreground: mocha.peach.hex
            }),
            ...createTypeTokenStyle({
                foreground: mocha.blue.hex
            }),
            ...createLiteralConstantTokenStyle({
                foreground: mocha.peach.hex
            }),
            ...createLiteralStringTokenStyle({
                foreground: mocha.green.hex
            }),
            ...createLiteralRegExpTokenStyle({
                foreground: mocha.red.hex
            }),
            ...createSourceTokenStyle({
                foreground: mocha.text.hex
            }),
            ...createEscapeTokenStyle({
                foreground: mocha.pink.hex
            }),
            ...createTagTokenStyle({
                foreground: mocha.pink.hex
            }),
        ]
    } as const;
}

type Theme = ReturnType<typeof createDarkTheme>;

function emitTheme(ident: string, theme: Theme) {
    console.log(`export const ${ident} = ` + JSON.stringify(theme, null, 4) + ";");
}



emitTheme("DarkTheme", createDarkTheme());

console.log("export type Theme = typeof DarkTheme;");
