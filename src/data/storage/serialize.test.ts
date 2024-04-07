import { compressString, decompressString } from "./serialize";

const runCompressDecompressTest = (input: string) => {
    const compressed = compressString(input);
    expect(compressed).not.toEqual(input);
    expect(decompressString(compressed)).toEqual(input);
};

describe.only("data/serialize.compress", () => {
    it("Should compress and decompress empty string", () => {
        runCompressDecompressTest("");
    });
    it("Should compress and decompress one character", () => {
        runCompressDecompressTest("a");
    });
    it("Should compress and decompress single command", () => {
        const input = "Break 5 Slots";
        runCompressDecompressTest(input);
    });
    it("Should compress and decompress large command", () => {
        const input =
            "Initialize 1 Tree Branch[equip] 1 Hammer 1 Travel Bow[Equip] 3 NormalArrow[Equip] 1 potlid 1 potlid[equip] 1 Fairy 1 SpeedFood 3 EnduraFood 1 Slate 9 SpiritOrb 1 Glider";
        runCompressDecompressTest(input);
    });
    it("Should compress and decompress multiple commands", () => {
        const inputArray = [
            "Break 5 Slots",
            "Save",
            "Eat 3 EnduraFood",
            "Eat SpeedFood",
        ];
        runCompressDecompressTest(inputArray.join("\n"));
    });
});
