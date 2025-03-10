import { memo } from "react";

import {
    useCurrentSecondaryExtensionId,
    useExtensionStore,
    useSecondaryExtensionIds,
} from "application/store";
import { openExtensionPopup } from "application/extension";

import { ExtensionToolbar } from "./components/ExtensionToolbar.tsx";

const ExtensionToolbarSecondaryConnected: React.FC = () => {
    const currentSecondaryId = useCurrentSecondaryExtensionId();
    const secondaryIds = useSecondaryExtensionIds();
    const openExtension = useExtensionStore((state) => state.open);
    const closeSecondary = useExtensionStore((state) => state.closeSecondary);

    return (
        <ExtensionToolbar
            id={currentSecondaryId}
            allIds={secondaryIds}
            onClickPopout={() => {
                openExtensionPopup(currentSecondaryId);
                closeSecondary();
            }}
            onSelect={(id) => {
                openExtension(id, "secondary");
            }}
            onClickClose={closeSecondary}
        />
    );
};

export const ExtensionToolbarSecondary = memo(
    ExtensionToolbarSecondaryConnected,
);
