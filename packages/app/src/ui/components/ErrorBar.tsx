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
};

export const ErrorBar: React.FC<PropsWithChildren<ErrorBarProps>> = ({
    title,
    children,
}) => {
    const t = useUITranslation();
    return (
        <MessageBar intent="error">
            <MessageBarBody>
                <MessageBarTitle>{title || t("error")}</MessageBarTitle>
                <Text>
                    {children} <BugReportText />
                </Text>
            </MessageBarBody>
        </MessageBar>
    );
};
