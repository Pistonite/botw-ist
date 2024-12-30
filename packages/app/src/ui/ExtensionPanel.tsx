import { memo } from "react";
import { ExtensionTitlebar, ExtensionTitlebarMobile } from "./ExtensionTitlebar";
import { isLessProductive } from "./platform";
import { makeStyles } from "@fluentui/react-components";
import { useCurrentPrimaryExtensionId, useCurrentSecondaryExtensionId, usePrimaryExtensionIds, useSecondaryExtensionIds } from "application/extensionStore";
import { ExtensionWrapper } from "extensions/ExtensionWindow";
import { useUIStore } from "./store";
import { ResizeLayout } from "./components/ResizeLayout";

const useStyles = makeStyles({
    container: {
        flex: 1,
        display: "flex",
        flexDirection: "column",
    },
    main: {
        flex: 1,
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

    const primaryExtensionWindowPercentage = useUIStore(state => state.primaryExtensionWindowPercentage);
    const setPrimaryExtensionWindowPercentage = useUIStore(state => state.setPrimaryExtensionWindowPercentage);

    const primaryWindow =  (
            primaryIds.map((id, i) => (
            <div key={i} data-extension-id={id} className={styles.extensionWindow} style={{
                        display: id === currentPrimaryId ? "block" : "none",
                    }}>
                <ExtensionWrapper id={id} />
            </div>
            ))
        );
    const secondaryWindow = (
            secondaryIds.map((id, i) => (
            <div key={i} data-extension-id={id} className={styles.extensionWindow} style={{
                        display: id === currentSecondaryId ? "block" : "none",
                    }}>
                <ExtensionWrapper id={id} />
            </div>
            ))
        );
    const hasTwoWindows = currentPrimaryId && currentSecondaryId;
    return (
    <div className={styles.container}>
            {
                isLessProductive ?
                <ExtensionTitlebarMobile />
                :
            <ExtensionTitlebar />
            }
            <div className={styles.main}>
                {
                    !hasTwoWindows && primaryWindow
                }
                {
                    !hasTwoWindows && secondaryWindow
                }
                {
                    hasTwoWindows && (
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
                    )
                }
            </div>
    </div>
    );
};

export const ExtensionPanel = memo(ExtensionPanelConnected);
