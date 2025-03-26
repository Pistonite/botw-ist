import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import {
    FluentProvider,
    Switch,
    webLightTheme,
} from "@fluentui/react-components";

import type { ItemSlotInfo } from "@pistonite/skybook-api";
import { initI18n } from "skybook-localization";

import { ItemTooltipProvider } from "../ItemTooltipProvider";
import { CookEffect, ItemUse, PouchItemType } from "../data/enums.ts";
import { ItemSlot } from "../ItemSlot";
import { ItemTooltip } from "../ItemTooltip.tsx";

const DUMMY: ItemSlotInfo = {
    actorName: "Dummy",
    itemType: PouchItemType.Sword,
    itemUse: ItemUse.WeaponSmallSword,
    value: 0,
    isEquipped: false,
    isInInventory: true,
    modEffectValue: 0,
    modEffectDuration: 0,
    modSellPrice: 0,
    modEffectId: CookEffect.None,
    modEffectLevel: 0,
    ingredientActorNames: ["", "", "", "", ""],
    listPos: 0,
    unallocated: false,
    poolPos: 0,
    isInBrokenSlot: false,
    holdingCount: 0,
    promptEntangled: false,
};

const TEST_ITEMS: ItemSlotInfo[] = [
    {
        ...DUMMY,
        actorName: "Weapon_Sword_070",
        itemType: PouchItemType.Sword,
        itemUse: ItemUse.WeaponSmallSword,
        value: 4000,
    },
    {
        ...DUMMY,
        actorName: "Obj_HeroSoul_Zora",
        itemType: PouchItemType.KeyItem,
    },
    {
        ...DUMMY,
        actorName: "Item_Ore_A",
        itemType: PouchItemType.Material,
        value: 99999,
    },
    {
        ...DUMMY,
        actorName: "Weapon_Lsword_010",
        itemType: PouchItemType.Sword,
        itemUse: ItemUse.WeaponLargeSword,
        value: 3850,
    },
    {
        ...DUMMY,
        actorName: "Item_PlantGet_Q",
        itemType: PouchItemType.Material,
        value: 5,
        holdingCount: 4,
    },
    {
        ...DUMMY,
        actorName: "Obj_DLC_HeroSoul_Goron",
        itemType: PouchItemType.KeyItem,
    },
    {
        ...DUMMY,
        actorName: "Weapon_Sword_502",
        itemType: PouchItemType.Sword,
        itemUse: ItemUse.WeaponSmallSword,
        value: 100,
    },
    {
        ...DUMMY,
        actorName: "Armor_011_Lower",
        itemType: PouchItemType.ArmorLower,
        itemUse: ItemUse.ArmorLower,
        value: 1,
    },
    {
        ...DUMMY,
        actorName: "Weapon_Bow_028",
        itemType: PouchItemType.Bow,
        itemUse: ItemUse.WeaponBow,
        value: 6000,
    },
    {
        ...DUMMY,
        actorName: "Weapon_Bow_028",
        itemType: PouchItemType.Bow,
        itemUse: ItemUse.WeaponBow,
        value: 6000,
        modEffectValue: 120,
        modSellPrice: 147,
    },
    {
        ...DUMMY,
        actorName: "Weapon_Shield_001",
        itemType: PouchItemType.Shield,
        itemUse: ItemUse.WeaponShield,
        value: 1000,
        modEffectValue: 120,
        modSellPrice: 0xffffffff,
    },
    {
        ...DUMMY,
        actorName: "Item_Cook_A_01",
        itemType: PouchItemType.Food,
        itemUse: ItemUse.CureItem,
        value: 1,
        modEffectValue: 120,
        isInInventory: false,
        modSellPrice: 115,
    },
    {
        ...DUMMY,
        actorName: "Item_Cook_C_17",
        itemType: PouchItemType.Food,
        itemUse: ItemUse.CureItem,
        value: 1,
        modEffectValue: 120,
        modEffectId: CookEffect.ExGutsMaxUp,
        modSellPrice: 115,
    },
    {
        ...DUMMY,
        actorName: "Armor_075_Head",
        itemType: PouchItemType.ArmorUpper,
        itemUse: ItemUse.ArmorUpper,
        value: 1,
    },
    {
        ...DUMMY,
        actorName: "Item_Cook_C_17",
        itemType: PouchItemType.Food,
        itemUse: ItemUse.CureItem,
        value: 1,
        modEffectValue: 25,
        modEffectId: CookEffect.LifeMaxUp,
    },
    {
        ...DUMMY,
        actorName: "Item_Cook_C_17",
        itemType: PouchItemType.Food,
        itemUse: ItemUse.CureItem,
        value: 1,
        modEffectValue: 25,
        modEffectId: CookEffect.GutsRecover,
        modEffectLevel: 1000,
    },
    {
        ...DUMMY,
        actorName: "Item_Cook_C_17",
        itemType: 78,
        itemUse: 87,
        value: 1,
        modEffectValue: 40,
        modEffectId: CookEffect.AllSpeed,
        modEffectLevel: 3,
        modEffectDuration: 3600,
    },
];

// eslint-disable-next-line react-refresh/only-export-components
const App: React.FC = () => {
    const [cheap, setCheap] = useState(false);
    const [isEquipped, setIsEquipped] = useState(false);
    const [isInBrokenSlot, setIsInBrokenSlot] = useState(false);
    const [deactive, setDeactive] = useState(false);
    const [badlyDamaged, setBadlyDamaged] = useState(false);
    const [animation, setAnimation] = useState(true);
    const [entangled, setEntangled] = useState(false);

    const items = TEST_ITEMS.map((item, i) => {
        return {
            ...item,
            isEquipped,
            isInBrokenSlot,
            listPosition: i,
            promptEntangled: entangled,
            ...(badlyDamaged ? { value: 200 } : {}),
        };
    });

    return (
        <>
            <div>
                <Switch
                    checked={cheap}
                    label="Cheap"
                    onChange={(_, { checked }) => {
                        setCheap(!!checked);
                    }}
                />
                <Switch
                    checked={isEquipped}
                    label="Equip"
                    onChange={(_, { checked }) => {
                        setIsEquipped(!!checked);
                    }}
                />
                <Switch
                    checked={isInBrokenSlot}
                    label="Broken"
                    onChange={(_, { checked }) => {
                        setIsInBrokenSlot(!!checked);
                    }}
                />
                <Switch
                    checked={deactive}
                    label="Deactive"
                    onChange={(_, { checked }) => {
                        setDeactive(!!checked);
                    }}
                />
                <Switch
                    checked={badlyDamaged}
                    label="Badly Damaged"
                    onChange={(_, { checked }) => {
                        setBadlyDamaged(!!checked);
                    }}
                />
                <Switch
                    checked={animation}
                    label="Animation"
                    onChange={(_, { checked }) => {
                        setAnimation(!!checked);
                    }}
                />
                <Switch
                    checked={entangled}
                    label="Entangled"
                    onChange={(_, { checked }) => {
                        setEntangled(!!checked);
                    }}
                />
            </div>
            <div style={{ display: "flex", flexWrap: "wrap" }}>
                {items.map((item, index) => {
                    return (
                        <ItemTooltip info={item} key={index}>
                            <ItemSlot
                                info={item}
                                cheap={cheap}
                                deactive={deactive}
                                disableAnimation={!animation}
                            />
                        </ItemTooltip>
                    );
                })}
            </div>
        </>
    );
};

void (async function main() {
    await initI18n(false);

    const root = document.getElementById("root");
    if (root) {
        createRoot(root).render(
            <React.StrictMode>
                <FluentProvider theme={webLightTheme}>
                    <ItemTooltipProvider backgroundUrl="/SheikahBackground.png">
                        <App />
                    </ItemTooltipProvider>
                </FluentProvider>
            </React.StrictMode>,
        );
    }
})();
