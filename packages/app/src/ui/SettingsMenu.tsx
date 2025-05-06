import { memo } from "react";
import {
    Button,
    Menu,
    MenuGroup,
    MenuGroupHeader,
    MenuList,
    MenuPopover,
    MenuTrigger,
    Tooltip,
} from "@fluentui/react-components";
import {
    FilmstripPlay20Regular,
    ImageSparkle20Regular,
    WrenchSettings20Regular,
} from "@fluentui/react-icons";
import {
    DarkToggle,
    LanguagePicker,
    MenuSwitch,
} from "@pistonite/shared-controls";

import { useUITranslation } from "skybook-localization";

import { useApplicationStore } from "self::application/store";
import { isLessProductive } from "../pure-contrib/platform";

/** Settings menu in the header */
const SettingsMenuImpl: React.FC = () => {
    const enableHighRes = useApplicationStore(
        (state) => state.enableHighQualityIcons,
    );
    const setEnableHighRes = useApplicationStore(
        (state) => state.setEnableHighQualityIcons,
    );
    const enableAnimations = useApplicationStore(
        (state) => state.enableAnimations,
    );
    const setEnableAnimations = useApplicationStore(
        (state) => state.setEnableAnimations,
    );
    const t = useUITranslation();

    const tooltipPosition = isLessProductive ? "below" : "after";
    return (
        <Menu>
            <MenuTrigger disableButtonEnhancement>
                <Tooltip
                    relationship="label"
                    content={t("menu.header.preference")}
                    positioning="below"
                >
                    <Button
                        appearance="subtle"
                        icon={<WrenchSettings20Regular />}
                    />
                </Tooltip>
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    <MenuGroup>
                        <MenuGroupHeader>
                            {t("menu.header.preference")}
                        </MenuGroupHeader>
                        <LanguagePicker as="submenu" />
                        <DarkToggle as="submenu" />
                        <Tooltip
                            relationship="description"
                            content={t("menu.high_res_icons.desc")}
                            positioning={tooltipPosition}
                        >
                            <MenuSwitch
                                icon={<ImageSparkle20Regular />}
                                checked={enableHighRes}
                                onChange={(_, { checked }) => {
                                    setEnableHighRes(checked);
                                }}
                            >
                                {t("menu.high_res_icons")}
                            </MenuSwitch>
                        </Tooltip>
                        <Tooltip
                            relationship="description"
                            content={t("menu.enable_animations.desc")}
                            positioning={tooltipPosition}
                        >
                            <MenuSwitch
                                icon={<FilmstripPlay20Regular />}
                                disabled={!enableHighRes}
                                checked={enableHighRes && enableAnimations}
                                onChange={(_, { checked }) => {
                                    setEnableAnimations(checked);
                                }}
                            >
                                {t("menu.enable_animations")}
                            </MenuSwitch>
                        </Tooltip>
                    </MenuGroup>
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};

export const SettingsMenu = memo(SettingsMenuImpl);
