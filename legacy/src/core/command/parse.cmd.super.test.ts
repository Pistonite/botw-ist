import { createMockItems, createMockItemSearch } from "data/test";
import { ItemStackArg } from "./ItemStackArg";
import {
    SuperCommandAddSlot,
    SuperCommandSortMaterial,
    SuperCommandSwap,
} from "./parse.cmd.super";

describe("core/command/parse.super !swap", () => {
    it("parses", () => {
        expect("!swap 5 8").toParseIntoCommand(
            undefined,
            new SuperCommandSwap(5, 8, []),
        );
    });
});

describe("core/command/parse.super !sort material", () => {
    it("parses", () => {
        expect("!sort material").toParseIntoCommand(
            undefined,
            new SuperCommandSortMaterial([]),
        );
    });
});

describe("core/command/parse.super !add slot", () => {
    it("parses", () => {
        const MockItems = createMockItems([
            "MaterialA",
            "MaterialB",
            "MaterialC",
            "Weapon1",
        ]);
        const mockSearchItem = createMockItemSearch(MockItems);

        expect(
            "! add slot 1 materialc 3 materialb[equip, life=700] from slot 7",
        ).toParseIntoCommand(
            mockSearchItem,
            new SuperCommandAddSlot(
                [
                    new ItemStackArg(MockItems.materialc.defaultStack, 1),
                    new ItemStackArg(
                        MockItems.materialb.defaultStack.modifyMeta({
                            equip: true,
                            life: 700,
                        }),
                        3,
                    ),
                ],
                7,
                [],
            ),
        );
    });
});
