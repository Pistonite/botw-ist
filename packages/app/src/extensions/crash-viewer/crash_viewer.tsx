import { Body1, Field } from "@fluentui/react-components";
import { useDark } from "@pistonite/celera";

import { useUITranslation } from "skybook-localization";

import { useStyleEngine } from "#util";
import { CopyButton } from "#ui/components";

export type CrashViewerProps = {
    crashInfo: string;
};

const useStyles = useStyleEngine.extend({
    dark: {
        backgroundColor: "#292c3c",
        color: "#ef9f76",
    },
    light: {
        backgroundColor: "#e6e9ef",
        color: "#e64553",
    },
});

export const CrashViewer: React.FC<CrashViewerProps> = ({ crashInfo }) => {
    const dark = useDark();
    const m = useStyles();
    const t = useUITranslation();
    if (!crashInfo) {
        return (
            <div className={m("flex-col h-100 border-box")}>
                <div className={m("flex flex-1 flex-center")}>
                    <Body1>{t("crash_viewer.no_crash")}</Body1>
                </div>
            </div>
        );
    }
    return (
        <div className={m("flex-col h-100 border-box")}>
            <div className={m("pad-4")}>
                <Field>
                    <CopyButton textToCopy={"```\n" + crashInfo + "```"} />
                </Field>
            </div>
            <div className={m("overflow-y-auto flex-1", dark ? "c-dark" : "c-light")}>
                <div className={m("max-h-0 overflow-visible pad-4")}>
                    <pre className={m("margin-0")}>{crashInfo}</pre>
                </div>
            </div>
        </div>
    );
};
