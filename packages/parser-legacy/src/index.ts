import { parseCommand } from "./command";
import type { ItemStack } from "./command/item.ts";

/**
 * Convert legacy script to Skybook script
 */
export const convertLegacyScript = (script: string): string => {
    const lines = script.split("\n");
    const commands = lines.map((line) => parseCommand(line, stubSearch));
    return commands.map((cmd) => cmd.convert()).join("\n");
};

const stubSearch = (item: string): ItemStack => {
    return {
        ident: item,
    };
};
