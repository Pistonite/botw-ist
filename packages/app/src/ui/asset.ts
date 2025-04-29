import { useDark } from "@pistonite/pure-react";
import { registerAssetLocation } from "botw-item-assets";

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
        console.log("[dev] using local item-assets");
        return;
    } catch {
        console.log("[dev] item-assets probing failed, using hosted");
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

export const getOverworldBackgroundUrl = () => {
    return `${assetPrefix}static/item-system/bg-hateno.jpg`;
};
