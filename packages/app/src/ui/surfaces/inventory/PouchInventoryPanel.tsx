import { memo, useMemo } from "react";
import {
    makeStyles,
    Tooltip,
    ToggleButton,
    Button,
    Link,
} from "@fluentui/react-components";
import { Grid20Regular, Info20Regular } from "@fluentui/react-icons";
import { useSwappedWheelScrollDirection } from "@pistonite/shared-controls";

import {
    ItemTab,
    OverworldItemSlotWithTooltip,
    PouchItemSlotWithTooltip,
} from "skybook-item-system";
import {
    translateRuntimeViewError,
    useUITranslation,
} from "skybook-localization";

import {
    useGdtInventoryView,
    useItemSlotPropsFromSettings,
    useOverworldItemsView,
    usePouchListView,
} from "self::application/store";
import {
    getOverworldBackgroundUrl,
    useUIStore,
    getTabNodesFromPouch,
    useStyleEngine,
} from "self::ui/functions";
import {
    InventoryTitle,
    InventoryTabButton,
    InventorySpinner,
    ErrorBar,
} from "self::ui/components";
import { openExtension } from "self::application/extension";

import { ScreenIndicator } from "./ScreenIndicator.tsx";

const useStyles = makeStyles({
    splitContainer: {
        zIndex: 1,
    },
    inventoryContainer: {
        padding: "8px",
        backgroundColor: "#00000066",
    },
    background: {
        // This is to hide the UI elements like health bar, etc
        scale: 1.3,
        backgroundPosition: "center",
        backgroundRepeat: "no-repeat",
        backgroundSize: "cover",
    },

    toolbar: {
        padding: "4px",
        gap: "4px",
        backgroundColor: "#00000044",
        borderRadius: "4px",
    },

    overworldScroll: {
        padding: "8px",
        maxHeight: "72px",
    },

    tabsMinimap: {
        borderRadius: "4px",
        backgroundColor: "#00000044",
    },

    errors: {
        gap: "4px",
    },
});

export const PouchInventoryPanelImpl: React.FC = () => {
    const m = useStyleEngine();
    const c = useStyles();
    const t = useUITranslation();

    const { data: pouch, loading, error: pouchError } = usePouchListView();
    const {
        data: overworld,
        loading: overworldLoading,
        error: overworldError,
    } = useOverworldItemsView();
    const { data: gdt, error: gdtError } = useGdtInventoryView();

    const showSpinner = loading || !pouch || overworldLoading;

    const itemSlotProps = useItemSlotPropsFromSettings();
    const isMasterSwordFullPower = !!gdt?.val?.masterSword?.isTrueForm;

    const isTabView = useUIStore((state) => state.isTabViewEnabled);
    const setTabView = useUIStore((state) => state.setIsTabViewEnabled);
    const tabNodes = useMemo(() => {
        if (!isTabView) {
            return undefined;
        }
        return getTabNodesFromPouch(pouch?.val);
    }, [pouch, isTabView]);

    const $Background = (
        <div
            className={m("pos-abs all-sides-0", c.background)}
            style={{
                backgroundImage: `url(${getOverworldBackgroundUrl()})`,
            }}
        />
    );

    const $Title = (
        <InventoryTitle title={t("main.visible_inventory.title")} dark>
            <Tooltip
                relationship="description"
                content={t("main.visible_inventory.desc")}
                withArrow
                positioning="below"
            >
                <Button icon={<Info20Regular />} appearance="transparent" />
            </Tooltip>
            <div className={m("flex-row flex-1 flex-centera", c.toolbar)}>
                <ScreenIndicator screen={pouch?.val?.screen}/>
                <Tooltip
                    relationship="label"
                    content={t("main.tabbed_inventory.button_tooltip")}
                    withArrow
                    positioning="below"
                >
                    <ToggleButton
                        checked={isTabView}
                        onClick={() => {
                            setTabView(!isTabView);
                        }}
                        icon={<Grid20Regular />}
                        appearance={isTabView ? "primary" : "transparent"}
                    />
                </Tooltip>
            </div>
        </InventoryTitle>
    );

    const { ref: tabbedScrollHandler, scrollToH: scrollTabPanel } =
        useSwappedWheelScrollDirection();
    const $TabsMinimap = tabNodes !== undefined && (
        <div className={m("overflow-y-auto scrollbar-thin")}>
            <div className={m("flex-row", c.tabsMinimap)}>
                {tabNodes.map((tab, i) => (
                    <InventoryTabButton
                        key={i}
                        category={tab.category}
                        onClick={() => {
                            const panel = document.getElementById(
                                "-tabbed-pouch-scroll-",
                            );
                            const targetTab = document.getElementById(
                                `-pouch-tab-${i}-`,
                            );
                            if (!targetTab || !panel) {
                                return;
                            }
                            const width = targetTab.clientWidth;
                            const offset = panel.clientWidth / 2;
                            scrollTabPanel(width * i + width / 2 - offset);
                        }}
                    />
                ))}
            </div>
        </div>
    );

    const $TabsWarning = isTabView && pouch && tabNodes === undefined && (
        <ErrorBar title={t("main.tabbed_inventory.bad_msgbar.title")}>
            {t("main.tabbed_inventory.bad_msgbar.body")}
        </ErrorBar>
    );

    const $PouchError = pouch?.err && (
        <>
            <ErrorBar title={t("main.visible_inventory.view_error")}>
                {translateRuntimeViewError(pouch.err, t)}
            </ErrorBar>
            {pouch.err.type === "Crash" && (
                <ErrorBar isWarning noBugReport>
                    <Link
                        onClick={() => {
                            void openExtension("crash-viewer");
                        }}
                    >
                        {t("main.view_crash_report")}
                    </Link>{" "}
                    {t("main.view_crash_report.desc")}
                </ErrorBar>
            )}
        </>
    );

    const $PouchItems = pouch?.val && !isTabView && (
        <div className={m("flex-1 overflow-y-auto scrollbar-thin")}>
            <div className={m("flex flex-wrap max-h-0 overflow-visible")}>
                {pouch.val.items.map((item, i) => (
                    <PouchItemSlotWithTooltip
                        item={item}
                        key={i}
                        list1Count={pouch.val.count}
                        isMasterSwordFullPower={isMasterSwordFullPower}
                        {...itemSlotProps}
                    />
                ))}
            </div>
        </div>
    );

    const $TabbedPouchItems = pouch?.val && tabNodes !== undefined && (
        <div
            id="-tabbed-pouch-scroll-"
            ref={tabbedScrollHandler}
            onScroll={(e) => {
                // sync scroll to the gdt panel
                const gdtPanel = document.getElementById("-tabbed-gdt-scroll-");
                if (gdtPanel) {
                    gdtPanel.scrollLeft = e.currentTarget.scrollLeft;
                }
            }}
            className={m("flex-1 overflow-auto scrollbar-thin")}
        >
            <div className={m("flex max-h-0 overflow-visible")}>
                {tabNodes.map((tab, i) => (
                    <div id={`-pouch-tab-${i}-`} key={i}>
                        <ItemTab
                            category={tab.category}
                            border
                            nodes={tab.items.map(({ slot, item }, i) => ({
                                slot,
                                element: (
                                    <PouchItemSlotWithTooltip
                                        item={item}
                                        key={i}
                                        list1Count={pouch.val.count}
                                        isMasterSwordFullPower={
                                            isMasterSwordFullPower
                                        }
                                        {...itemSlotProps}
                                    />
                                ),
                            }))}
                        />
                    </div>
                ))}
            </div>
        </div>
    );

    const { ref: overworldScrollHandler } = useSwappedWheelScrollDirection();
    const $OverworldItems = overworld?.val &&
        overworld.val.items.length > 0 && (
            <div
                className={m(
                    "pos-rel overflow-x-auto overflow-y-hidden scrollbar-thin",
                    c.overworldScroll,
                )}
                ref={overworldScrollHandler}
            >
                <div className={m("flex-row")}>
                    {overworld.val.items.map((item, i) => (
                        <OverworldItemSlotWithTooltip
                            item={item}
                            key={i}
                            isMasterSwordFullPower={isMasterSwordFullPower}
                            {...itemSlotProps}
                        />
                    ))}
                </div>
            </div>
        );

    return (
        <div className={m("pos-rel wh-100 overflow-hidden")}>
            {$Background}
            <div
                className={m("pos-abs all-sides-0 flex-col", c.splitContainer)}
            >
                <div className={m("flex-col flex-1", c.inventoryContainer)}>
                    {$Title}
                    <div className={m("flex-col", c.errors)}>
                        {$TabsWarning}
                        {$PouchError}
                        {pouchError && <ErrorBar>{pouchError}</ErrorBar>}
                        {overworldError && (
                            <ErrorBar>{overworldError}</ErrorBar>
                        )}
                        {gdtError && <ErrorBar>{gdtError}</ErrorBar>}
                    </div>
                    {$TabsMinimap}
                    {$PouchItems}
                    {$TabbedPouchItems}
                </div>
                {$OverworldItems}
            </div>
            <InventorySpinner show={showSpinner} />
        </div>
    );
};

export const PouchInventoryPanel = memo(PouchInventoryPanelImpl);
