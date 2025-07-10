import type { Extension } from "../extension_api.ts";
import type { ExtensionApp } from "../extension_app.ts";

/**
 * Extension extended with local APIs for setting up the extension
 */
export type ExtensionModule = Extension & {
    /**
     * Called when the connection between the application and the extension is established.
     *
     * This means the extension can start calling the application APIs via
     * the provided `app` object.
     */
    onAppConnectionEstablished(app: ExtensionApp): void;
};
