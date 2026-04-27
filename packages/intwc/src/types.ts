import type { LanguageClient } from "./language/language_client.ts";
import type { ThemeOptions } from "./theme/options.ts";
import type { IEditorOptions, IGlobalEditorOptions } from "./monaco_types.ts";

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

    /**
     * Theme options
     */
    theme?: ThemeOptions;
};

export type PreferenceOption = {
    /** If the preference should be persisted to and loaded from localStorage */
    persist?: boolean;

    /**
     * Override the default preference
     *
     * These will not be applied to the persisted preference
     */
    defaults?: Partial<Preference>;
};

export type Preference = {
    /**
     * Input mode for the editor, defaults to "code"
     */
    inputMode: InputMode;
};

export type LanguageOption = {
    /**
     * TypeScript Configuration
     *
     * If this is not specified, TypeScript features will not be enabled.
     * You also need to use the `intwc` vite plugin to load the TypeScript worker.
     */
    typescript?: TSOption;

    /** Custom language support */
    custom?: LanguageClient[];
};

export type EditorOption = {
    /**
     * Options used when constructing the editor
     *
     * These are added on top of the defaults provided by this wrapper
     */
    options: IEditorOptions & IGlobalEditorOptions;
};

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
    name: string;
    /** The type definition file content */
    content: string;
};

/** Input mode of the editor */
export type InputMode = "code" | "vim" | "emacs";
