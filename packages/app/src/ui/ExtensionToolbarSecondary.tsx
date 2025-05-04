import { memo } from "react";

import {
    getSecondaryExtensionIdsForDropdown,
    useCurrentSecondaryExtensionId,
    useExtensionStore,
} from "self::application/store";
import { openExtensionPopup } from "self::application/extension";

import { ExtensionToolbar } from "./components/ExtensionToolbar.tsx";

const ExtensionToolbarSecondaryConnected: React.FC = () => {
    const currentSecondaryId = useCurrentSecondaryExtensionId();
    const secondaryIds = useExtensionStore(getSecondaryExtensionIdsForDropdown);
    const openExtension = useExtensionStore((state) => state.open);
    const updateRecency = useExtensionStore((state) => state.updateRecency);
    const closeSecondary = useExtensionStore((state) => state.closeSecondary);

    return (
        <ExtensionToolbar
            id={currentSecondaryId}
            allIds={secondaryIds}
            onClickPopout={() => {
                updateRecency(currentSecondaryId);
                openExtensionPopup(currentSecondaryId);
                closeSecondary();
            }}
            onSelect={(id) => {
                updateRecency(id);
                openExtension(id, "secondary");
            }}
            onClickClose={closeSecondary}
        />
    );
};

export const ExtensionToolbarSecondary = memo(
    ExtensionToolbarSecondaryConnected,
);
