import { memo, useMemo } from "react";
import {
    Text,
    makeStyles,
    Tooltip,
    ToggleButton,
    MessageBar,
    MessageBarBody,
    MessageBarTitle,
    Button,
} from "@fluentui/react-components";
import { Grid20Regular, Info20Regular } from "@fluentui/react-icons";
import { useSwappedWheelScrollDirection } from "@pistonite/shared-controls";

import {
    ItemTab,
    OverworldItemSlotWithTooltip,
    PouchItemSlotWithTooltip,
} from "skybook-item-system";
import { useUITranslation } from "skybook-localization";

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
    BugReportText,
} from "self::ui/components";

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
});

export const PouchInventoryPanelImpl: React.FC = () => {
    const m = useStyleEngine();
    const c = useStyles();
    const t = useUITranslation();

    const { data: pouch, stale, loading } = usePouchListView();
    const {
        data: overworld,
        stale: overworldStale,
        loading: overworldLoading,
    } = useOverworldItemsView();
    const { data: gdt } = useGdtInventoryView();

    const showSpinner =
        loading || stale || !pouch || overworldLoading || overworldStale;

    const itemSlotProps = useItemSlotPropsFromSettings();
    const isMasterSwordFullPower = !!gdt?.masterSword?.isTrueForm;

    const isTabView = useUIStore((state) => state.isTabViewEnabled);
    const setTabView = useUIStore((state) => state.setIsTabViewEnabled);
    const tabNodes = useMemo(() => {
        if (!isTabView) {
            return undefined;
        }
        return getTabNodesFromPouch(pouch);
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
                <InventorySpinner show={showSpinner} />
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
        <MessageBar intent="error">
            <MessageBarBody>
                <MessageBarTitle>
                    {t("main.tabbed_inventory.bad_msgbar.title")}
                </MessageBarTitle>
                <Text>
                    {t("main.tabbed_inventory.bad_msgbar.body")}{" "}
                    <BugReportText />
                </Text>
            </MessageBarBody>
        </MessageBar>
    );

    const $PouchItems = pouch !== undefined && !isTabView && (
        <div className={m("flex-1 overflow-y-auto scrollbar-thin")}>
            <div className={m("flex flex-wrap max-h-0 overflow-visible")}>
                {pouch.items.map((item, i) => (
                    <PouchItemSlotWithTooltip
                        item={item}
                        key={i}
                        list1Count={pouch.count}
                        isMasterSwordFullPower={isMasterSwordFullPower}
                        {...itemSlotProps}
                    />
                ))}
            </div>
        </div>
    );

    const $TabbedPouchItems = pouch !== undefined && tabNodes !== undefined && (
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
                                        list1Count={pouch.count}
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
    const $OverworldItems = overworld !== undefined &&
        overworld.items.length > 0 && (
            <div
                className={m(
                    "pos-rel overflow-x-auto overflow-y-hidden scrollbar-thin",
                    c.overworldScroll,
                )}
                ref={overworldScrollHandler}
            >
                <div className={m("flex-row")}>
                    {overworld.items.map((item, i) => (
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
                    {$TabsWarning}
                    {$TabsMinimap}
                    {$PouchItems}
                    {$TabbedPouchItems}
                </div>
                {$OverworldItems}
            </div>
        </div>
    );
};

export const PouchInventoryPanel = memo(PouchInventoryPanelImpl);
