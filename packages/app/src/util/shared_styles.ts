import { gale } from "@pistonite/shared-controls";

export const useStyleEngine = gale({
    "gap-1": {
        gap: "1px",
    },
    "gap-2": {
        gap: "2px",
    },
    "gap-4": {
        gap: "4px",
    },
    "gap-8": {
        gap: "8px",
    },
    "gap-16": {
        gap: "16px",
    },
    "pad-4": {
        padding: "4px",
    },
    "pad-8": {
        padding: "8px",
    },
    "pad-itemtop": {
        // this is to ensure the oversized icons aren't getting
        // truncated when being in the first row
        paddingTop: "10px",
    },
});
