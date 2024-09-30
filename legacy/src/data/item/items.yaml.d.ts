declare module "*.items.yaml" {
    type ItemOption = Partial<
        Readonly<{
            // If the item supports animated image. Default: false
            animated: boolean;
            // If the item is stackable. Default: true
            stackable: boolean;
            // If the item is repeatable. Default: true. If repeatable is false, stackable is automatically false
            repeatable: boolean;
            // Durability of equipments. Integer value. Pot lid is 10
            durability: number;
            // armor subtype
            subtype: "upper" | "middle" | "lower";
            // Higher priority will make the item match first in the same category when searching.
            // Default 0. Can be both positive and negative
            priority: integer;
            // bow has default zoom. default false
            bowZoom: boolean;
            // bow has default (spread) multishot, default 0
            bowMultishot: integer;
            // bow has default rapid fire (centralized multishot), default 0
            bowRapidfire: integer;
        }>
    >;
    type ItemEntry =
        | {
              [id: string]: ItemOption;
          }
        | string;
    type ItemArray = Array<ItemEntry>;
    type ItemCategory = {
        global?: ItemOption; //global option
        entries: ItemArray; //actual items
    };
    type ItemData = Partial<
        Readonly<{
            weapon: ItemCategory;
            bow: ItemCategory;
            arrow: ItemCategory;
            shield: ItemCategory;
            armor: ItemCategory;
            material: ItemCategory;
            food: ItemCategory;
            key: ItemCategory;
            flag: ItemCategory;
        }>
    >;
    const classes: ItemData;
    export default classes;
}
