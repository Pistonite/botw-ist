import { memo } from "react";

import {
    BuiltinExtensionIds,
    getPrimaryExtensionIdsForDropdown,
    useExtensionStore,
} from "self::application/store";
import { openExtensionPopup } from "self::application/extension";
import { isLessProductive } from "self::pure-contrib";
import { ExtensionToolbar } from "self::ui/components";

const ExtensionToolbarPrimaryConnected: React.FC = () => {
    const currentPrimaryId = useExtensionStore((state) => state.currentPrimary);
    const primaryIds = useExtensionStore(getPrimaryExtensionIdsForDropdown);
    const openExtension = useExtensionStore((state) => state.open);
    const updateRecency = useExtensionStore((state) => state.updateRecency);
    const closePrimary = useExtensionStore((state) => state.closePrimary);

    return (
        <ExtensionToolbar
            id={currentPrimaryId}
            allIds={primaryIds}
            onClickPopout={() => {
                updateRecency(currentPrimaryId);
                void openExtensionPopup(currentPrimaryId);
                closePrimary();
            }}
            onSelect={(id) => {
                updateRecency(id);
                openExtension(id, "primary");
            }}
            onClickClose={closePrimary}
        ></ExtensionToolbar>
    );
};

const ExtensionToolbarPrimaryMemo = memo(ExtensionToolbarPrimaryConnected);

/** Titlebar for mobile platform with simplified controls */
const ExtensionToolbarPrimaryMobileConnected: React.FC = () => {
    const currentId = useExtensionStore((state) => state.currentPrimary);
    const updateRecency = useExtensionStore((state) => state.updateRecency);
    const openExtension = useExtensionStore((state) => state.open);

    return (
        <ExtensionToolbar
            id={currentId}
            allIds={BuiltinExtensionIds as unknown as string[]}
            onSelect={(id) => {
                updateRecency(id);
                openExtension(id, "primary");
            }}
        />
    );
};

const ExtensionToolbarPrimaryMobileMemo = memo(
    ExtensionToolbarPrimaryMobileConnected,
);

export const ExtensionToolbarPrimary = () => {
    if (isLessProductive) {
        return <ExtensionToolbarPrimaryMobileMemo />;
    }
    return <ExtensionToolbarPrimaryMemo />;
};
