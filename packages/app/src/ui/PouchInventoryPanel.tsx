import { memo } from "react";
import { useDark } from "@pistonite/pure-react";
import {
    Text,
    makeStyles,
    Tooltip,
    mergeClasses,
} from "@fluentui/react-components";

import { PouchItemSlotWithTooltip } from "skybook-item-system";
import { useUITranslation } from "skybook-localization";

import { useGdtInventoryView, useItemSlotPropsFromSettings, usePouchListView } from "self::application/store";

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
    main: {
        position: "absolute",
        padding: "8px",
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
        display: "flex",
        flexDirection: "column",
        zIndex: 1,
    },
    mainBgDark: {
        backgroundColor: "#00000066",
    },
    mainBgLight: {
        backgroundColor: "#ffffff44",
    },
    background: {
        scale: 1.3,
        position: "absolute",
        top: 0,
        left: 0,
        right: 0,
        bottom: 0,
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
});

export const PouchInventoryPanelImpl: React.FC = () => {
    const styles = useStyles();
    const dark = useDark();
    const { inventory: pouch, stale, loading } = usePouchListView();
    const { inventory: gdt, } = useGdtInventoryView();

    const showSpinner = loading || stale || !pouch;
    const t = useUITranslation();

    const itemSlotProps = useItemSlotPropsFromSettings();

    return (
        <div className={styles.container}>
            <div
                className={styles.background}
                style={{
                    backgroundImage: `url(${getOverworldBackgroundUrl()})`,
                }}
            />
            <div
                className={mergeClasses(
                    styles.main,
                    dark ? styles.mainBgDark : styles.mainBgLight,
                )}
            >
                <InventoryTitle
                    title={t("main.visible_inventory.title")}
                    description={t("main.visible_inventory.desc")}
                    supertitle={
                        <Tooltip
                            relationship="label"
                            content={
                                <>
                                    {t("main.mcount_desc")} (<Code>mCount</Code>
                                    )
                                </>
                            }
                        >
                            <Text font="numeric">
                                [ {pouch?.count ?? "???"} ]
                            </Text>
                        </Tooltip>
                    }
                    loading={showSpinner}
                ></InventoryTitle>
                {pouch !== undefined && (
                    <div className={styles.inventoryScroll}>
                        <div className={styles.inventoryList}>
                            {pouch.items.map((item, i) => (
                                <PouchItemSlotWithTooltip
                                    item={item}
                                    key={i}
                                    list1Count={pouch.count}
                                    isMasterSwordFullPower={!!(gdt?.masterSword?.isTrueForm)}
                                    {...itemSlotProps}
                                />
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export const PouchInventoryPanel = memo(PouchInventoryPanelImpl);
