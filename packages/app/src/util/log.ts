import { logger } from "@pistonite/pure/log";

export const devLog = logger("dev", "gray").info();
export const bootLog = logger("boot", "#7D8509").info();
export const extLog = logger("extension", "#85096C").info();
export const log = logger("app", "#098543").info();
