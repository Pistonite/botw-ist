import { Button } from "@fluentui/react-components";
import { Checkmark20Regular, Copy20Regular } from "@fluentui/react-icons";

import { useUITranslation } from "skybook-localization";

import { useCopyToClipboard } from "#ui/hooks";

export type CopyButtonProps = {
    textToCopy: string | (() => Promise<string | undefined>);
};

export const CopyButton: React.FC<CopyButtonProps> = ({ textToCopy }) => {
    const t = useUITranslation();
    const { copyFn, isJustCopied } = useCopyToClipboard(textToCopy);
    return (
        <Button
            appearance="primary"
            icon={isJustCopied ? <Checkmark20Regular /> : <Copy20Regular />}
            onClick={copyFn}
        >
            {isJustCopied ? t("button.copied") : t("button.copy")}
        </Button>
    );
};
