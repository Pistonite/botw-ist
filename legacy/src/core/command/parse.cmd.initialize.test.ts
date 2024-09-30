import { createMockItemSearch, createMockItems } from "data/test";
import { ItemStackArg } from "./ItemStackArg";
import { CmdErr } from "./command";
import { CommandInitialize } from "./parse.cmd.initialize";

describe("core/command/parse.initialize", () => {
    const MockItems = createMockItems([
        "MaterialA",
        "MaterialB",
        "MaterialC",
        "Weapon1",
    ]);
    const mockSearchItem = createMockItemSearch(MockItems);
    it("parses hint when failed", () => {
        expect("initialize ???").toParseIntoCommand(
            mockSearchItem,
            CmdErr.Guess,
        );
    });
    it("parses empty items", () => {
        expect("initialize").toParseIntoCommand(
            mockSearchItem,
            new CommandInitialize([], []),
        );
    });
    it("parses single item", () => {
        expect("initialize Mat eri al A").toParseIntoCommand(
            mockSearchItem,
            new CommandInitialize(
                [new ItemStackArg(MockItems.materiala.defaultStack, 1)],
                [],
            ),
        );
    });
    it("parses list of items", () => {
        expect(
            "initialize 1 Material A 2 Material B 3 Material C",
        ).toParseIntoCommand(
            mockSearchItem,
            new CommandInitialize(
                [
                    new ItemStackArg(MockItems.materiala.defaultStack, 1),
                    new ItemStackArg(MockItems.materialb.defaultStack, 2),
                    new ItemStackArg(MockItems.materialc.defaultStack, 3),
                ],
                [],
            ),
        );
    });
    it("parses list of items with meta", () => {
        expect(
            "initialize 1 Material A 2 Material B 1 weapon1 [equip]",
        ).toParseIntoCommand(
            mockSearchItem,
            new CommandInitialize(
                [
                    new ItemStackArg(MockItems.materiala.defaultStack, 1),
                    new ItemStackArg(MockItems.materialb.defaultStack, 2),
                    new ItemStackArg(
                        MockItems.weapon1.defaultStack.modifyMeta({
                            equip: true,
                        }),
                        1,
                    ),
                ],
                [],
            ),
        );
    });
    it("parses repeated as separate slots", () => {
        expect("initialize 2 materialb 2 materialb").toParseIntoCommand(
            mockSearchItem,
            new CommandInitialize(
                [
                    new ItemStackArg(MockItems.materialb.defaultStack, 2),
                    new ItemStackArg(MockItems.materialb.defaultStack, 2),
                ],
                [],
            ),
        );
    });
});
