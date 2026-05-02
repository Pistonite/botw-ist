import { memo } from "react";
import { tokens } from "@fluentui/react-components";
import { isMobile } from "@pistonite/celera";

import { useSessionStore } from "#application";
import { ExtensionsMenu } from "#ui/surfaces/extension";
import { useStyleEngine } from "#util";

import icon from "./icon.svg";
import iconPurple from "./icon-purple.svg";
import { SettingsMenu } from "./settings.tsx";
import { PerfMonitor } from "./debugger.tsx";
import { MiscMenu } from "./three_dot.tsx";
import { ModeSwitcher } from "./switch_mode.tsx";

const useStyles = useStyleEngine.extend({
    container: {
        backgroundColor: tokens.colorNeutralBackground2,
        height: "40px",
    },
    logo: {
        width: "40px",
    },
});

export const Header: React.FC = memo(() => {
    const m = useStyles();

    const isRunningCustomImage = useSessionStore((state) => state.runningCustomImageVersion);
    return (
        <div className={m("flex-row flex-centera gap-4 c-container")}>
            <div className={m("flex flex-center c-logo")}>
                <img src={isRunningCustomImage ? iconPurple : icon} height="32px" />
            </div>
            <SettingsMenu />
            {
                // Custom extensions are limited to PC platform only
                // On other platforms, you can already select all built-in extensions
                // through the extension window toolbar, so there's no need
                // for this menu
                !isMobile() && <ExtensionsMenu />
            }
            <MiscMenu />
            <div className={m("flex-row flex-1 flex-end")}>
                <ModeSwitcher />
                <PerfMonitor />
            </div>
        </div>
    );
});
