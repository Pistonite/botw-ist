import { useDark } from "@pistonite/pure-react";

import {
    type BackgroundName,
    getSheikaBackgroundLightUrl,
    getSheikaBackgroundUrl,
    registerAssetLocation,
} from "@pistonite/skybook-itemsys";

import { devLog } from "./log.ts";

export const probeAndRegisterAssetLocation = async () => {
    if (!import.meta.env.DEV) {
        registerAssetLocation("/static/itemsys/");
        return;
    }
    try {
        const response = await fetch("/static/itemsys/sprites/modifiers.webp");
        if (response.ok) {
            registerAssetLocation("/static/itemsys/");
        }
        devLog.info("using local item assets");
        return;
    } catch {
        devLog.info("item-assets probing failed, using hosted");
        registerAssetLocation("https://ist.pistonite.app/static/itemsys/");
    }
};

export const useThemedSheikaBackgroundUrl = () => {
    const dark = useDark();
    return dark ? getSheikaBackgroundUrl() : getSheikaBackgroundLightUrl();
};

const BackgroundGacha: [BackgroundName, number][] = [
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
