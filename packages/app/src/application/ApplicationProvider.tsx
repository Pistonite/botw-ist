import type { PropsWithChildren } from "react";
import type { Application } from "@pistonite/skybook-extension-api";

import { ApplicationContext } from "./useApplication.ts";

export const ApplicationProvider: React.FC<
    PropsWithChildren<{ app: Application }>
> = ({ app, children }) => {
    return (
        <ApplicationContext.Provider value={app}>
            {children}
        </ApplicationContext.Provider>
    );
};
