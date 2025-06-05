/** Parser for the env tag in the script */

import type { Result } from "@pistonite/pure/result";
import type { CustomImageInitParams } from "./native";

/** Parse the leading env tag from the script */
export const parseEnvFromScript = (script: string): ScriptEnv => {
    const params: CustomImageInitParams = {
        dlc: 3,
        programStart: "",
        stackStart: "",
        stackSize: 0,
        heapFreeSize: 0,
        pmdmAddr: "",
    };
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
        return { params, errors: [] };
    }
    const envStart = lines[i].trim();
    if (envStart !== "'''env") {
        return { params, errors: [] };
    }
    i++; // skip the '''env line
    const lineStart = i; // 1-indexed
    const errors: ScriptEnvError[] = [];

    let image: ScriptEnvImage | undefined = undefined;

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
        const lineNumber = i + 1;
        const valueIndex = l.length - val.length;
        switch (key.trim().toLowerCase()) {
            case "image": {
                image = parseEnvImage(val.trim());
                break;
            }
            case "dlc": {
                params.dlc = parseEnvDlcVersion(val.trim());
                break;
            }
            case "program-start": {
                const res = parseRegionStart(val.trim());
                if (res.err) {
                    errors.push({
                        type: "AbsAddrError",
                        err: res.err,
                        line: lineNumber,
                        valueIndex,
                    });
                } else {
                    params.programStart = res.val;
                }
                break;
            }
            case "stack-start": {
                const res = parseRegionStart(val.trim());
                if (res.err) {
                    errors.push({
                        type: "AbsAddrError",
                        err: res.err,
                        line: lineNumber,
                        valueIndex,
                    });
                } else {
                    params.stackStart = res.val;
                }
                break;
            }
            case "stack-size": {
                const res = parseRegionSize(val.trim());
                if (res.err) {
                    errors.push({
                        type: "RegionSizeError",
                        err: res.err,
                        line: lineNumber,
                        valueIndex,
                    });
                } else {
                    params.stackSize = res.val;
                }
                break;
            }
            case "heap-free-size": {
                const res = parseRegionSize(val.trim());
                if (res.err) {
                    errors.push({
                        type: "RegionSizeError",
                        err: res.err,
                        line: lineNumber,
                        valueIndex,
                    });
                } else {
                    params.heapFreeSize = res.val;
                }
                break;
            }
            case "pmdm-addr": {
                const res = parseAbsAddrStringInternal(val.trim(), false);
                if (res.err) {
                    errors.push({
                        type: "AbsAddrError",
                        err: res.err,
                        line: lineNumber,
                        valueIndex,
                    });
                } else {
                    params.pmdmAddr = res.val;
                }
                break;
            }
            default: {
                errors.push({
                    type: "UnknownKey",
                    key: key.trim(),
                    line: lineNumber,
                    valueIndex: 0,
                });
                break;
            }
        }
    }
    return {
        params,
        image,
        lines: [lineStart, i + 1],
        errors,
    };
};

/**
 * Data to specify what environment the script should run in
 *
 * See https://skybook.pistonite.dev/user/custom_image
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
    errors: ScriptEnvError[];

    /**
     * Specify the specs for the BlueFlame image required to run the script
     *
     * Unspecified is the same as "default"
     */
    image?: ScriptEnvImage;

    /** Parameters parsed from the env block */
    params: CustomImageInitParams;
};

export type ScriptEnvImage = "1.5" | "1.6";

export const parseEnvImage = (image: string): ScriptEnvImage | undefined => {
    if (image.includes("1.5")) {
        return "1.5";
    }
    if (image.includes("1.6")) {
        return "1.6";
    }
    return undefined;
};

export const parseEnvDlcVersion = (dlc: string): number => {
    for (const c of [3, 2, 1, 0]) {
        if (dlc.includes(c.toString())) {
            return c;
        }
    }
    switch (dlc.trim()) {
        case "nodlc":
        case "none":
        case "uninstalled":
            return 0;
        case "master-trials":
        case "mt":
            return 2;
        default:
            return 3;
    }
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
export type ScriptEnvError = {
    /** The line of the error, 1-indexed */
    line: number;
    /** The index (char pos) of the value, 0-indexed */
    valueIndex: number;
} & (
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
      }
);
