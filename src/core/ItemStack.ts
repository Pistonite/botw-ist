import { Item } from "./Item";

export enum ItemType {
    Material,
    Meal,
    Key
}

export type ItemStack = {
    item: Item,
    count: number,
}
