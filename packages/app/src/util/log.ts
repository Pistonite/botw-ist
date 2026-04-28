import { logger } from "@pistonite/pure/log";

export const devLog = logger("dev", { color: "gray", level: "info"});
export const bootLog = logger("boot", { color: "#7D8509", level: "info" });
export const extLog = logger("extension", { color: "#85096C", level: "info"});
export const log = logger("app", { color: "#098543", level: "info" });
