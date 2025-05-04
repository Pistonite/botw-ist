import { createSelector } from "reselect";

import { useUITranslation } from "skybook-localization";

import { BuiltinExtensionIds, useExtensionStore, type ExtensionStore, ExtensionOpenMode, getCustomExtensionId } from "./ExtensionStore.ts";
import { isLessProductive, useNarrow } from "self::pure-contrib";


/** Selector to get primary extension ids to display in the toolbar dropdown */
export const getPrimaryExtensionIdsForDropdown = createSelector(
    [
        (state: ExtensionStore) => state.primaryIds,
        (state: ExtensionStore) => state.currentPrimary,
    ],
    (ids, current) => {
        return toSortedDedupedBuiltinExtensionIds([...ids, current]);
    }
);

/** Selector to get secondary extension ids to display in the toolbar dropdown */
export const getSecondaryExtensionIdsForDropdown = createSelector(
    [
        (state: ExtensionStore) => state.secondaryIds,
        (state: ExtensionStore) => state.currentSecondary,
    ],
    (ids, current) => {
        return toSortedDedupedBuiltinExtensionIds([...ids, current]);
    }
);

/** Dedupe and sort the built-in extension ids based on predefined order */
const toSortedDedupedBuiltinExtensionIds = (ids: string[]): string[] => {
    const set = new Set<string>(ids);
    return BuiltinExtensionIds.filter((id) => set.has(id));
};

export const getOpenModeForExtension = (id: string): ExtensionOpenMode => {
    if (!(BuiltinExtensionIds as readonly string[]).includes(id)) {
        return "popout";
    }
    const { primaryIds, secondaryIds } = useExtensionStore.getState();
    if (primaryIds.includes(id)) {
        return "primary";
    }
    if (secondaryIds.includes(id)) {
        if (id === "editor") {
            return "primary";
        }
        return "secondary";
    }
    return "popout";
}

export const useExtensionName = (id: string): string => {
    const custom = useExtensionStore((state) => state.custom);
    const t = useUITranslation();
    if (id.startsWith("custom-")) {
        const config = custom.find((c) => getCustomExtensionId(c.url) === id);
            return config?.name || "";
    }
    return t(`extension.${id}.name`);
}

/** Get the effective current secondary extension id */
export const useCurrentSecondaryExtensionId = () => {
    const narrow = useNarrow();
    const secondary = useExtensionStore((state) => state.currentSecondary);
    // hide secondary extension window when the screen is narrow
    // or on less productive platforms
    return narrow || isLessProductive ? "" : secondary;
};

/** Get if the extension panel is shown */
export const useIsShowingExtensionPanel = () => {
    const primary = useExtensionStore((state) => state.currentPrimary);
    const secondary = useCurrentSecondaryExtensionId();
    return !!(primary || secondary);
};

export const getCustomExtensionConfigText = () => {
    const custom = useExtensionStore.getState().custom;
    return custom.map(({name, url}) => `${name}=${url}`).join("\n");
};
