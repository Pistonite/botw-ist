import type { RuntimeWorkerInitError } from "@pistonite/skybook-api";

import { type Translator, translateUI } from "../translate.ts";

export const translateRuntimeInitError = (
    error: RuntimeWorkerInitError,
    translator: Translator  = translateUI,
): string => {
    const key = `runtime_init.${error.type}`;
    switch (error.type) {
        case "BadDlcVersion":
            return translator(key, { version: error.data });
        case "ProgramStartMismatch": {
            const [addr_ci, addr_script] = error.data;
            return translator(key, { addr_ci, addr_script });
        }
        default:
            return translator(key);
    }
};
