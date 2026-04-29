import { logger, type Logger } from "@pistonite/pure/log";

export const devLog: Logger = logger("dev", { color: "gray", level: "info" });
export const bootLog: Logger = logger("boot", { color: "#7D8509", level: "info" });
export const extLog: Logger = logger("extension", { color: "#85096C", level: "info" });
export const log: Logger = logger("app", { color: "#098543", level: "info" });
