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
} from "@fluentui/react-components";
import {
    BookQuestionMark20Regular,
    MoreHorizontal20Regular,
} from "@fluentui/react-icons";
import {
    DarkToggle,
    GitHubLink,
    LanguagePicker,
} from "@pistonite/shared-controls";

import { useSessionStore } from "self::application/store";

import { ExtensionOpenButton } from "./ExtensionOpenButton.tsx";
import icon from "./icon.svg";
import iconPurple from "./icon-purple.svg";

const useStyles = makeStyles({
    container: {
        backgroundColor: tokens.colorNeutralBackground2,
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
    },
});

export const Header: React.FC = () => {
    const styles = useStyles();
    const version = import.meta.env.VERSION.replace("0.", "v");
    const commitShort = import.meta.env.COMMIT.substring(0, 8);

    const isRunningCustomImage = useSessionStore(
        (state) => state.runningCustomImageVersion,
    );
    return (
        <div className={styles.container}>
            <img src={isRunningCustomImage ? iconPurple : icon} height="24px" />
            <Menu>
                <MenuTrigger disableButtonEnhancement>
                    <Button
                        appearance="subtle"
                        icon={<MoreHorizontal20Regular />}
                    />
                </MenuTrigger>
                <MenuPopover>
                    <MenuList>
                        <MenuDivider />

                        <Caption1>
                            {version} ({commitShort})
                        </Caption1>
                    </MenuList>
                </MenuPopover>
            </Menu>

            <LanguagePicker />
            <DarkToggle />
            <GitHubLink href="https://github.com/Pistonite/botw-ist" />
            <Button appearance="subtle" icon={<BookQuestionMark20Regular />} />
            <ExtensionOpenButton />
            <span>5 errors</span>
        </div>
    );
};
