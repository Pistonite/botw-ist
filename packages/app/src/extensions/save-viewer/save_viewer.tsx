import { useState } from "react";
import { makeStyles, Tab, TabList } from "@fluentui/react-components";
import { ResizeLayout } from "@pistonite/shared-controls";

import {
    translateRuntimeViewError,
    useUITranslation,
} from "skybook-localization";
import { GdtItemSlotWithTooltip } from "skybook-item-system";

import { isLessProductive } from "self::pure-contrib";
import { ErrorBar } from "self::ui/components";
import { useStyleEngine, useThemedSheikaBackgroundUrl } from "self::util";

import type { SaveViewerStore } from "./store.ts";

export type SaveViewerProps = {
    useStore: SaveViewerStore;
};

const useStyles = makeStyles({
    saveTab: {
        "& span": {
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
        },
    },
});

export const SaveViewer: React.FC<SaveViewerProps> = ({ useStore }) => {
    const saveNames = useStore((state) => state.saveNames);
    const selectedSave = useStore((state) => state.selectedSave);
    const data = useStore((state) => state.displayedData);
    const cheap = useStore((state) => state.cheap);
    const disableAnimation = useStore((state) => state.disableAnimation);
    const setSelectedSave = useStore((state) => state.setSelectedSave);
    const m = useStyleEngine();
    const t = useUITranslation();
    const c = useStyles();
    const [sidebarPercent, setSidebarPercent] = useState(20);
    const $Error = data?.err && (
        <ErrorBar title={t("main.save_inventory.view_error")}>
            {translateRuntimeViewError(data.err, t)}
        </ErrorBar>
    );

    const $ListView = data?.val && (
        <div className={m("flex-1 h-100 overflow-y-auto scrollbar-thin")}>
            <div
                className={m(
                    "flex flex-wrap max-h-0 overflow-visible pad-itemtop",
                )}
            >
                {data.val.items.map((item, i) => (
                    <GdtItemSlotWithTooltip
                        item={item}
                        key={i}
                        isMasterSwordFullPower={
                            !!data.val.masterSword.isTrueForm
                        }
                        cheap={cheap}
                        disableAnimation={disableAnimation}
                    />
                ))}
            </div>
        </div>
    );
    return (
        <ResizeLayout
            className={m("wh-100")}
            minWidth={150}
            valuePercent={sidebarPercent}
            setValuePercent={setSidebarPercent}
            touch={isLessProductive}
        >
            <div className={m("min-w-0")}>
                <TabList
                    vertical
                    selectedValue={selectedSave || ""}
                    onTabSelect={(_, { value }) => {
                        if (saveNames.includes(value as string)) {
                            setSelectedSave(value as string);
                            return;
                        }
                        setSelectedSave(undefined);
                    }}
                >
                    <Tab className={m("overflow-hidden", c.saveTab)} value="">
                        {t("save_viewer.manual_save")}
                    </Tab>
                    {saveNames.map((name) => (
                        <Tab
                            key={name}
                            className={m("overflow-hidden", c.saveTab)}
                            value={name}
                        >
                            {name}
                        </Tab>
                    ))}
                </TabList>
            </div>
            <div
                className={m("min-w-0 h-100 pad-8")}
                style={{ background: `url(${useThemedSheikaBackgroundUrl()})` }}
            >
                {$Error}
                {$ListView}
            </div>
        </ResizeLayout>
    );
};
