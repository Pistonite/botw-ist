import { injectStyle, registerTranslationLoader } from "@pistonite/celera";

import { loadItemTranslations, loadItemUITranslations } from "#i18n";
import { type DistFileKey, DistFileMapping } from "#codegen";

let theAssetLocation: string = "";

/**
 * Register the location of the itemsys assets
 *
 * assetLocation is the URL prefix for the asset, should end with `/`.
 * The location should serve a flat list of the expected asset files
 */
export const registerAssetLocation = async (assetLocation: string) => {
    if (!assetLocation.endsWith("/")) {
        assetLocation += "/";
    }
    const css =
        makeSpriteSheetStyle(assetLocation, "chunk0x32.webp") +
        makeSpriteSheetStyle(assetLocation, "chunk1x32.webp") +
        makeSpriteSheetStyle(assetLocation, "chunk2x32.webp") +
        makeSpriteSheetStyle(assetLocation, "chunk0x64.webp") +
        makeSpriteSheetStyle(assetLocation, "chunk1x64.webp") +
        makeSpriteSheetStyle(assetLocation, "chunk2x64.webp") +
        makeSpriteSheetStyle(assetLocation, "modifiers.webp") +
        makeFontStyle(assetLocation);

    injectStyle("skybook-itemsys", css);

    theAssetLocation = assetLocation;

    // just a good place to register them :)
    await registerTranslationLoader("skybook-itemsys", loadItemTranslations);
    await registerTranslationLoader("skybook-itemsys-ui", loadItemUITranslations);
};

const makeSpriteSheetStyle = (assetLocation: string, chunk: DistFileKey & `${string}.webp`) => {
    const distChunkFile = DistFileMapping[chunk];
    const chunkKey = chunk.substring(0, chunk.length - ".webp".length);
    const chunkCSS = `.bia--sprite-${chunkKey}{background-image:url("${assetLocation}${distChunkFile}")}`;
    const maskCSS = `.bia--sprite-mask-${chunkKey}{mask-image:url("${assetLocation}${distChunkFile}")}`;
    return chunkCSS + maskCSS;
};

const makeFontStyle = (assetLocation: string) => {
    const distFontFile = DistFileMapping["Calamity-Regular.otf"];
    return `@font-face{font-family: CalamitySans; src:url("${assetLocation}${distFontFile}") format("opentype")}`;
};

export const getSheikaBackgroundUrl = () => {
    return getDistFileUrl("SheikahBackground.png");
};

export const getSheikaBackgroundLightUrl = () => {
    return getDistFileUrl("SheikahBackgroundLight.png");
};

export const getOverworldBackgroundUrl = (name: BackgroundName) => {
    return getDistFileUrl(`bg-${name}.jpg` as const);
};

export const getDistFileUrl = (file: DistFileKey) => {
    return `${theAssetLocation}${DistFileMapping[file]}`;
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
