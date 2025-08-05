import { InlineLink } from "@pistonite/shared-controls";

import { useUITranslation } from "skybook-localization";

import { Interpolate } from "./Interpolate.tsx";

export const BugReportText: React.FC = () => {
    const t = useUITranslation();
    return (
        <Interpolate
            github={
                <InlineLink href="https://github.com/Pistonite/botw-ist/issues">GitHub</InlineLink>
            }
        >
            {t("main.bug_report")}
        </Interpolate>
    );
};
