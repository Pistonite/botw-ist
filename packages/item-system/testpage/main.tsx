import React, { useState } from "react";
import { createRoot } from "react-dom/client";
import { FluentProvider, Switch, webDarkTheme, webLightTheme } from "@fluentui/react-components";

import { ItemTooltipProvider } from "../src/ItemTooltipProvider";
import { ItemSlotInfo, ItemUse, PouchItemType } from "../src/ItemSlotInfo";
import { ItemSlot } from "../src/ItemSlot";

const DUMMY: ItemSlotInfo = {
    actorName: "Dummy",
    itemType: PouchItemType.Sword,
    itemUse: ItemUse.WeaponSmallSword,
    value: 0,
    isEquipped: false,
    isInInventory: false,
    modEffectValue: 0,
    modEffectDuration: 0,
    modSellPrice: 0,
    modEffectId: 0,
    modEffectLevel: 0,
    ingredientActorNames: [],
    listPosition: 0,
    unallocated: false,
    poolPosition: 0,
    isInBrokenSlot: false,
    holdingCount: 0,
    promptEntangled: false,
}

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
];

const App: React.FC = () => {
    const [cheap, setCheap] = useState(false);
    const [isEquipped, setIsEquipped] = useState(false);
    const [isInBrokenSlot, setIsInBrokenSlot] = useState(false);
    const [badlyDamaged, setBadlyDamaged] = useState(false);
    const [animation, setAnimation] = useState(true);
    const [entangled, setEntangled] = useState(false);

    const items = TEST_ITEMS.map((item, i) => {
        return {
            ...item, isEquipped, isInBrokenSlot, listPosition: i,
            promptEntangled: entangled,
            ...(badlyDamaged ? { value: 200 } : {})
        }
    });

    return <>
    <div>
            <Switch checked={cheap} label="Cheap" onChange={(_, {checked}) => {
                setCheap(!!checked);
            }} />
            <Switch checked={isEquipped} label="Equip" onChange={(_, {checked}) => {
                setIsEquipped(!!checked);
            }} />
            <Switch checked={isInBrokenSlot} label="Broken" onChange={(_, {checked}) => {
                setIsInBrokenSlot(!!checked);
            }} />
            <Switch checked={badlyDamaged} label="Badly Damaged" onChange={(_, {checked}) => {
                setBadlyDamaged(!!checked);
            }} />
            <Switch checked={animation} label="Animation" onChange={(_, {checked}) => {
                setAnimation(!!checked);
            }} />
            <Switch checked={entangled} label="Entangled" onChange={(_, {checked}) => {
                setEntangled(!!checked);
            }} />
    </div>
        <div style={{display: "flex"}}>
            {
                items.map((item, index) => {
                    return <ItemSlot 
                        key={index} 
                        info={item}
                        cheap={cheap}
                        disableAnimation={!animation}
                    />;
                })
            }
        </div>
    </>;
};

const root = document.getElementById('root');
if (root) {
    createRoot(root).render(
        <React.StrictMode>
            <FluentProvider theme={webLightTheme}>
                <ItemTooltipProvider>
                    <App />
                </ItemTooltipProvider>
            </FluentProvider>
        </React.StrictMode>
    );
}

