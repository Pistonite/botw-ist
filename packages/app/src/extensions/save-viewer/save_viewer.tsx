import { useState } from "react";
import { Tab, TabList } from "@fluentui/react-components";
import { ResizeLayout } from "@pistonite/celera";

import { translateRuntimeViewError, useUITranslation } from "skybook-localization";
import { GdtItemSlot } from "@pistonite/skybook-itemsys";

import { ErrorBar } from "#ui/components";
import { useStyleEngine, useThemedSheikaBackgroundUrl } from "#util";

import type { SaveViewerStore } from "./store.ts";

export type SaveViewerProps = {
    useStore: SaveViewerStore;
};

const useStyles = useStyleEngine.extend({
    "save-tab": {
        "& span": {
            textOverflow: "ellipsis",
            whiteSpace: "nowrap",
        },
    },
});

export const createSaveViewerComponent = (useStore: SaveViewerStore): React.FC => {
    const SaveViewer: React.FC = () => {
        const saveNames = useStore((state) => state.saveNames);
        const displayedSave = useStore((state) => state.displayedSave);
        const data = useStore((state) => state.displayedData);
        const cheap = useStore((state) => state.cheap);
        const disableAnimation = useStore((state) => state.disableAnimation);
        const setSelectedSave = useStore((state) => state.setSelectedSave);
        const t = useUITranslation();
        const m = useStyles();
        const [sidebarPercent, setSidebarPercent] = useState(20);

        const $Error = data?.err && (
            <ErrorBar title={t("main.save_inventory.view_error")}>
                {translateRuntimeViewError(data.err, t)}
            </ErrorBar>
        );

        const $ListView = data?.val && (
            <div className={m("flex-1 h-100 overflow-y-auto scrollbar-thin")}>
                <div className={m("flex flex-wrap max-h-0 overflow-visible pad-itemtop")}>
                    {data.val.items.map((item, i) => (
                        <GdtItemSlot
                            tooltip
                            item={item}
                            key={i}
                            isMasterSwordFullPower={!!data.val.masterSword.isTrueForm}
                            cheap={cheap}
                            disableAnimation={disableAnimation}
                        />
                    ))}
                </div>
            </div>
        );
        return (
            <ResizeLayout
                className={m("wh-100 overflow-auto")}
                minWidth={30}
                valuePercent={sidebarPercent}
                setValuePercent={setSidebarPercent}
            >
                <div className={m("min-w-0")}>
                    <TabList
                        vertical
                        selectedValue={displayedSave || ""}
                        onTabSelect={(_, { value }) => {
                            if (saveNames.includes(value as string)) {
                                setSelectedSave(value as string);
                                return;
                            }
                            setSelectedSave(undefined);
                        }}
                    >
                        <Tab className={m("overflow-hidden c-save-tab")} value="">
                            {t("save_viewer.manual_save")}
                        </Tab>
                        {saveNames.map((name) => (
                            <Tab
                                key={name}
                                className={m("overflow-hidden c-save-tab")}
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
    return SaveViewer;
};
