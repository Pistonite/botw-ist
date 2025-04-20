import { createContext, useContext } from "react";

import type { Runtime } from "@pistonite/skybook-api";

export const RuntimeContext = createContext<Runtime>(undefined as unknown as Runtime);

export const useRuntime = () => {
    return useContext(RuntimeContext);
};
