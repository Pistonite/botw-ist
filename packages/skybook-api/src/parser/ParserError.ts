// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Category } from "./Category";
import type { MetaValue } from "./MetaValue";

/**
 * Error type for the parser
 */
export type ParserError =
    | { type: "Unexpected"; data: string }
    | { type: "SyntaxUnexpected" }
    | { type: "SyntaxUnexpectedExpecting"; data: string }
    | { type: "SyntaxUnexpectedEof" }
    | { type: "InvalidItem"; data: string }
    | { type: "InvalidEmptyItem" }
    | { type: "IntFormat"; data: string }
    | { type: "FloatFormat"; data: string }
    | { type: "UnusedMetaKey"; data: string }
    | { type: "InvalidMetaValue"; data: [string, MetaValue] }
    | { type: "InvalidWeaponModifier"; data: string }
    | { type: "InvalidCookEffect"; data: string }
    | { type: "TooManyIngredients" }
    | { type: "InvalidArmorStarNum"; data: number }
    | { type: "InvalidSlotClause"; data: number }
    | { type: "InvalidTimesClause"; data: number }
    | { type: "InvalidTrial"; data: string }
    | { type: "InvalidCategory"; data: Category }
    | { type: "InvalidInventoryRow"; data: number }
    | { type: "InvalidInventoryCol"; data: number };
