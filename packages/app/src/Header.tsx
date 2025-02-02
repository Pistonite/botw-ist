import { Button, makeStyles, tokens } from "@fluentui/react-components";
import {
    BookQuestionMark20Regular,
    Settings20Regular,
} from "@fluentui/react-icons";
import {
    DarkToggle,
    GitHubLink,
    LanguagePicker,
} from "@pistonite/shared-controls";

import { useIsShowingExtensionPanel } from "application/extensionStore";
import { ExtensionOpenButton } from "ui/ExtensionOpenButton";

import icon from "./icon.svg";

const useStyles = makeStyles({
    container: {
        backgroundColor: tokens.colorNeutralBackground2,
        display: "flex",
        flexDirection: "row",
    },
});

export const Header: React.FC = () => {
    const styles = useStyles();
    const showingExtensionPanel = useIsShowingExtensionPanel();
    return (
        <div className={styles.container}>
            <img src={icon} height="32px" />
            <span>v4.0.0</span>

            <LanguagePicker />
            <DarkToggle />
            <GitHubLink href="https://github.com/Pistonite/botw-ist" />
            <Button appearance="subtle" icon={<Settings20Regular />} />
            <Button appearance="subtle" icon={<BookQuestionMark20Regular />} />
            {!showingExtensionPanel && <ExtensionOpenButton />}
            <span>5 errors</span>
        </div>
    );
};
