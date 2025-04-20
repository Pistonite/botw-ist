import { makeStyles } from "@fluentui/react-components";
import { ResizeLayout } from "@pistonite/shared-controls";

import { useIsShowingExtensionPanel } from "self::application/store";
import { useNarrow, isLessProductive } from "self::pure-contrib";

import { ExtensionPanel } from "./ExtensionPanel.tsx";
import { useUIStore } from "./store.ts";
import { Header } from "./Header.tsx";
import { memo } from "react";

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

const AppImpl: React.FC = () => {
    const narrow = useNarrow();
    const styles = useStyles();

    // // Boot setups tied to component lifetime
    // // so these work with HMR
    // const runtime = useRuntime();
    // useEffect(() => {
    //     console.log("setting up main app component");
    //
    //     const updateInventoryView = serial({
    //         fn: (checkCancel) => async () => {
    //         }
    //     });
    //
    //
    //     const unsubscribe = useSessionStore.subscribe((curr, prev) => {
    //         if (curr.activeScript !== prev.activeScript) {
    //
    //         }
    //     });
    //
    //     return () => {
    //         console.log("cleaning up main app component");
    //         unsubscribe();
    //     }
    // }, [runtime]);

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

export const App = memo(AppImpl);
