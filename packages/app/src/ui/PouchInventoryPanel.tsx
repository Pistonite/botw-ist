import { memo, useRef } from "react";
import {
    Text,
    makeStyles,
    Tooltip,
    mergeClasses,
} from "@fluentui/react-components";

import {
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

import { InventoryTitle } from "./components/InventoryTitle.tsx";
import { Code } from "./components/Code.tsx";
import { getOverworldBackgroundUrl } from "./asset.ts";

const useStyles = makeStyles({
    container: {
        position: "relative",
        width: "100%",
        height: "100%",
        overflow: "hidden",
    },
    absPos: {
        position: "absolute",
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
    },
    splitContainer: {
        display: "flex",
        flexDirection: "column",
        zIndex: 1,
    },
    inventoryContainer: {
        padding: "8px",
        display: "flex",
        flexDirection: "column",
        flex: 1,
        backgroundColor: "#00000066",
    },
    background: {
        // This is to hide the UI elements like health bar, etc
        scale: 1.3,
        backgroundPosition: "center",
        backgroundRepeat: "no-repeat",
        backgroundSize: "cover",
    },

    inventoryScroll: {
        flex: 1,
        overflowY: "auto",
    },

    inventoryList: {
        display: "flex",
        flexWrap: "wrap",
        maxHeight: 0,
        overflow: "visible",
    },

    overworldScroll: {
        position: "relative",
        padding: "8px",
        maxHeight: "72px",
        overflowX: "auto",
        overflowY: "hidden",
        scrollbarWidth: "thin",
        scrollBehavior: "smooth",
    },

    overworldList: {
        display: "flex",
        flexDirection: "row",
    },
});

export const PouchInventoryPanelImpl: React.FC = () => {
    const styles = useStyles();
    const { data: pouch, stale, loading } = usePouchListView();
    const {
        data: overworld,
        stale: overworldStale,
        loading: overworldLoading,
    } = useOverworldItemsView();
    const { data: gdt } = useGdtInventoryView();

    const showSpinner =
        loading || stale || !pouch || overworldLoading || overworldStale;
    const t = useUITranslation();

    const itemSlotProps = useItemSlotPropsFromSettings();
    const isMasterSwordFullPower = !!gdt?.masterSword?.isTrueForm;

    const $Background = (
        <div
            className={mergeClasses(styles.background, styles.absPos)}
            style={{
                backgroundImage: `url(${getOverworldBackgroundUrl()})`,
            }}
        />
    );

    const $Title = (
        <InventoryTitle
            title={t("main.visible_inventory.title")}
            description={t("main.visible_inventory.desc")}
            supertitle={
                <Tooltip
                    relationship="label"
                    content={
                        <>
                            {t("main.mcount_desc")} (<Code>mCount</Code>)
                        </>
                    }
                >
                    <Text font="numeric">[ {pouch?.count ?? "???"} ]</Text>
                </Tooltip>
            }
            loading={showSpinner}
            dark
        />
    );

    const $PouchItems = pouch !== undefined && (
        <div className={styles.inventoryScroll}>
            <div className={styles.inventoryList}>
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

    const overworldScrollHandler = useHorizontalScrollWheelHandler();
    const $OverworldItems = overworld !== undefined &&
        overworld.items.length > 0 && (
            <div
                className={styles.overworldScroll}
                onWheel={overworldScrollHandler}
            >
                <div className={styles.overworldList}>
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
        <div className={styles.container}>
            {$Background}
            <div className={mergeClasses(styles.absPos, styles.splitContainer)}>
                <div className={styles.inventoryContainer}>
                    {$Title}
                    {$PouchItems}
                </div>
                {$OverworldItems}
            </div>
        </div>
    );
};

export const PouchInventoryPanel = memo(PouchInventoryPanelImpl);

/**
 * A trick to scroll horizontally on mouch wheel events.
 * Requires scroll-behavior: smooth on target
 */
const useHorizontalScrollWheelHandler = () => {
    const scrollLeftTarget = useRef(0);
    const handler = (e: React.WheelEvent<HTMLDivElement>) => {
        if (!e.deltaY) {
            return;
        }
        const max = e.currentTarget.scrollWidth - e.currentTarget.clientWidth;
        if (max <= 0) {
            return;
        }
        const target = Math.max(
            Math.min(scrollLeftTarget.current + e.deltaY, max),
            0,
        );
        scrollLeftTarget.current = target;
        e.currentTarget.scrollLeft = target;
    };
    return handler;
};
