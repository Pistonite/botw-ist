import { memo } from "react";
import { makeStyles } from "@fluentui/react-components";
import { ResizeLayout } from "@pistonite/shared-controls";

import { useIsShowingExtensionPanel } from "self::application/store";
import { useNarrow, isLessProductive } from "self::pure-contrib";

import { ExtensionPanel } from "./ExtensionPanel.tsx";
import { useUIStore } from "./store.ts";
import { Header } from "./Header.tsx";
import { PouchInventoryPanel } from "./PouchInventoryPanel.tsx";
import { GdtInventoryPanel } from "./GdtInventoryPanel.tsx";
import { ExtensionLaunchDialog } from "./ExtensionLaunchDialog.tsx";
import { CustomExtensionDialog } from "./CustomExtensionDialog.tsx";

const useStyles = makeStyles({
    root: {
        width: "100vw",
        height: "100vh",
    },
    side: {
        width: "100%",
        height: "100%",
        display: "flex",
        flexDirection: "column",
    },
    fullwh: {
        width: "100%",
        height: "100%",
    },
});

const AppImpl: React.FC = () => {
    const narrow = useNarrow();
    const styles = useStyles();

    const showExtensionPanel = useIsShowingExtensionPanel();

    const extensionPanelPercentage = useUIStore(
        (state) => state.extensionPanelPercentage,
    );
    const setExtensionPanelPercentage = useUIStore(
        (state) => state.setExtensionPanelPercentage,
    );

    const gamedataInventoryPercentage = useUIStore(
        (state) => state.gamedataInventoryPercentage,
    );
    const setGamedataInventoryPercentage = useUIStore(
        (state) => state.setGamedataInventoryPercentage,
    );

    return (
        <>
            <ResizeLayout
                className={styles.root}
                vertical={narrow || !showExtensionPanel}
                disabled={!showExtensionPanel}
                naturalSize={!showExtensionPanel}
                valuePercent={extensionPanelPercentage}
                setValuePercent={setExtensionPanelPercentage}
                minWidth={330}
                minHeight={45}
                touch={isLessProductive}
            >
                <div className={styles.side}>
                    <Header />
                    {showExtensionPanel && <ExtensionPanel />}
                </div>
                <main className={styles.fullwh}>
                    <ResizeLayout
                        className={styles.fullwh}
                        vertical
                        valuePercent={gamedataInventoryPercentage}
                        setValuePercent={setGamedataInventoryPercentage}
                        minHeight={60}
                    >
                        <GdtInventoryPanel />
                        <PouchInventoryPanel />
                    </ResizeLayout>
                </main>
            </ResizeLayout>
            <ExtensionLaunchDialog />
            <CustomExtensionDialog />
        </>
    );
};

export const App = memo(AppImpl);
