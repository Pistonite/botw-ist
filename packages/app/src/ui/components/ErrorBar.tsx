import type { PropsWithChildren } from "react";
import {
    Text,
    MessageBar,
    MessageBarBody,
    MessageBarTitle,
} from "@fluentui/react-components";

import { useUITranslation } from "skybook-localization";

import { BugReportText } from "./BugReportText.tsx";

export type ErrorBarProps = {
    /** Title for the error. If not specified, use the generic term "Error" */
    title?: string;

    /** Use warning style instead of error style */
    isWarning?: boolean;

    /** Hide the bug report text */
    noBugReport?: boolean;
};

export const ErrorBar: React.FC<PropsWithChildren<ErrorBarProps>> = ({
    title,
    isWarning,
    noBugReport,
    children,
}) => {
    const t = useUITranslation();
    return (
        <MessageBar intent={isWarning ? "warning" : "error"}>
            <MessageBarBody>
                {!isWarning && (
                    <MessageBarTitle>{title || t("error")}</MessageBarTitle>
                )}
                <Text>
                    {children} {!noBugReport && <BugReportText />}
                </Text>
            </MessageBarBody>
        </MessageBar>
    );
};
