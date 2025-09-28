import { memo, useMemo } from "react";
import { makeStyles, Text, Tooltip, Button, Link } from "@fluentui/react-components";
import { Grid20Regular, Info20Regular } from "@fluentui/react-icons";
import { useSwappedWheelScrollDirection } from "@pistonite/shared-controls";

import {
    ItemTab,
    OverworldItemSlot,
    PouchItemSlot,
    getOverworldBackgroundUrl,
} from "@pistonite/skybook-itemsys";
import { translateRuntimeViewError, useUITranslation } from "skybook-localization";

import {
    useGdtInventoryView,
    useItemSlotPropsFromSettings,
    useOverworldItemsView,
    usePouchListView,
    useUIStore,
    openExtension,
} from "self::application";
import {
    getInBrokenSlotArray,
    getTabNodesFromPouch,
    getUndiscoveredTabMap,
    useStyleEngine,
} from "self::util";
import {
    InventoryTitle,
    InventoryTabButton,
    InventorySpinner,
    ErrorBar,
} from "self::ui/components";

import { ScreenIndicator } from "./screen_icon.tsx";
import {
    ArrowlessSmuggleIcon,
    HoldingIcon,
    MenuOverloadIcon,
    TrialModeIcon,
} from "./pouch_icon.tsx";

const useStyles = makeStyles({
    splitContainer: {
        zIndex: 1,
    },
    inventoryContainer: {
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
        backgroundColor: "#00000044",
        borderRadius: "4px",
    },

    toolbarDivider: {
        width: "4px",
        height: "20px",
        marginRight: "4px",
        borderRight: "1px solid #888",
    },

    overworldScroll: {
        maxHeight: "72px",
    },

    tabsMinimap: {
        borderRadius: "4px",
        backgroundColor: "#00000044",
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

    const backgroundName = useUIStore((state) => state.background);
    const hasGlider = !!gdt?.val?.info?.isParagliderObtained;
    const backgroundUrl = getOverworldBackgroundUrl(backgroundName);

    const isTabView = useUIStore((state) => state.isTabViewEnabled);
    const setTabView = useUIStore((state) => state.setIsTabViewEnabled);
    const isInBrokenSlotArray = useMemo(() => getInBrokenSlotArray(pouch?.val), [pouch]);
    const tabNodes = useMemo(() => {
        if (!isTabView) {
            return undefined;
        }
        return getTabNodesFromPouch(pouch?.val);
    }, [pouch, isTabView]);
    const undiscoveredTabs = useMemo(() => {
        if (!isTabView) {
            return {};
        }
        return getUndiscoveredTabMap(gdt?.val);
    }, [gdt, isTabView]);

    const isGameClosed = pouch?.err?.type === "Closed";

    const $Background = (
        <div
            className={m("pos-abs all-sides-0", c.background)}
            style={{
                backgroundImage: `url(${backgroundUrl})`,
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
            <div className={m("flex-row flex-1 flex-centera gap-4 pad-4", c.toolbar)}>
                <ScreenIndicator screen={pouch?.val?.screen} hasGlider={hasGlider} />
                <Tooltip
                    relationship="label"
                    content={t("main.visible_inventory.mcount.desc")}
                    withArrow
                    positioning="below"
                >
                    <Button
                        icon={
                            <Text font="monospace" size={400}>
                                {pouch?.val?.count ?? "--"}
                            </Text>
                        }
                        appearance={(pouch?.val?.count || 0) > 0 ? "secondary" : "transparent"}
                    />
                </Tooltip>
                <Tooltip
                    relationship="label"
                    content={t("main.tabbed_inventory.button_tooltip")}
                    withArrow
                    positioning="below"
                >
                    <Button
                        onClick={() => {
                            setTabView(!isTabView);
                        }}
                        icon={<Grid20Regular />}
                        appearance={isTabView ? "secondary" : "transparent"}
                    />
                </Tooltip>
                <div className={c.toolbarDivider} />
                {pouch?.val?.isHoldingInInventory && <HoldingIcon />}
                {pouch?.val?.isArrowlessSmuggle && <ArrowlessSmuggleIcon />}
                {pouch?.val?.isTrialMode && <TrialModeIcon />}
                {pouch?.val?.isMenuOverloaded && <MenuOverloadIcon />}
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
                            const panel = document.getElementById("-tabbed-pouch-scroll-");
                            const targetTab = document.getElementById(`-pouch-tab-${i}-`);
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

    // don't show "If you think this is a bug" if the message is "game is closed"
    const $PouchError = pouch?.err && (
        <>
            <ErrorBar title={t("main.visible_inventory.view_error")} noBugReport={isGameClosed}>
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
            <div className={m("flex flex-wrap max-h-0 overflow-visible pad-itemtop")}>
                {pouch.val.items.map((item, i) => (
                    <PouchItemSlot
                        tooltip
                        item={item}
                        key={i}
                        inBrokenSlot={isInBrokenSlotArray[i]}
                        isMasterSwordFullPower={isMasterSwordFullPower}
                        dragData={{
                            type: "pouch",
                            payload: item,
                            isMasterSwordFullPower,
                        }}
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
                            undiscovered={undiscoveredTabs[tab.category]}
                            border
                            nodes={tab.items.map(({ slot, item, itemIndex }, si) => ({
                                slot,
                                element: (
                                    <PouchItemSlot
                                        tooltip
                                        item={item}
                                        key={si}
                                        inBrokenSlot={
                                            itemIndex >= 0 && isInBrokenSlotArray[itemIndex]
                                        }
                                        isMasterSwordFullPower={isMasterSwordFullPower}
                                        dragData={{
                                            type: "pouch",
                                            payload: item,
                                            isMasterSwordFullPower,
                                            position: [i, si],
                                        }}
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
    const $OverworldItems = overworld?.val && overworld.val.items.length > 0 && (
        <div
            className={m(
                "pos-rel overflow-x-auto overflow-y-hidden scrollbar-thin pad-8",
                c.overworldScroll,
            )}
            ref={overworldScrollHandler}
        >
            <div className={m("flex-row")}>
                {overworld.val.items.map((item, i) => (
                    <OverworldItemSlot
                        tooltip
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
            <div className={m("pos-abs all-sides-0 flex-col", c.splitContainer)}>
                <div className={m("flex-col flex-1 pad-8", c.inventoryContainer)}>
                    {$Title}
                    <div className={m("flex-col gap-4")}>
                        {$TabsWarning}
                        {$PouchError}
                        {pouchError && <ErrorBar>{pouchError}</ErrorBar>}
                        {overworldError && <ErrorBar>{overworldError}</ErrorBar>}
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
