import { memo } from "react";

import {
    useAllNonPopoutExtensionIds,
    useCurrentPrimaryExtensionId,
    useExtensionStore,
    usePrimaryExtensionIds,
} from "self::application/store";
import { openExtensionPopup } from "self::application/extension";
import { isLessProductive } from "self::pure-contrib";

import { ExtensionToolbar } from "./components/ExtensionToolbar.tsx";

const ExtensionToolbarPrimaryConnected: React.FC = () => {
    const currentPrimaryId = useCurrentPrimaryExtensionId();
    const primaryIds = usePrimaryExtensionIds();
    const openExtension = useExtensionStore((state) => state.open);
    const closePrimary = useExtensionStore((state) => state.closePrimary);

    return (
        <ExtensionToolbar
            id={currentPrimaryId}
            allIds={primaryIds}
            onClickPopout={() => {
                openExtensionPopup(currentPrimaryId);
                closePrimary();
            }}
            onSelect={(id) => {
                openExtension(id, "primary");
            }}
            onClickClose={closePrimary}
        ></ExtensionToolbar>
    );
};

const ExtensionToolbarPrimaryMemo = memo(ExtensionToolbarPrimaryConnected);

/** Titlebar for mobile platform with simplified controls */
const ExtensionToolbarPrimaryMobileConnected: React.FC = () => {
    const allIds = useAllNonPopoutExtensionIds();
    const currentId = useCurrentPrimaryExtensionId();
    const openExtension = useExtensionStore((state) => state.open);

    return (
        <ExtensionToolbar
            id={currentId}
            allIds={allIds}
            onSelect={(id) => {
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
