import { injectStyle } from "@pistonite/pure/pref";

let theAssetLocation: string = "";

/**
 * Register the location of the item assets
 *
 * assetLocation is the URL prefix for the asset, should end with `/`.
 * The location should have `special` and `sprites` directory
 */
export const registerSpriteLocation = (assetLocation: string) => {
    const css =
        makeSpriteSheetStyle(assetLocation, "chunk0x32") +
        makeSpriteSheetStyle(assetLocation, "chunk1x32") +
        makeSpriteSheetStyle(assetLocation, "chunk2x32") +
        makeSpriteSheetStyle(assetLocation, "chunk0x64") +
        makeSpriteSheetStyle(assetLocation, "chunk1x64") +
        makeSpriteSheetStyle(assetLocation, "chunk2x64") +
        makeSpriteSheetStyle(assetLocation, "modifiers");

    injectStyle("botw-item-assets", css);

    theAssetLocation = assetLocation;
};

const makeSpriteSheetStyle = (assetLocation: string, chunk: string) => {
    const chunkCSS = `.bia--sprite-${chunk}{background-image:url("${assetLocation}sprites/${chunk}.webp")}`;
    const maskCSS = `.bia--sprite-mask-${chunk}{mask-image:url("${assetLocation}sprites/${chunk}.webp")}`;
    return chunkCSS + maskCSS;
};

export const getSpecialIconUrl = (file: string) => {
    return `${theAssetLocation}special/${file}`;
};
