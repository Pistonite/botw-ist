import { memo, useMemo } from "react";
import { Button, Tooltip } from "@fluentui/react-components";
import { Info20Regular } from "@fluentui/react-icons";
import { useDark } from "@pistonite/pure-react";
import { useSwappedWheelScrollDirection } from "@pistonite/shared-controls";

import { translateRuntimeViewError, useUITranslation } from "skybook-localization";
import { GdtItemSlot, ItemTab } from "@pistonite/skybook-itemsys";

import {
    useGdtInventoryView,
    useItemSlotPropsFromSettings,
    usePouchListView,
    useUIStore,
} from "self::application";
import { useThemedSheikaBackgroundUrl, getTabNodesForGdt, useStyleEngine } from "self::util";
import { InventoryTitle, InventorySpinner, ErrorBar } from "self::ui/components";

const GdtInventoryPanelImpl: React.FC = () => {
    const m = useStyleEngine();
    const dark = useDark();
    const { data: pouch, loading: pouchLoading, error: pouchError } = usePouchListView();
    const { data: gdt, loading, error: gdtError } = useGdtInventoryView();

    const showSpinner = loading || pouchLoading || !gdt;
    const t = useUITranslation();

    const isTabView = useUIStore((state) => state.isTabViewEnabled);
    const [tabNodes, missingItemsInTabbedGdt] = useMemo(() => {
        if (!isTabView) {
            return [undefined, false];
        }
        const tabNodes = getTabNodesForGdt(pouch?.val, gdt?.val);
        const missingItemsInTabbedGdt =
            (pouch?.val?.items?.length || 0) < (gdt?.val?.items?.length || 0);
        return [tabNodes, missingItemsInTabbedGdt];
    }, [pouch, gdt, isTabView]);

    const itemSlotProps = useItemSlotPropsFromSettings();

    const noShowMaybeBugMsg = gdt?.err?.type === "Closed";
    const $Error = gdt?.err && (
        <ErrorBar title={t("main.gdt_inventory.view_error")} noBugReport={noShowMaybeBugMsg}>
            {translateRuntimeViewError(gdt.err, t)}
        </ErrorBar>
    );
    const $MissingItemInTabbedError = missingItemsInTabbedGdt && (
        <ErrorBar isWarning noBugReport>
            {t("main.gdt_inventory.missing_items_tabbed")}
        </ErrorBar>
    );

    const $ListView = gdt?.val && (
        <div className={m("flex-1 overflow-y-auto scrollbar-thin")}>
            <div className={m("flex flex-wrap max-h-0 overflow-visible pad-itemtop")}>
                {gdt.val.items.map((item, i) => (
                    <GdtItemSlot
                        tooltip
                        item={item}
                        key={i}
                        isMasterSwordFullPower={!!gdt.val.masterSword.isTrueForm}
                        {...itemSlotProps}
                        dragData={{
                            type: "gdt",
                            payload: item,
                            isMasterSwordFullPower: true
                        }}
                    />
                ))}
            </div>
        </div>
    );

    const { ref: tabbedScrollHandler } = useSwappedWheelScrollDirection();
    const $TabView = isTabView && gdt?.val && tabNodes !== undefined && (
        <div
            id="-tabbed-gdt-scroll-"
            ref={tabbedScrollHandler}
            onScroll={(e) => {
                // sync scroll to the pouch panel
                const pouchPanel = document.getElementById("-tabbed-pouch-scroll-");
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
                                <GdtItemSlot
                                    tooltip
                                    item={item}
                                    key={i}
                                    isMasterSwordFullPower={!!gdt.val.masterSword.isTrueForm}
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
                className={m("pos-abs all-sides-0 flex-col pad-8")}
            >
                <InventoryTitle title={t("main.gdt_inventory.title")} dark={dark}>
                    <Tooltip
                        relationship="description"
                        content={t("main.gdt_inventory.desc")}
                        withArrow
                        positioning="below"
                    >
                        <Button icon={<Info20Regular />} appearance="transparent" />
                    </Tooltip>
                </InventoryTitle>
                <div className={m("flex-col gap-4")}>
                    {$Error}
                    {$MissingItemInTabbedError}
                    {pouchError && <ErrorBar>{pouchError}</ErrorBar>}
                    {gdtError && <ErrorBar>{gdtError}</ErrorBar>}
                </div>
                {$TabView || $ListView}
            </div>
            <InventorySpinner show={showSpinner} />
        </div>
    );
};

export const GdtInventoryPanel = memo(GdtInventoryPanelImpl);
