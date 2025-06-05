import { memo, useMemo } from "react";
import { Button, Tooltip, makeStyles } from "@fluentui/react-components";
import { Info20Regular } from "@fluentui/react-icons";
import { useDark } from "@pistonite/pure-react";
import { useSwappedWheelScrollDirection } from "@pistonite/shared-controls";

import { translateRuntimeViewError, useUITranslation } from "skybook-localization";
import { GdtItemSlotWithTooltip, ItemTab } from "skybook-item-system";

import {
    useGdtInventoryView,
    useItemSlotPropsFromSettings,
    usePouchListView,
} from "self::application/store";
import {
    useThemedSheikaBackgroundUrl,
    useUIStore,
    getTabNodesForGdt,
    useStyleEngine,
} from "self::ui/functions";
import {
    InventoryTitle,
    InventorySpinner,
    ErrorBar,
} from "self::ui/components";

const useStyles = makeStyles({
    main: {
        padding: "8px",
    },
    errors: {
        gap: "4px",
    },
});

const GdtInventoryPanelImpl: React.FC = () => {
    const m = useStyleEngine();
    const c = useStyles();
    const dark = useDark();
    const {
        data: pouch,
        loading: pouchLoading,
        error: pouchError,
    } = usePouchListView();
    const {
        data: gdt,
        stale,
        loading,
        error: gdtError,
    } = useGdtInventoryView();

    const showSpinner = loading || pouchLoading || stale || !gdt;
    const t = useUITranslation();

    const isTabView = useUIStore((state) => state.isTabViewEnabled);
    const tabNodes = useMemo(() => {
        if (!isTabView) {
            return undefined;
        }
        return getTabNodesForGdt(pouch?.val, gdt?.val);
    }, [pouch, gdt, isTabView]);

    const itemSlotProps = useItemSlotPropsFromSettings();

    const $Error = gdt?.err && (
        <ErrorBar title={t("main.gdt_inventory.view_error")}>
            {translateRuntimeViewError(gdt.err, t)}
        </ErrorBar>
    );

    const $ListView = gdt?.val && (
        <div className={m("flex-1 overflow-y-auto scrollbar-thin")}>
            <div className={m("flex flex-wrap max-h-0 overflow-visible")}>
                {gdt.val.items.map((item, i) => (
                    <GdtItemSlotWithTooltip
                        item={item}
                        key={i}
                        isMasterSwordFullPower={!!gdt.val.masterSword.isTrueForm}
                        {...itemSlotProps}
                    />
                ))}
            </div>
        </div>
    );

    const { ref: tabbedScrollHandler } = useSwappedWheelScrollDirection();
    const $TabView = isTabView &&
        gdt?.val &&
        tabNodes !== undefined && (
            <div
                id="-tabbed-gdt-scroll-"
                ref={tabbedScrollHandler}
                onScroll={(e) => {
                    // sync scroll to the pouch panel
                    const pouchPanel = document.getElementById(
                        "-tabbed-pouch-scroll-",
                    );
                    if (pouchPanel) {
                        pouchPanel.scrollLeft = e.currentTarget.scrollLeft;
                    }
                }}
                className={m("flex-1 overflow-auto scrollbar-thin")}
            >
                <div className={m("flex max-h-0 overflow-visible")}>
                    {tabNodes.map((tab, i) => (
                        <ItemTab
                            key={i}
                            category={tab.category}
                            nodes={tab.items.map(({ slot, item }, i) => ({
                                slot,
                                element: (
                                    <GdtItemSlotWithTooltip
                                        item={item}
                                        key={i}
                                        isMasterSwordFullPower={
                                            !!gdt.val.masterSword.isTrueForm
                                        }
                                        {...itemSlotProps}
                                    />
                                ),
                            }))}
                        />
                    ))}
                </div>
            </div>
        );

    return (
        <div className={m("pos-rel wh-100 overflow-hidden")}>
            <div
                style={{
                    backgroundImage: `url(${useThemedSheikaBackgroundUrl()})`,
                }}
                className={m("pos-abs all-sides-0 flex-col", c.main)}
            >
                <InventoryTitle
                    title={t("main.gdt_inventory.title")}
                    dark={dark}
                >
                    <Tooltip
                        relationship="description"
                        content={t("main.gdt_inventory.desc")}
                        withArrow
                        positioning="below"
                    >
                        <Button
                            icon={<Info20Regular />}
                            appearance="transparent"
                        />
                    </Tooltip>
                    <InventorySpinner show={showSpinner} />
                </InventoryTitle>
                <div className={m("flex-col", c.errors)}>
                    {$Error}
                    {pouchError && <ErrorBar>{pouchError}</ErrorBar>}
                    {gdtError && <ErrorBar>{gdtError}</ErrorBar>}
                </div>
                {$TabView || $ListView}
            </div>
        </div>
    );
};

export const GdtInventoryPanel = memo(GdtInventoryPanelImpl);
