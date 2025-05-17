import type { RuntimeWorkerInitError } from "@pistonite/skybook-api";

import { translateUI } from "../translate.ts";

export const translateRuntimeInitError = (error: RuntimeWorkerInitError): string => {
    const key = `runtime_init.${error.type}`;
    switch (error.type) {
        case "BadDlcVersion":
            return translateUI(key, { version: error.data });
        case "ProgramStartMismatch": {
            const [addr_ci, addr_script] = error.data;
            return translateUI(key, { addr_ci, addr_script });
        }
        default:
            return translateUI(key);
    }
};
