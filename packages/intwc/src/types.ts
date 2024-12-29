import * as monaco from "monaco-editor-contrib";
import type { LanguageClient } from "./language/LanguageClient.ts";

/** Option to pass in to init */
export type InitOption = {
    /**
     * Preferences for the editor
     */
    preferences?: PreferenceOption;

    /**
     * Language support configurations
     */
    language?: LanguageOption;

    /**
     * Option for the editor
     */
    editor?: EditorOption;
}

export type PreferenceOption = {
    /** If the preference should be persisted to and loaded from localStorage */
    persist?: boolean;

    /**
     * Override the default preference
     *
     * These will not be applied to the persisted preference
     */
    defaults?: Partial<Preference>;
}

export type Preference = {
    /**
     * Input mode for the editor, defaults to "code"
     */
    inputMode: InputMode;
}

export type LanguageOption = {
    /** 
     * TypeScript Configuration
     *
     * If this is not specified, TypeScript features will not be enabled
     */
    typescript?: TSOption;

    /** If JSON language support should be enabled */
    json?: boolean;

    /** If CSS language support should be enabled */
    css?: boolean;

    /** If HTML language support should be enabled */
    html?: boolean;

    /** Custom language support */
    custom: LanguageClient[];
}

export type EditorOption = {
    /**
     * Options used when constructing the editor
     *
     * These are added on top of the defaults provided by this wrapper
     */
    options: monaco.editor.IEditorOptions & monaco.editor.IGlobalEditorOptions
}

export type TSOption = {
    /** 
     * If DOM API should be enabled for type checking
     *
     * Default is true
     */
    dom?: boolean;
    /** 
     * Extra libraries to load
     */
    extraLibs?: TSExtraLib[];
};

export type TSExtraLib = {
    /**
     * The library name. This is used to make the file uri.
     * For example, if the name is "foo", the file uri will
     * be "_lib_foo.ts"
     */
    name: string,
    /** The type definition file content */
    content: string
}


/** Input mode of the editor */
export type InputMode = "code" | "vim" | "emacs";
