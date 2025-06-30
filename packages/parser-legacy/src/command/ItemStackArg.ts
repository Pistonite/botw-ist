import { convertItem, type ItemStack } from "./item.ts";
import type { AmountAllType } from "./type.ts";

export class ItemStackArg {
    public stack: ItemStack;
    public number: number | AmountAllType;
    constructor(stack: ItemStack, number: number | AmountAllType) {
        this.stack = stack;
        this.number = number;
    }

    public convert(slotIndex: number): string {
        return `${this.number.toString().toLocaleLowerCase()} ${convertItem(this.stack, slotIndex)}`;
    }
}
