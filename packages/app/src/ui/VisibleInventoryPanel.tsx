import { memo } from "react";
import { useDark } from "@pistonite/pure-react";
import {
    Text,
    makeStyles,
    Tooltip,
    mergeClasses,
} from "@fluentui/react-components";

import { ItemSlot, ItemTooltip } from "skybook-item-system";
import { useUITranslation } from "skybook-localization";

import { useInventoryListView } from "self::application/store";

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
    divider: {
        color: "#b7f1ff",
        width: "100%",
        maxHeight: "2px",
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

export const VisibleInventoryPanelImpl: React.FC = () => {
    const styles = useStyles();
    const dark = useDark();
    const { inventory, stale, loading } = useInventoryListView();
    const showSpinner = loading || stale || !inventory;
    const t = useUITranslation();
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
                                [ {inventory?.count ?? "???"} ]
                            </Text>
                        </Tooltip>
                    }
                    loading={showSpinner}
                ></InventoryTitle>
                {inventory !== undefined && (
                    <div className={styles.inventoryScroll}>
                        <div className={styles.inventoryList}>
                            {inventory.items.map((info, i) => (
                                <ItemTooltip info={info} key={i}>
                                    <ItemSlot info={info} />
                                </ItemTooltip>
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export const VisibleInventoryPanel = memo(VisibleInventoryPanelImpl);
