import { injectStyle } from "@pistonite/pure/pref";

let theAssetLocation: string = "";

/**
 * Register the location of the item assets
 *
 * assetLocation is the URL prefix for the asset, should end with `/`.
 * The location should have `special` and `sprites` directory
 */
export const registerAssetLocation = (assetLocation: string) => {
    const css =
        makeSpriteSheetStyle(assetLocation, "chunk0x32") +
        makeSpriteSheetStyle(assetLocation, "chunk1x32") +
        makeSpriteSheetStyle(assetLocation, "chunk2x32") +
        makeSpriteSheetStyle(assetLocation, "chunk0x64") +
        makeSpriteSheetStyle(assetLocation, "chunk1x64") +
        makeSpriteSheetStyle(assetLocation, "chunk2x64") +
        makeSpriteSheetStyle(assetLocation, "modifiers") +
        makeFontStyle(assetLocation);

    injectStyle("botw-item-assets", css);

    theAssetLocation = assetLocation;
};

const makeSpriteSheetStyle = (assetLocation: string, chunk: string) => {
    const chunkCSS = `.bia--sprite-${chunk}{background-image:url("${assetLocation}sprites/${chunk}.webp")}`;
    const maskCSS = `.bia--sprite-mask-${chunk}{mask-image:url("${assetLocation}sprites/${chunk}.webp")}`;
    return chunkCSS + maskCSS;
};

const makeFontStyle = (assetLocation: string) => {
    return `@font-face{font-family: CalamitySans; src:url("${assetLocation}fonts/Calamity-Regular.otf") format("opentype")}`;
};

export const getSpecialIconUrl = (file: string) => {
    return `${theAssetLocation}special/${file}`;
};

export const getSheikaBackgroundUrl = () => {
    return `${theAssetLocation}images/SheikahBackground.png`;
};

export const getSheikaBackgroundLightUrl = () => {
    return `${theAssetLocation}images/SheikahBackgroundLight.png`;
};

export const getOverworldBackgroundUrl = (name: BackgroundName) => {
    return `${theAssetLocation}images/bg-${name}.jpg`;
};

export type BackgroundName =
    | "gerudo"
    | "goron"
    | "hateno"
    | "kakariko"
    | "korok-forest"
    | "plateau"
    | "sor"
    | "rito"
    | "zora";
