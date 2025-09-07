/**
 * This is the main app UI
 */
import { memo, useEffect } from "react";
import { ResizeLayout } from "@pistonite/shared-controls";

import { useUITranslation } from "skybook-localization";
import { ItemTooltipProvider } from "skybook-item-system";

import { MainWindowItemDnDProvider, useIsShowingExtensionPanel, useSessionStore, useUIStore } from "self::application";
import { useNarrow, isLessProductive } from "self::pure-contrib";
import {
    ExtensionPanel,
    ExtensionLaunchDialog,
    CustomExtensionDialog,
} from "self::ui/surfaces/extension";
import { Header } from "self::ui/surfaces/header";
import { PouchInventoryPanel, GdtInventoryPanel } from "self::ui/surfaces/inventory";
import { getSheikaBackgroundUrl, useStyleEngine } from "self::util";

const AppImpl: React.FC = () => {
    const m = useStyleEngine();
    const t = useUITranslation();
    const narrow = useNarrow();

    // save the crash localization to localstorage, so we can
    // use it in the future if needed without relying on the translation
    // component being functional
    const crashScreenTitle = t("crash_screen.title");
    const crashScreenDesc = t("crash_screen.desc");
    const crashScreenButtonSaveReload = t("crash_screen.button.save_reload");
    const crashScreenButtonReload = t("crash_screen.button.reload");
    useEffect(() => {
        try {
            localStorage.setItem(
                "Skybook.CrashScreenLocalization",
                JSON.stringify({
                    title: crashScreenTitle,
                    desc: crashScreenDesc,
                    button_save_reload: crashScreenButtonSaveReload,
                    button_reload: crashScreenButtonReload,
                }),
            );
        } catch {
            // ignore error
        }
    }, [crashScreenTitle, crashScreenDesc, crashScreenButtonSaveReload, crashScreenButtonReload]);

    const showExtensionPanel = useIsShowingExtensionPanel();

    const extensionPanelPercentage = useUIStore((state) => state.extensionPanelPercentage);
    const setExtensionPanelPercentage = useUIStore((state) => state.setExtensionPanelPercentage);

    const gamedataInventoryPercentage = useUIStore((state) => state.gamedataInventoryPercentage);
    const setGamedataInventoryPercentage = useUIStore(
        (state) => state.setGamedataInventoryPercentage,
    );

    const dragData = useSessionStore(state => state.dragData);

    const $App = 
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
        </>;

    return (
        <MainWindowItemDnDProvider>
            <ItemTooltipProvider 
                suppress={!!dragData}
                backgroundUrl={getSheikaBackgroundUrl()}>
                {$App}
            </ItemTooltipProvider>
        </MainWindowItemDnDProvider>
    );
};

export const App = memo(AppImpl);
