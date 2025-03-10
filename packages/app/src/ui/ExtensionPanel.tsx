import { memo } from "react";
import { makeStyles } from "@fluentui/react-components";
import { ResizeLayout } from "@pistonite/shared-controls";

import {
    useCurrentPrimaryExtensionId,
    useCurrentSecondaryExtensionId,
    usePrimaryExtensionIds,
    useSecondaryExtensionIds,
} from "application/store";

import { useUIStore } from "./store.ts";
import { ExtensionToolbarPrimary } from "./ExtensionToolbarPrimary.tsx";
import { ExtensionToolbarSecondary } from "./ExtensionToolbarSecondary.tsx";
import { ExtensionWindow } from "./components/ExtensionWindow.tsx";

const useStyles = makeStyles({
    container: {
        flex: 1,
        display: "flex",
        flexDirection: "column",
        width: "100%",
        height: "100%",
    },
    main: {
        flex: 1,
        width: "100%",
        height: "100%",
    },
    extensionWindow: {
        width: "100%",
        height: "100%",
    },
});

const ExtensionPanelConnected: React.FC = () => {
    const styles = useStyles();
    const primaryIds = usePrimaryExtensionIds();
    const secondaryIds = useSecondaryExtensionIds();
    const currentPrimaryId = useCurrentPrimaryExtensionId();
    const currentSecondaryId = useCurrentSecondaryExtensionId();

    const primaryExtensionWindowPercentage = useUIStore(
        (state) => state.primaryExtensionWindowPercentage,
    );
    const setPrimaryExtensionWindowPercentage = useUIStore(
        (state) => state.setPrimaryExtensionWindowPercentage,
    );

    const primaryWindow = (
        <div className={styles.container}>
            <ExtensionToolbarPrimary />
            <ExtensionWindow ids={primaryIds} currentId={currentPrimaryId} />
        </div>
    );
    const secondaryWindow = (
        <div className={styles.container}>
            <ExtensionToolbarSecondary />
            <ExtensionWindow
                ids={secondaryIds}
                currentId={currentSecondaryId}
            />
        </div>
    );
    const hasTwoWindows = currentPrimaryId && currentSecondaryId;
    return (
        <div className={styles.main}>
            {!hasTwoWindows && primaryWindow}
            {!hasTwoWindows && secondaryWindow}
            {hasTwoWindows && (
                <ResizeLayout
                    className={styles.extensionWindow}
                    vertical
                    valuePercent={primaryExtensionWindowPercentage}
                    setValuePercent={setPrimaryExtensionWindowPercentage}
                >
                    <div className={styles.extensionWindow}>
                        {primaryWindow}
                    </div>
                    <div className={styles.extensionWindow}>
                        {secondaryWindow}
                    </div>
                </ResizeLayout>
            )}
        </div>
    );
};

export const ExtensionPanel = memo(ExtensionPanelConnected);
