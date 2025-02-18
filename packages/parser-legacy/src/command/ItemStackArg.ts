import { convertItem, type ItemStack } from "./item.ts";
import type { AmountAllType } from "./type.ts";

export class ItemStackArg {
    public stack: ItemStack;
    public number: number | AmountAllType;
    constructor(stack: ItemStack, number: number | AmountAllType) {
        this.stack = stack;
        this.number = number;
    }

    public convert(): string {
        return `${this.number.toString().toLocaleLowerCase()} ${convertItem(this.stack)}`;
    }

    // public getContextStackAndSlotCount(): [ItemStack, number | AmountAllType] {
    //     if (this.number !== AmountAll && this.stack.item.stackable) {
    //         return [this.stack.modify({ count: this.number }), 1];
    //     }
    //     return [this.stack, this.number];
    // }
    //
    // public equals(other: ItemStackArg): boolean {
    //     return this.stack.equals(other.stack) && this.number === other.number;
    // }
}

// // converts stacks from command to stacks to add
// export const getSlotsToAdd = (stacks: ItemStackArg[]): ItemStack[] => {
//     const returnStacks: ItemStack[] = [];
//     stacks.forEach((stack) => {
//         const [actualStack, count] = stack.getContextStackAndSlotCount();
//         if (count === "All") {
//             console.error(
//                 "Unexpected count === All when all is not allowed by grammar",
//             );
//         } else {
//             for (let i = 0; i < count; i++) {
//                 returnStacks.push(actualStack);
//             }
//         }
//     });
//     return returnStacks;
// };
