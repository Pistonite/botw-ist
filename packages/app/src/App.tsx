import { makeStyles } from "@fluentui/react-components";
import { ResizeLayout } from "@pistonite/shared-controls";

import { useIsShowingExtensionPanel } from "application/store";

import { ExtensionPanel } from "ui/ExtensionPanel";
import { useNarrow } from "pure-contrib/narrow";
import { useUIStore } from "ui/store";
import { isLessProductive } from "pure-contrib/platform";

import { Header } from "./Header.tsx";

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
});

export const App: React.FC = () => {
    const narrow = useNarrow();
    const styles = useStyles();

    const showExtensionPanel = useIsShowingExtensionPanel();

    const extensionPanelPercentage = useUIStore(
        (state) => state.extensionPanelPercentage,
    );
    const setExtensionPanelPercentage = useUIStore(
        (state) => state.setExtensionPanelPercentage,
    );

    return (
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
            <main style={{ background: "green", height: "100%" }}>main</main>
        </ResizeLayout>
    );
};
