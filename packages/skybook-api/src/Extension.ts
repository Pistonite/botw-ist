import type { WorkexPromise } from "@pistonite/workex";

/**
 * API implemented by the extension and called by the application.
 *
 * @workex:send app
 * @workex:recv extension
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
