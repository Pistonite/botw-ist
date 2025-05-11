import { memo } from "react";
import {
    Menu,
    Button,
    makeStyles,
    tokens,
    MenuTrigger,
    MenuPopover,
    MenuList,
    MenuDivider,
    Caption1,
    MenuItem,
} from "@fluentui/react-components";
import {
    BookQuestionMark20Regular,
    MoreHorizontal20Regular,
} from "@fluentui/react-icons";
import { GitHubLink } from "@pistonite/shared-controls";

import { useSessionStore } from "self::application/store";
import { isLessProductive } from "self::pure-contrib";
import { ExtensionsMenu } from "self::ui/surfaces/extension";

import icon from "./icon.svg";
import iconPurple from "./icon-purple.svg";
import { SettingsMenu } from "./SettingsMenu.tsx";
import { PerfMonitor } from "./PerfMonitor.tsx";
import { useStyleEngine } from "../../functions/SharedStyles.ts";

const useStyles = makeStyles({
    container: {
        backgroundColor: tokens.colorNeutralBackground2,
        gap: "4px",
        height: "40px",
    },
    logo: {
        width: "40px",
    },
});

const HeaderImpl: React.FC = () => {
    const m = useStyleEngine();
    const c = useStyles();
    const version = import.meta.env.VERSION.replace("0.", "v");
    const commitShort = import.meta.env.COMMIT.substring(0, 8);

    const isRunningCustomImage = useSessionStore(
        (state) => state.runningCustomImageVersion,
    );
    return (
        <div className={m("flex-row flex-centera", c.container)}>
            <div className={m("flex flex-center", c.logo)}>
                <img
                    src={isRunningCustomImage ? iconPurple : icon}
                    height="32px"
                />
            </div>
            <SettingsMenu />
            {
                // Custom extensions are limited to PC platform only
                // On other platforms, you can already select all built-in extensions
                // through the extension window toolbar, so there's no need
                // for this menu
                !isLessProductive && <ExtensionsMenu />
            }

            <Menu>
                <MenuTrigger disableButtonEnhancement>
                    <Button
                        appearance="subtle"
                        icon={<MoreHorizontal20Regular />}
                    />
                </MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        <MenuItem icon={<BookQuestionMark20Regular />}>
                            Skybook Manual
                        </MenuItem>
                        <GitHubLink href="https://github.com/Pistonite/botw-ist" />
                        <MenuDivider />

                        <Caption1>
                            {version} ({commitShort})
                        </Caption1>
                    </MenuList>
                </MenuPopover>
            </Menu>
            <div className={m("flex-row flex-1 flex-end")}>
                <PerfMonitor />
            </div>
        </div>
    );
};

export const Header = memo(HeaderImpl);
