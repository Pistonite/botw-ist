import { describe, expect, it } from "vitest";

import {
    parseRegionStart,
    parseEnvFromScript,
    parseEnvImage,
    parseRegionSize,
} from "./envParser.ts";

describe("parseEnvFromScript", () => {
    it("parses empty", () => {
        expect(parseEnvFromScript("")).toEqual({});
        expect(parseEnvFromScript("\n")).toEqual({});
        expect(parseEnvFromScript("// comment\n")).toEqual({});
        expect(
            parseEnvFromScript("// comment\nline\n'''env\nafter ignored"),
        ).toEqual({});
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
            image: "custom-ver1.5",
            programStart: "0x0000000034500000",
            stackStart: "0x0000000034500000",
            stackSize: 0x345000,
            heapFreeSize: 0x345000,
            pmdmAddr: "0x0000000034500000",
        });
    });
});

describe("parseEnvImage", () => {
    it("parses default", () => {
        expect(parseEnvImage("")).toBe("default");
        expect(parseEnvImage("def")).toBe("default");
        expect(parseEnvImage("default")).toBe("default");
    });
    it("parses custom, unspecified dlc", () => {
        expect(parseEnvImage("custom")).toBe("custom-anyver");
        expect(parseEnvImage("custom-anyver")).toBe("custom-anyver");
        expect(parseEnvImage("custom-any")).toBe("custom-anyver");
        expect(parseEnvImage("custom-ver")).toBe("custom-anyver");
        expect(parseEnvImage("custom-ver1.5")).toBe("custom-ver1.5");
        expect(parseEnvImage("custom-1.5")).toBe("custom-ver1.5");
        expect(parseEnvImage("custom1.5")).toBe("custom-ver1.5");
        expect(parseEnvImage("custom-ver1.6")).toBe("custom-ver1.6");
        expect(parseEnvImage("custom-1.6")).toBe("custom-ver1.6");
        expect(parseEnvImage("custom1.6")).toBe("custom-ver1.6");
    });
    it("parses custom, no dlc", () => {
        expect(parseEnvImage("custom-ver1.5-nodlc")).toBe(
            "custom-ver1.5-nodlc",
        );
        expect(parseEnvImage("custom-ver1.6-no-dlc")).toBe(
            "custom-ver1.6-nodlc",
        );
        expect(parseEnvImage("custom-no-dlc-1.5")).toBe("custom-ver1.5-nodlc");
        expect(parseEnvImage("custom-no-dlc- ver1.6")).toBe(
            "custom-ver1.6-nodlc",
        );
    });
    it("parses custom, single dlc version", () => {
        expect(parseEnvImage("custom-ver1.5-dlc-1")).toBe(
            "custom-ver1.5-dlc-1",
        );
        expect(parseEnvImage("custom-ver1.6-1-dlc")).toBe(
            "custom-ver1.6-dlc-1",
        );
        expect(parseEnvImage("custom-ver1.5-dlc-2")).toBe(
            "custom-ver1.5-dlc-2",
        );
        expect(parseEnvImage("custom-ver1.6-2-dlc")).toBe(
            "custom-ver1.6-dlc-2",
        );
        expect(parseEnvImage("custom-ver1.5-dlc-3")).toBe(
            "custom-ver1.5-dlc-3",
        );
        expect(parseEnvImage("custom-ver1.6-3-dlc")).toBe(
            "custom-ver1.6-dlc-3",
        );
        // don't do this pls
        expect(parseEnvImage("custom3-ver1.5-dlc")).toBe("custom-ver1.5-dlc-3");
    });
    it("parses custom, 2 dlc versions", () => {
        expect(parseEnvImage("custom-dlc-12")).toBe("custom-anyver-dlc-1-or-2");
        expect(parseEnvImage("custom-dlc-2-or-1")).toBe(
            "custom-anyver-dlc-1-or-2",
        );
        expect(parseEnvImage("custom-dlc-1-or-2")).toBe(
            "custom-anyver-dlc-1-or-2",
        );
        expect(parseEnvImage("custom-1dlc-2")).toBe("custom-anyver-dlc-1-or-2");
        expect(parseEnvImage("custom-dlc-32")).toBe("custom-anyver-dlc-2-or-3");
        expect(parseEnvImage("custom-dlc-23")).toBe("custom-anyver-dlc-2-or-3");
        expect(parseEnvImage("custom-dlc-2-or-3")).toBe(
            "custom-anyver-dlc-2-or-3",
        );
        expect(parseEnvImage("custom-3dlc2")).toBe("custom-anyver-dlc-2-or-3");
        expect(parseEnvImage("custom-1dlc-3")).toBe("custom-anyver-dlc-1-or-3");
        expect(parseEnvImage("custom-dlc-31")).toBe("custom-anyver-dlc-1-or-3");
        expect(parseEnvImage("custom-dlc-13")).toBe("custom-anyver-dlc-1-or-3");
        expect(parseEnvImage("custom-dlc-1-or-3")).toBe(
            "custom-anyver-dlc-1-or-3",
        );
        expect(parseEnvImage("custom-3dlc1")).toBe("custom-anyver-dlc-1-or-3");
    });
    it("parses custom, 3 dlc versions", () => {
        expect(parseEnvImage("custom-ver1.5-dlc")).toBe("custom-ver1.5-dlc");
        expect(parseEnvImage("custom-ver1.5-dlc-123")).toBe(
            "custom-ver1.5-dlc",
        );
        expect(parseEnvImage("custom-ver1.5-dlc-1-or-2-or-3")).toBe(
            "custom-ver1.5-dlc",
        );
        expect(parseEnvImage("custom-ver1.5-1dlc23")).toBe("custom-ver1.5-dlc");
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
