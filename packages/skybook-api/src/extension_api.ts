import type { WxPromise } from "@pistonite/workex";

import type { SessionMode } from "./types.ts";

/**
 * API implemented by the extension and called by the application.
 */
export interface Extension {
    /**
     * Notify the extension that the dark mode preference has changed.
     *
     * The extension can update the theme based on this event
     */
    onDarkModeChanged(dark: boolean): WxPromise<void>;

    /**
     * Notify the extension that the locale perference has changed.
     *
     * The locale string is one of the supported locales by the application,
     * such as `en-US`, `fr-FR`, etc.
     *
     * The extension can update the UI strings based on this event.
     */
    onLocaleChanged(locale: string): WxPromise<void>;

    /**
     * Notify the extension that the app mode has changed.
     *
     * See https://skybook.pistonite/dev/user/index.html#modes
     */
    onAppModeChanged(mode: SessionMode): WxPromise<void>;

    /**
     * Notify the extension that the icon rendering preference has changed
     */
    onIconSettingsChanged(
        enableHighQualityIcons: boolean,
        enableAnimations: boolean,
    ): WxPromise<void>;

    /**
     * Notify the extension that the script has changed.
     *
     * First party extensions that aren't visible in the app will not get this update.
     * Popouts will always get this update.
     */
    onScriptChanged(script: string, charPos: number): WxPromise<void>;
}
