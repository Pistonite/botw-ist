import { createContext, useContext } from "react";
import type { ExtensionApp } from "@pistonite/skybook-api";

export const ExtensionAppContext = createContext<ExtensionApp>(
    // must be provided
    {} as unknown as ExtensionApp,
);

export const useExtensionApp = () => {
    return useContext(ExtensionAppContext);
};
