import { useDark } from "@pistonite/pure-react";

import { registerAssetLocation } from "botw-item-assets";

import { devLog } from "./log.ts";

let assetPrefix = "/";

export const probeAndRegisterAssetLocation = async () => {
    if (!import.meta.env.DEV) {
        registerAssetLocation("/static/item-assets/");
        return;
    }
    try {
        const response = await fetch(
            "/static/item-assets/sprites/modifiers.webp",
        );
        if (response.ok) {
            registerAssetLocation("/static/item-assets/");
        }
        devLog.info("using local item-assets");
        return;
    } catch {
        devLog.info("[dev] item-assets probing failed, using hosted");
        registerAssetLocation("https://ist.pistonite.app/static/item-assets/");
        assetPrefix = "https://ist.pistonite.app/";
    }
};

export const getSheikaBackgroundUrl = () => {
    return `${assetPrefix}static/item-system/SheikahBackground.png`;
};

export const getSheikaBackgroundLightUrl = () => {
    return `${assetPrefix}static/item-system/SheikahBackgroundLight.png`;
};

export const useThemedSheikaBackgroundUrl = () => {
    const dark = useDark();
    return dark ? getSheikaBackgroundUrl() : getSheikaBackgroundLightUrl();
};

export const getOverworldBackgroundUrl = (name: BackgroundName) => {
    return `${assetPrefix}static/item-system/bg-${name}.jpg`;
};

const BackgroundGacha = [
    ["gerudo", 0.1],
    ["goron", 0.1],
    ["hateno", 0.25],
    ["kakariko", 0.25],
    ["korok-forest", 0.06],
    ["plateau", 0.03],
    ["sor", 0.01],
    ["rito", 0.1],
    ["zora", 0.1],
] as const;
export type BackgroundName = (typeof BackgroundGacha)[number][0];
export const getRandomBackgroundName = (
    current: BackgroundName,
    hasGlider: boolean,
): BackgroundName => {
    if (!hasGlider) {
        return "plateau";
    }
    outer: for (let i = 0; i < 100; i++) {
        const x = Math.random();
        let c = 0;
        for (const [name, chance] of BackgroundGacha) {
            c += chance;
            if (x < c) {
                if (name === current) {
                    continue outer;
                }
                return name;
            }
        }
    }
    // not reachable
    return "hateno";
};
