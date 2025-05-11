import { memo } from "react";
import { ResizeLayout } from "@pistonite/shared-controls";

import { useIsShowingExtensionPanel } from "self::application/store";
import { useNarrow, isLessProductive } from "self::pure-contrib";
import {
    ExtensionPanel,
    ExtensionLaunchDialog,
    CustomExtensionDialog,
} from "self::ui/surfaces/extension";
import { Header } from "self::ui/surfaces/header";
import {
    PouchInventoryPanel,
    GdtInventoryPanel,
} from "self::ui/surfaces/inventory";
import { useStyleEngine, useUIStore } from "self::ui/functions";

const AppImpl: React.FC = () => {
    const m = useStyleEngine();
    const narrow = useNarrow();

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
                className={m("wh-100v")}
                vertical={narrow || !showExtensionPanel}
                disabled={!showExtensionPanel}
                naturalSize={!showExtensionPanel}
                valuePercent={extensionPanelPercentage}
                setValuePercent={setExtensionPanelPercentage}
                minWidth={330}
                minHeight={45}
                touch={isLessProductive}
            >
                <div className={m("flex-col wh-100")}>
                    <Header />
                    {showExtensionPanel && <ExtensionPanel />}
                </div>
                <main className={m("wh-100")}>
                    <ResizeLayout
                        className={m("wh-100")}
                        vertical
                        valuePercent={gamedataInventoryPercentage}
                        setValuePercent={setGamedataInventoryPercentage}
                        minHeight={60}
                        minWidth={400}
                        touch={isLessProductive}
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
