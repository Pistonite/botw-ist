import { memo } from "react";
import {
    Button,
    Caption1,
    Menu,
    MenuDivider,
    MenuItem,
    MenuList,
    MenuPopover,
    MenuTrigger,
    Tooltip,
} from "@fluentui/react-components";
import {
    BookQuestionMark20Regular,
    MoreHorizontal20Regular,
    PersonFeedback20Regular,
} from "@fluentui/react-icons";
import { useLocale } from "@pistonite/pure-react";

import { useUITranslation } from "skybook-localization";
import { GitHubLink } from "@pistonite/shared-controls";

const MiscMenuImpl: React.FC = () => {
    const t = useUITranslation();

    const version = import.meta.env.VERSION.replace("0.", "v");
    const commitShort = import.meta.env.COMMIT.substring(0, 8);

    const locale = useLocale();
    const isZh = locale.startsWith("zh");
    const manualBaseUrl = isZh
        ? "https://skybook.pistonite.dev/zh/" 
         :"https://skybook.pistonite.dev/";

    return (
        <Menu>
            <MenuTrigger disableButtonEnhancement>
                <Tooltip
                    relationship="label"
                    content={t("menu.header.more")}
                    positioning="below"
                >
                    <Button
                        appearance="subtle"
                        icon={<MoreHorizontal20Regular />}
                    />
                </Tooltip>
            </MenuTrigger>
            <MenuPopover>
                <MenuList>
                    <MenuItem
                        icon={<BookQuestionMark20Regular />}
                        onClick={() => {
                            window.open(manualBaseUrl, "_blank");
                        }}
                    >
                        {t("menu.skybook_manual")}
                    </MenuItem>
                    <MenuItem
                        icon={<PersonFeedback20Regular />}
                        onClick={() => {
                            window.open(manualBaseUrl + (isZh ? "#遇到问题" : "#having-an-issue"), "_blank");
                        }}
                    >
                        {t("menu.report_issue")}
                    </MenuItem>
                    <GitHubLink
                        href="https://github.com/Pistonite/botw-ist"
                        as="submenu"
                    />
                    <MenuDivider />
                    <Caption1>
                        {version} ({commitShort})
                    </Caption1>
                </MenuList>
            </MenuPopover>
        </Menu>
    );
};

export const MiscMenu = memo(MiscMenuImpl);
