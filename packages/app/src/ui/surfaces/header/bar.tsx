import { memo } from "react";
import { makeStyles, tokens } from "@fluentui/react-components";

import { useSessionStore } from "self::application";
import { isLessProductive } from "self::pure-contrib";
import { ExtensionsMenu } from "self::ui/surfaces/extension";
import { useStyleEngine } from "self::util";

import icon from "./icon.svg";
import iconPurple from "./icon-purple.svg";
import { SettingsMenu } from "./settings.tsx";
import { PerfMonitor } from "./debugger.tsx";
import { MiscMenu } from "./three_dot.tsx";
import { ModeSwitcher } from "./switch_mode.tsx";

const useStyles = makeStyles({
    container: {
        backgroundColor: tokens.colorNeutralBackground2,
        height: "40px",
    },
    logo: {
        width: "40px",
    },
});

const HeaderImpl: React.FC = () => {
    const m = useStyleEngine();
    const c = useStyles();

    const isRunningCustomImage = useSessionStore(
        (state) => state.runningCustomImageVersion,
    );
    return (
        <div className={m("flex-row flex-centera gap-4", c.container)}>
            <div className={m("flex flex-center", c.logo)}>
                <img
                    src={isRunningCustomImage ? iconPurple : icon}
                    height="32px"
                />
            </div>
            <ModeSwitcher />
            <SettingsMenu />
            {
                // Custom extensions are limited to PC platform only
                // On other platforms, you can already select all built-in extensions
                // through the extension window toolbar, so there's no need
                // for this menu
                !isLessProductive && <ExtensionsMenu />
            }
            <MiscMenu />
            <div className={m("flex-row flex-1 flex-end")}>
                <PerfMonitor />
            </div>
        </div>
    );
};

export const Header = memo(HeaderImpl);
