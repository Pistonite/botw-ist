/** Parser for the env tag in the script */

import type { Result } from "@pistonite/pure/result";

/** Parse the leading env tag from the script */
export const parseEnvFromScript = (script: string): ScriptEnv => {
    const lines = script.split("\n");
    let i = 0;
    for (; i < lines.length; i++) {
        const l = lines[i].trim();
        if (l.startsWith("//") || l.startsWith("#")) {
            continue;
        }
        if (!l) {
            continue;
        }
        break;
    }
    if (i >= lines.length) {
        return {};
    }
    const envStart = lines[i].trim();
    if (envStart !== "'''env") {
        return {};
    }
    i++;
    const lineStart = i;
    const env: ScriptEnv = {};
    const errors: ScriptEnvError[] = [];
    for (; i < lines.length; i++) {
        const l = lines[i].trim();
        if (!l) {
            continue;
        }
        if (l.startsWith("'''")) {
            break;
        }
        const parts = l.split("=", 2);
        if (parts.length < 2) {
            continue;
        }
        const [key, val] = parts;
        switch (key.trim().toLowerCase()) {
            case "image": {
                env.image = parseEnvImage(val.trim());
                break;
            }
            case "program-start": {
                const res = parseRegionStart(val.trim());
                if (res.err) {
                    errors.push({ type: "AbsAddrError", err: res.err });
                } else {
                    env.programStart = res.val;
                }
                break;
            }
            case "stack-start": {
                const res = parseRegionStart(val.trim());
                if (res.err) {
                    errors.push({ type: "AbsAddrError", err: res.err });
                } else {
                    env.stackStart = res.val;
                }
                break;
            }
            case "stack-size": {
                const res = parseRegionSize(val.trim());
                if (res.err) {
                    errors.push({ type: "RegionSizeError", err: res.err });
                } else {
                    env.stackSize = res.val;
                }
                break;
            }
            case "heap-free-size": {
                const res = parseRegionSize(val.trim());
                if (res.err) {
                    errors.push({ type: "RegionSizeError", err: res.err });
                } else {
                    env.heapFreeSize = res.val;
                }
                break;
            }
            case "pmdm-addr": {
                const res = parseAbsAddrStringInternal(val.trim(), false);
                if (res.err) {
                    errors.push({ type: "AbsAddrError", err: res.err });
                } else {
                    env.pmdmAddr = res.val;
                }
                break;
            }
            default: {
                errors.push({ type: "UnknownKey", key: key.trim() });
                break;
            }
        }
    }
    env.lines = [lineStart, i + 1];
    if (errors.length > 0) {
        env.errors = errors;
    }
    return env;
};

/**
 * Data to specify what environment the script should run in
 */
export type ScriptEnv = {
    /**
     * If an env block is defined, the line number range of the block
     *
     * The numbers are 1-indexed (first line is line 1), and inclusive
     */
    lines?: [number, number];

    /**
     * Errors encountered while parsing the env block
     */
    errors?: ScriptEnvError[];
    /**
     * Specify the specs for the BlueFlame image required to run the script
     *
     * Unspecified is the same as "default"
     */
    image?: ScriptEnvImage;
    /**
     * Specify the physical address of the program start
     *
     * The string should look like 0x000000XXXXX00000, where X is a hex digit
     *
     * Unspecified means the script can run with any program start address
     */
    programStart?: string;

    /**
     * Specify the physical address of the stack start
     *
     * The string should look like 0x000000XXXXX00000, where X is a hex digit
     *
     * Unspecified means the script can run with any stack start address
     */
    stackStart?: string;

    /**
     * Size of the stack in bytes
     *
     * Unspecified, or 0, means the script can run with any stack size
     */
    stackSize?: number;

    /**
     * Size of the free region of the heap in bytes
     *
     * Unspecified, or 0, means the script can run with any heap size
     */
    heapFreeSize?: number;

    /**
     * Physical address of the PauseMenuDataMgr (i.e. This value is PauseMenuDataMgr*)
     * This is used to determine the address of the other singletons, as well as allocating
     * the appropriate address space for the heap.
     *
     * Unspecified, means the script can run with any PauseMenuDataMgr address
     */
    pmdmAddr?: string;
};

export type ScriptEnvImage =
    | "default"
    | `custom-${ScriptEnvImageVersionSpec}${ScriptEnvImageDlcSpec}`;

export type ScriptEnvImageVersionSpec = "anyver" | "ver1.5" | "ver1.6";

export type ScriptEnvImageDlcSpec =
    | ""
    | "-nodlc"
    | "-dlc-1"
    | "-dlc-2"
    | "-dlc-3"
    | "-dlc-1-or-2"
    | "-dlc-1-or-3"
    | "-dlc-2-or-3"
    | "-dlc";

export const parseEnvImage = (image: string): ScriptEnvImage => {
    if (!image.includes("custom")) {
        return "default";
    }
    image = image.replace(/custom-/g, "");
    let version: ScriptEnvImageVersionSpec = "anyver";
    if (image.includes("1.5")) {
        version = "ver1.5";
        image = image.replace(/1\.5/g, "");
    } else if (image.includes("1.6")) {
        version = "ver1.6";
        image = image.replace(/1\.6/g, "");
    }
    if (!image.includes("dlc")) {
        return `custom-${version}`;
    }
    if (image.includes("nodlc") || image.includes("no-dlc")) {
        return `custom-${version}-nodlc`;
    }
    const allowOne = image.includes("1");
    const allowTwo = image.includes("2");
    const allowThree = image.includes("3");
    if (allowOne) {
        if (allowTwo) {
            if (allowThree) {
                return `custom-${version}-dlc`;
            }
            return `custom-${version}-dlc-1-or-2`;
        }
        if (allowThree) {
            return `custom-${version}-dlc-1-or-3`;
        }
        return `custom-${version}-dlc-1`;
    }
    if (allowTwo) {
        if (allowThree) {
            return `custom-${version}-dlc-2-or-3`;
        }
        return `custom-${version}-dlc-2`;
    }
    if (allowThree) {
        return `custom-${version}-dlc-3`;
    }
    return `custom-${version}-dlc`;
};
/** Input is hex string with optional 0x prefix */
export const parseRegionStart = (
    addr: string,
): Result<string, AbsAddrError> => {
    return parseAbsAddrStringInternal(addr, true);
};

/** Input is hex string with optional 0x prefix */
const parseAbsAddrStringInternal = (
    addr: string,
    forceRegionAlign: boolean,
): Result<string, AbsAddrError> => {
    if (!addr) {
        return { val: "0x0000000000000000" };
    }
    if (addr.startsWith("0x")) {
        addr = addr.substring(2);
    }
    while (addr.length < 5) {
        addr = "0" + addr;
    }
    if (forceRegionAlign) {
        if (!addr.endsWith("00000")) {
            return { err: "suffix" };
        }
    } else {
        // 8 bytes aligned
        if (!addr.endsWith("0") && !addr.endsWith("8")) {
            return { err: "suffix" };
        }
    }
    if (!addr.match(/^[0-9a-fA-F]+$/)) {
        return { err: "hex" };
    }
    addr = addr.substring(0, addr.length - 5);
    while (addr.length < 5) {
        addr = "0" + addr;
    }
    const importantBits = addr.substring(addr.length - 5);
    let prefix = addr.substring(0, addr.length - 5);
    if (prefix.length == 0) {
        prefix = "0";
    }
    if (!prefix.match(/^0+$/)) {
        return { err: "prefix" };
    }
    return { val: `0x000000${importantBits.toLowerCase()}00000` };
};

/** Input is hex string with optional 0x prefix */
export const parseRegionSize = (
    num: string,
): Result<number, RegionSizeError> => {
    if (!num) {
        return { val: 0 };
    }
    if (num.startsWith("0x")) {
        num = num.substring(2);
    }
    if (!num.match(/^[0-9a-fA-F]+$/)) {
        return { err: "hex" };
    }
    const size = parseInt(num, 16);
    if (size >= 0xfffff000) {
        return { err: "overflow" };
    }
    if (Number.isNaN(size) || size < 0) {
        return { err: "hex" };
    }
    if (size % 0x1000 !== 0) {
        return { err: "align" };
    }
    return { val: size };
};

export type AbsAddrError = "suffix" | "prefix" | "hex";
export type RegionSizeError = "hex" | "align" | "overflow";
export type ScriptEnvError =
    | {
          type: "AbsAddrError";
          err: AbsAddrError;
      }
    | {
          type: "RegionSizeError";
          err: RegionSizeError;
      }
    | {
          type: "UnknownKey";
          key: string;
      };
