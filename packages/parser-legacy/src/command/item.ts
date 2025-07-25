// Type of the item
export enum ItemType {
    Weapon = 0,
    Bow = 1,
    Arrow = 2,
    Shield = 3,
    ArmorUpper = 4,
    ArmorMiddle = 5,
    ArmorLower = 6,
    Material = 7,
    Food = 8,
    Key = 9,
    Flag = -1, // flags in game data, not actual items. such as HasRitoSoulPlus
}

// V3->V4: this is the info needed to convert the input to V4 script
export type ItemStack = {
    ident: string;
    meta?: MetaModifyOption;
};

// the extra data on an item stack
export type MetaModifyOption = Partial<{
    // life value, count or durability*100
    life: number;
    // equipped.
    equip: boolean;
    // food sell price or weapon modifier
    price: number;
    // modifier hearts recover value
    hp: number;
    // food effect
    cookEffect: CookEffect;
}>;

export enum CookEffect {
    None,
    Chilly, // Alias: hotresist
    Spicy, // Alias: coldresist
    Electro,
    Sneaky, // Alias: stealth
    Energizing,
    Enduring,
    Hasty, // Alias: speed
    Mighty,
    Tough,
    Fireproof,
    Hearty,
}

export const iterateCookEffect = (): CookEffect[] => [
    CookEffect.None,
    CookEffect.Chilly,
    CookEffect.Spicy,
    CookEffect.Electro,
    CookEffect.Sneaky,
    CookEffect.Energizing,
    CookEffect.Enduring,
    CookEffect.Hasty,
    CookEffect.Mighty,
    CookEffect.Tough,
    CookEffect.Fireproof,
    CookEffect.Hearty,
];

export const WeaponModifier = {
    None: 0,
    AttackUp: 1,
    DurabilityUp: 1 << 1,
    CriticalHit: 1 << 2,
    LongThrow: 1 << 3,
    MultiShot: 1 << 4,
    Zoom: 1 << 5,
    QuickShot: 1 << 6,
    SurfMaster: 1 << 7,
    GuardUp: 1 << 8,
    Yellow: 1 << 31,
} as const;

// V3->V4: item search strings are joined with underscores
// for example `get 1 royal claymore` is `get 1 royal-claymore`
export const joinItemSearchStrings = (ids: string[]) => {
    return ids.join("-").toLowerCase().replaceAll("*", "-");
};

export const convertItem = (
    item: ItemStack,
    slotIndex: number,
    replacePlaceholder: boolean,
): string => {
    return (
        convertItemName(item.ident, replacePlaceholder) +
        convertItemMeta(item.meta, slotIndex)
    );
};

const convertItemName = (name: string, replacePlaceholder: boolean): string => {
    const matchName = name.toLowerCase().trim();
    if (replacePlaceholder) {
        // In V3, "bow", "weapon", "shield" are placeholder items.
        // These were removed in V4, since the system and syntax is much better
        if (
            matchName === "bow" ||
            matchName === "bows" ||
            matchName === "bowes"
        ) {
            return "traveller-bow";
        }
        if (
            matchName === "weapon" ||
            matchName === "weapons" ||
            matchName === "weapones"
        ) {
            return "axe";
        }
        if (
            matchName === "shield" ||
            matchName === "shields" ||
            matchName === "shieldes"
        ) {
            return "pot-lid";
        }
        if (
            matchName === "food" ||
            matchName === "foodes"
        ) {
            return "dubious-food";
        }
        if (matchName === "foods") {
            return "seafood-skewer";
        }
    }

    // In V3, "korok" is Korok Leaf, but in V4 it is Korok Seed
    // Update "korok" in V3 to "korok-leaf"
    if (matchName === "korok") {
        return "korok-leaf";
    }
    return name;
};

export const convertItemMeta = (
    meta: MetaModifyOption | undefined,
    slotIndex: number,
): string => {
    if (slotIndex > 1 || (meta && Object.keys(meta).length > 0)) {
        const props: string[] = [];
        if (meta) {
            if ("life" in meta) {
                props.push(`life=${meta.life}`);
            }
            if ("equip" in meta) {
                props.push(`equip=${meta.equip}`);
            }
            if ("price" in meta) {
                props.push(`price=${meta.price}`);
            }
            if ("hp" in meta) {
                props.push(`hp=${meta.hp}`);
            }
            if ("cookEffect" in meta && meta.cookEffect) {
                props.push(`effect=${CookEffect[meta.cookEffect]}`);
            }
        }
        if (slotIndex > 1) {
            props.push(`from-slot=${slotIndex}`);
        }
        return `[${props.join(", ")}]`;
    }
    return "";
};
