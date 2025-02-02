import type { Result } from "@pistonite/pure/result";
import type { WorkexPromise } from "@pistonite/workex";

/**
 * API implemented by the extension and called by the application.
 *
 * @workex:send app
 * @workex:recv ext
 */
export interface Extension {
    /**
     * Notify the extension that the dark mode preference has changed.
     *
     * The extension can update the theme based on this event
     */
    onDarkModeChanged(dark: boolean): WorkexPromise<void>;

    /**
     * Notify the extension that the locale perference has changed.
     *
     * The locale string is one of the supported locales by the application,
     * such as `en-US`, `fr-FR`, etc.
     *
     * The extension can update the UI strings based on this event.
     */
    onLocaleChanged(locale: string): WorkexPromise<void>;

    /**
     * Notify the extension that the script has changed.
     */
    onScriptChanged(script: string): WorkexPromise<void>;
}

/**
 * API implemented by the application and called by the extension.
 *
 * @workex:send ext
 * @workex:recv app
 */
export interface Application {
    /** Get the current simulator script. */
    getScript(): WorkexPromise<string>;

    /** Set the simulator script. */
    setScript(script: string): WorkexPromise<void>;

    /**
     * Resolve an item from a query
     *
     * If localized is true, treat the query as a localized item search query (i.e. "[tag:]words"),
     * otherwise, treat it as an identifier search query.
     *
     * Cook effect is the game's representation, or 0 for no effect
     */
    resolveItem(
        query: string,
        localized: boolean,
        limit: number,
    ): WorkexPromise<Result<{ actor: string; cookEffect: number }[], string>>;

    // /** Get the semantic tokens for the current script */
    // provideSemanticTokens(start: number, end: number): WorkexPromise<Uint32Array>;
}
