import { memo, useMemo } from "react";
import {
    Button,
    Menu,
    MenuDivider,
    MenuGroup,
    MenuGroupHeader,
    MenuItem,
    MenuList,
    MenuPopover,
    MenuTrigger,
    Tooltip,
    useRestoreFocusTarget,
} from "@fluentui/react-components";
import {
    Checkmark20Regular,
    PuzzlePiece20Regular,
    WindowDevTools20Regular,
} from "@fluentui/react-icons";

import { useUITranslation } from "skybook-localization";

import {
    getOpenModeForExtension,
    useCurrentSecondaryExtensionId,
    useExtensionName,
    useExtensionStore,
    openExtensionPopup,
    useUIStore,
} from "self::application";

const RECENT_LIMIT = 5;

const ExtensionMenuImpl: React.FC = () => {
    const restoreFocusTargetAttribute = useRestoreFocusTarget();
    const t = useUITranslation();
    const setOpenedDialog = useUIStore((state) => state.setOpenedDialog);

    const pinnedIds = useExtensionStore((state) => state.pinnedIds);
    const recentIds = useExtensionStore((state) => state.recentIds);
    const recentFiltered = useMemo(() => {
        const ids: string[] = [];
        for (const id of recentIds) {
            if (ids.length >= RECENT_LIMIT) {
                break;
            }
            if (pinnedIds.includes(id)) {
                continue;
            }
            if (ids.includes(id)) {
                continue;
            }
            ids.push(id);
        }
        return ids;
    }, [pinnedIds, recentIds]);
    return (
        <Menu hasIcons>
            <MenuTrigger disableButtonEnhancement>
                <Tooltip
                    relationship="label"
                    content={t("menu.header.extensions")}
                    positioning="below"
                >
                    <Button
                        appearance="subtle"
                        icon={<PuzzlePiece20Regular />}
                    />
                </Tooltip>
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    {pinnedIds.length > 0 && (
                        <MenuGroup>
                            <MenuGroupHeader>
                                {t("menu.header.pinned")}
                            </MenuGroupHeader>
                            {pinnedIds.map((id) => (
                                <ExtensionMenuItem key={id} id={id} />
                            ))}
                        </MenuGroup>
                    )}
                    {recentFiltered.length > 0 && (
                        <MenuGroup>
                            <MenuGroupHeader>
                                {t("menu.header.recent")}
                            </MenuGroupHeader>
                            {recentFiltered.map((id) => (
                                <ExtensionMenuItem key={id} id={id} />
                            ))}
                        </MenuGroup>
                    )}
                    {(pinnedIds.length > 0 || recentFiltered.length > 0) && (
                        <MenuDivider />
                    )}
                    <MenuItem
                        icon={<WindowDevTools20Regular />}
                        {...restoreFocusTargetAttribute}
                        onClick={() => {
                            setOpenedDialog("extension-launch");
                        }}
                    >
                        {t("menu.open_configure_extension")}
                    </MenuItem>
                    <MenuItem
                        icon={<PuzzlePiece20Regular />}
                        {...restoreFocusTargetAttribute}
                        onClick={() => {
                            setOpenedDialog("custom-extension");
                        }}
                    >
                        {t("menu.custom_extensions")}
                    </MenuItem>
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};

export const ExtensionsMenu = memo(ExtensionMenuImpl);

type ExtensionMenuItemProps = {
    /** Id of the extension to open */
    id: string;
};
const ExtensionMenuItem: React.FC<ExtensionMenuItemProps> = ({ id }) => {
    const name = useExtensionName(id);
    const openExtension = useExtensionStore((state) => state.open);
    const updateRecency = useExtensionStore((state) => state.updateRecency);
    const primaryId = useExtensionStore((state) => state.currentPrimary);
    const secondaryId = useCurrentSecondaryExtensionId();
    return (
        <MenuItem
            icon={
                id === primaryId || id === secondaryId ? (
                    <Checkmark20Regular />
                ) : undefined
            }
            onClick={() => {
                const mode = getOpenModeForExtension(id);
                updateRecency(id);
                if (mode === "popout") {
                    // this covers the custom case
                    void openExtensionPopup(id);
                } else {
                    openExtension(id, mode);
                }
            }}
        >
            {name}
        </MenuItem>
    );
};
