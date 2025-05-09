import "../CalamitySans.css";

import React from "react";
import { createRoot } from "react-dom/client";
import { FluentProvider, webLightTheme } from "@fluentui/react-components";

import type {
    InvView_GdtItem,
    InvView_PouchItem,
} from "@pistonite/skybook-api";
import { initI18n } from "skybook-localization";
import { registerAssetLocation } from "botw-item-assets";

import { ItemTooltipProvider } from "../tooltip";
import { CookEffect, PouchItemType, PouchItemUse } from "../data";
import {
    GdtItemSlotWithTooltip,
    PouchItemSlotWithTooltip,
    StandaloneItemSlotWithTooltip,
} from "../Wrapper.tsx";

const STANDALONE = [
    { actor: "Weapon_Sword_070" },
    { actor: "Armor_015_Head" },
    { actor: "Item_Cook_C_17", effect: CookEffect.AllSpeed },
];

const GDT: InvView_GdtItem[] = [
    {
        common: {
            actorName: "Weapon_Sword_070",
            value: 100,
            isEquipped: true,
        },
        idx: 0,
        data: {
            type: "sword",
            idx: 0,
            info: {
                flag: 0,
                value: 0,
            },
        },
    },
    {
        common: {
            actorName: "Weapon_Sword_030",
            value: 1000,
            isEquipped: false,
        },
        idx: 1,
        data: {
            type: "sword",
            idx: 2,
            info: {
                flag: 0x7fffffff,
                value: 120,
            },
        },
    },
    {
        common: {
            actorName: "Weapon_Bow_028",
            value: 6000,
            isEquipped: true,
        },
        idx: 1,
        data: {
            type: "bow",
            idx: 1,
            info: {
                flag: 0x7fffffff,
                value: 120,
            },
        },
    },
    {
        common: {
            actorName: "Item_Cook_C_17",
            value: 100,
            isEquipped: false,
        },
        idx: 2,
        data: {
            type: "food",
            idx: 0,
            info: {
                effectId: CookEffect.ExGutsMaxUp,
                effectLevel: 3,
                effectDuration: 180,
                sellPrice: 50,
                effectValue: 3,
            },
            unused_effect_1y: 0,
            ingredients: [
                "Animal_Insect_A",
                "Animal_Insect_A",
                "Animal_Insect_A",
                "Animal_Insect_A",
                "Animal_Insect_A",
            ],
        },
    },
    {
        common: {
            actorName: "Item_Roast_50",
            value: 100,
            isEquipped: false,
        },
        idx: 3,
        data: {
            type: "food",
            idx: 1,
            info: {
                effectId: CookEffect.LifeMaxUp,
                effectLevel: 3,
                effectDuration: 180,
                sellPrice: 51,
                effectValue: 3,
            },
            unused_effect_1y: 0,
            ingredients: ["", "", "", "", ""],
        },
    },
];

const POUCH: InvView_PouchItem[] = [
    {
        common: {
            actorName: "Weapon_Sword_070",
            value: 1000,
            isEquipped: false,
        },
        itemType: PouchItemType.Material,
        itemUse: PouchItemUse.Item,
        isInInventory: true,
        isNoIcon: false,
        data: {
            effectValue: 0,
            effectDuration: 0,
            effectId: -1,
            effectLevel: 0,
            sellPrice: 0,
        },
        ingredients: ["", "", "", "", ""],
        holdingCount: 2,
        promptEntangled: false,
        nodeAddr: 0n,
        nodeValid: true,
        nodePos: 419n,
        nodePrev: 0n,
        nodeNext: 0n,
        allocatedIdx: 0,
        unallocatedIdx: -1,
        tabIdx: 0,
        tabSlot: 0,
        accessible: true,
        dpadAccessible: false,
    },
    {
        common: {
            actorName: "AncientArrow",
            value: 100,
            isEquipped: true,
        },
        itemType: PouchItemType.Material,
        itemUse: PouchItemUse.Item,
        isInInventory: true,
        isNoIcon: false,
        data: {
            effectValue: 0,
            effectDuration: 0,
            effectId: -1,
            effectLevel: 0,
            sellPrice: 0,
        },
        ingredients: ["", "", "", "", ""],
        holdingCount: 0,
        promptEntangled: false,
        nodeAddr: 0n,
        nodeValid: true,
        nodePos: 419n,
        nodePrev: 0n,
        nodeNext: 0n,
        allocatedIdx: 0,
        unallocatedIdx: -1,
        tabIdx: 0,
        tabSlot: 0,
        accessible: false,
        dpadAccessible: true,
    },
    {
        common: {
            actorName: "Weapon_Bow_028",
            value: 6000,
            isEquipped: false,
        },
        itemType: PouchItemType.Bow,
        itemUse: PouchItemUse.WeaponBow,
        isInInventory: false,
        isNoIcon: false,
        data: {
            effectValue: 1200,
            effectDuration: 0,
            effectId: -1,
            effectLevel: 0,
            sellPrice: 0x7fffffff,
        },
        ingredients: ["", "", "", "", ""],
        holdingCount: 0,
        promptEntangled: true,
        nodeAddr: 0n,
        nodeValid: true,
        nodePos: 418n,
        nodePrev: 0n,
        nodeNext: 0n,
        allocatedIdx: 1,
        unallocatedIdx: -1,
        tabIdx: 0,
        tabSlot: 0,
        accessible: false,
        dpadAccessible: false,
    },
];

// eslint-disable-next-line react-refresh/only-export-components
const App: React.FC = () => {
    return (
        <>
            <p>Standalone</p>
            <div style={{ display: "flex", flexWrap: "wrap" }}>
                {STANDALONE.map((item, index) => {
                    return (
                        <StandaloneItemSlotWithTooltip key={index} {...item} />
                    );
                })}
            </div>
            <p>GDT</p>
            <div style={{ display: "flex", flexWrap: "wrap" }}>
                {GDT.map((item, index) => {
                    return (
                        <GdtItemSlotWithTooltip
                            key={index}
                            item={item}
                            isMasterSwordFullPower={false}
                        />
                    );
                })}
            </div>
            <p>POUCH</p>
            <div style={{ display: "flex", flexWrap: "wrap" }}>
                {POUCH.map((item, index) => {
                    return (
                        <PouchItemSlotWithTooltip
                            key={index}
                            item={item}
                            list1Count={1}
                            isMasterSwordFullPower={false}
                        />
                    );
                })}
            </div>
        </>
    );
};

registerAssetLocation("https://ist.pistonite.app/static/item-assets/");

void (async function main() {
    await initI18n(true);

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
