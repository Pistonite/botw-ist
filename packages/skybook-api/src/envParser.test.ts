import { describe, expect, it } from "vitest";

import {
    parseRegionStart,
    parseEnvFromScript,
    parseEnvImage,
    parseRegionSize,
    parseEnvDlcVersion,
} from "./envParser.ts";

describe("parseEnvFromScript", () => {
    it.each([
        "",
        "\n",
        "// comment\n",
        "// comment\nline\n'''env\nafter ignored",
    ])("parses empty", (input) => {
        expect(parseEnvFromScript(input)).toEqual({
            params: {
                dlc: 3,
                programStart: "",
                stackStart: "",
                stackSize: 0,
                heapFreeSize: 0,
                pmdmAddr: "",
            },
            errors: [],
        });
    });
    it("parses valid input", () => {
        expect(
            parseEnvFromScript(`
'''env
image = custom-ver1.5
program-start = 34500000
stack-start = 34500000
stack-size = 0x345000
heap-free-size = 0x345000
pmdm-addr = 0x34500000
'''
`),
        ).toEqual({
            lines: [2, 9],
            image: "1.5",
            params: {
                dlc: 3,
                programStart: "0x0000000034500000",
                stackStart: "0x0000000034500000",
                stackSize: 0x345000,
                heapFreeSize: 0x345000,
                pmdmAddr: "0x0000000034500000",
            },
            errors: [],
        });
    });
});

describe("parseEnvImage", () => {
    it("parses default", () => {
        expect(parseEnvImage("")).toBe(undefined);
        expect(parseEnvImage("def")).toBe(undefined);
        expect(parseEnvImage("default")).toBe(undefined);
    });
    it.each(["1.5", "1.6"])("parses specific version", (version) => {
        expect(parseEnvImage(version)).toBe(version);
        expect(parseEnvImage("ver" + version)).toBe(version);
        expect(parseEnvImage("v" + version)).toBe(version);
        expect(parseEnvImage("custom-v" + version)).toBe(version);
        expect(parseEnvImage("custom-ver" + version)).toBe(version);
    });
});

describe("parseEnvDlcVersion", () => {
    it("parses default", () => {
        expect(parseEnvDlcVersion("")).toBe(3);
        expect(parseEnvDlcVersion("def")).toBe(3);
        expect(parseEnvDlcVersion("default")).toBe(3);
    });
    it.each([0, 1, 2, 3])("parses from number", (version) => {
        expect(parseEnvDlcVersion(version.toString())).toBe(version);
        expect(parseEnvDlcVersion("dlc-" + version.toString())).toBe(version);
        expect(parseEnvDlcVersion("dlc-" + version.toString() + ".0")).toBe(
            version,
        );
        expect(parseEnvDlcVersion("ver-" + version.toString() + ".0")).toBe(
            version,
        );
        expect(parseEnvDlcVersion("ver-" + version.toString())).toBe(version);
    });
    it("parses shorthands", () => {
        expect(parseEnvDlcVersion("nodlc")).toBe(0);
        expect(parseEnvDlcVersion("none")).toBe(0);
        expect(parseEnvDlcVersion("uninstalled")).toBe(0);

        expect(parseEnvDlcVersion("day-1")).toBe(1);
        expect(parseEnvDlcVersion("master-trials")).toBe(2);
        expect(parseEnvDlcVersion("mt")).toBe(2);
        expect(parseEnvDlcVersion("cb")).toBe(3);
    });
});

describe("parseAbsAddrString", () => {
    it.each(["", "0", "0x0", "0x00"])("parses empty", (input) => {
        expect(parseRegionStart(input)).toEqual({
            val: "0x0000000000000000",
        });
    });
    it.each(["34500", "0x34500", "3400", "0x3400", "340000", "0x340000"])(
        "detects invalid suffix",
        (input) => {
            expect(parseRegionStart(input)).toEqual({
                err: "suffix",
            });
        },
    );
    it.each`
        input               | output
        ${"34500000"}       | ${"0x0000000034500000"}
        ${"0x34500000"}     | ${"0x0000000034500000"}
        ${"34000000"}       | ${"0x0000000034000000"}
        ${"0x34000000"}     | ${"0x0000000034000000"}
        ${"3a0000000"}      | ${"0x00000003a0000000"}
        ${"0x3A0000000"}    | ${"0x00000003a0000000"}
        ${"B3a0000000"}     | ${"0x000000b3a0000000"}
        ${"0xb3A0000000"}   | ${"0x000000b3a0000000"}
        ${"B3aff00000"}     | ${"0x000000b3aff00000"}
        ${"0xb3Aff00000"}   | ${"0x000000b3aff00000"}
        ${"0B3aff00000"}    | ${"0x000000b3aff00000"}
        ${"0x0b3Aff00000"}  | ${"0x000000b3aff00000"}
        ${"00B3aff00000"}   | ${"0x000000b3aff00000"}
        ${"0x00b3Aff00000"} | ${"0x000000b3aff00000"}
    `("parses with valid suffix", ({ input, output }) => {
        expect(parseRegionStart(input)).toEqual({
            val: output,
        });
    });

    it.each(["34534500000", "34534000000", "0x34534000000"])(
        "detects invalid prefix",
        (input) => {
            expect(parseRegionStart(input)).toEqual({
                err: "prefix",
            });
        },
    );

    it.each(["foo00000", "0xfoo00000"])("detects invalid hex", (input) => {
        expect(parseRegionStart(input)).toEqual({
            err: "hex",
        });
    });
});

describe("parseRegionSize", () => {
    it.each(["", "0", "00", "0x00", "0x0"])("parses empty", (input) => {
        expect(parseRegionSize(input)).toEqual({
            val: 0,
        });
    });
    it.each(["345", "0x345", "0x3450", "34500"])(
        "detects invalid align",
        (input) => {
            expect(parseRegionSize(input)).toEqual({
                err: "align",
            });
        },
    );
    it.each(["foo00000", "0xfoo00000"])("detects invalid hex", (input) => {
        expect(parseRegionSize(input)).toEqual({
            err: "hex",
        });
    });
    it.each(["fffff000", "ffffff000", "0xfffff000"])(
        "detects overflow",
        (input) => {
            expect(parseRegionSize(input)).toEqual({
                err: "overflow",
            });
        },
    );
    it.each`
        input           | output
        ${"1000"}       | ${4096}
        ${"0x1000"}     | ${4096}
        ${"4000"}       | ${0x4000}
        ${"0x4000"}     | ${0x4000}
        ${"beef0000"}   | ${0xbeef0000}
        ${"0xbeef0000"} | ${0xbeef0000}
    `("parses valid numbers", ({ input, output }) => {
        expect(parseRegionSize(input)).toEqual({
            val: output,
        });
    });
});
