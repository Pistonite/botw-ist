import { memo } from "react";
import { makeStyles } from "@fluentui/react-components";

import { useUITranslation } from "skybook-localization";
import { GdtItemSlotWithTooltip } from "skybook-item-system";

import {
    useGdtInventoryView,
    useItemSlotPropsFromSettings,
} from "self::application/store";

import { useThemedSheikaBackgroundUrl } from "./asset.ts";
import { InventoryTitle } from "./components/InventoryTitle.tsx";

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

const GdtInventoryPanelImpl: React.FC = () => {
    const styles = useStyles();
    const { inventory: gdt, stale, loading } = useGdtInventoryView();

    const showSpinner = loading || stale || !gdt;
    const t = useUITranslation();

    const itemSlotProps = useItemSlotPropsFromSettings();

    return (
        <div className={styles.container}>
            <div
                style={{
                    backgroundImage: `url(${useThemedSheikaBackgroundUrl()})`,
                }}
                className={styles.main}
            >
                <InventoryTitle
                    title={t("main.gdt_inventory.title")}
                    description={t("main.gdt_inventory.desc")}
                    loading={showSpinner}
                ></InventoryTitle>
                {gdt !== undefined && (
                    <div className={styles.inventoryScroll}>
                        <div className={styles.inventoryList}>
                            {gdt.items.map((item, i) => (
                                <GdtItemSlotWithTooltip
                                    item={item}
                                    key={i}
                                    isMasterSwordFullPower={
                                        !!gdt.masterSword.isTrueForm
                                    }
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

export const GdtInventoryPanel = memo(GdtInventoryPanelImpl);
