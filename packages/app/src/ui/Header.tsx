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

import icon from "./icon.svg";
import iconPurple from "./icon-purple.svg";
import { SettingsMenu } from "./SettingsMenu.tsx";
import { ExtensionsMenu } from "./ExtensionsMenu.tsx";
import { isLessProductive } from "../pure-contrib/platform.ts";
import { PerfMonitor } from "./PerfMonitor.tsx";

const useStyles = makeStyles({
    container: {
        backgroundColor: tokens.colorNeutralBackground2,
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        gap: "4px",
        height: "40px",
    },
    logo: {
        width: "40px",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
    },
    end: {
        flexGrow: 1,
        display: "flex",
        flexDirection: "row",
        justifyContent: "flex-end",
    },
});

const HeaderImpl: React.FC = () => {
    const styles = useStyles();
    const version = import.meta.env.VERSION.replace("0.", "v");
    const commitShort = import.meta.env.COMMIT.substring(0, 8);

    const isRunningCustomImage = useSessionStore(
        (state) => state.runningCustomImageVersion,
    );
    return (
        <div className={styles.container}>
            <div className={styles.logo}>
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
            <div className={styles.end}>
                <PerfMonitor />
            </div>
        </div>
    );
};

export const Header = memo(HeaderImpl);
