import { memo } from "react";
import { ResizeLayout } from "@pistonite/celera";

import {
    getPrimaryExtensionIdsForDropdown,
    getSecondaryExtensionIdsForDropdown,
    useCurrentSecondaryExtensionId,
    useExtensionStore,
    useUIStore,
} from "#application";
import { ExtensionWindow } from "#ui/components";
import { useStyleEngine } from "#util";

import { ExtensionToolbarPrimary, ExtensionToolbarSecondary } from "./toolbar.tsx";

const ExtensionPanelConnected: React.FC = () => {
    const m = useStyleEngine();

    const primaryIds = useExtensionStore(getPrimaryExtensionIdsForDropdown);
    const secondaryIds = useExtensionStore(getSecondaryExtensionIdsForDropdown);
    const currentPrimaryId = useExtensionStore((state) => state.currentPrimary);
    const currentSecondaryId = useCurrentSecondaryExtensionId();

    const primaryExtensionWindowPercentage = useUIStore(
        (state) => state.primaryExtensionWindowPercentage,
    );
    const setPrimaryExtensionWindowPercentage = useUIStore(
        (state) => state.setPrimaryExtensionWindowPercentage,
    );

    const primaryWindow = (
        <div className={m("flex-col flex-1 wh-100")}>
            <ExtensionToolbarPrimary />
            <ExtensionWindow ids={primaryIds} currentId={currentPrimaryId} />
        </div>
    );
    const secondaryWindow = (
        <div className={m("flex-col flex-1 wh-100")}>
            <ExtensionToolbarSecondary />
            <ExtensionWindow ids={secondaryIds} currentId={currentSecondaryId} />
        </div>
    );
    const hasTwoWindows = currentPrimaryId && currentSecondaryId;
    return (
        <div className={m("flex-1 wh-100")}>
            {!hasTwoWindows && currentPrimaryId && primaryWindow}
            {!hasTwoWindows && currentSecondaryId && secondaryWindow}
            {hasTwoWindows && (
                <ResizeLayout
                    className={m("wh-100")}
                    vertical
                    valuePercent={primaryExtensionWindowPercentage}
                    setValuePercent={setPrimaryExtensionWindowPercentage}
                >
                    <div className={m("wh-100")}>{primaryWindow}</div>
                    <div className={m("wh-100")}>{secondaryWindow}</div>
                </ResizeLayout>
            )}
        </div>
    );
};

export const ExtensionPanel = memo(ExtensionPanelConnected);
