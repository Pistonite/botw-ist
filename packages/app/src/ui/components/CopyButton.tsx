import { Button } from "@fluentui/react-components";
import { Checkmark20Regular, Copy20Regular } from "@fluentui/react-icons";
import { useRef, useState } from "react";

import { useUITranslation } from "skybook-localization";

import { log } from "self::util";

export type CopyButtonProps = {
    textToCopy: string | (() => Promise<string | undefined>);
};

export const CopyButton: React.FC<CopyButtonProps> = ({ textToCopy }) => {
    const t = useUITranslation();
    const [isCopied, setIsCopied] = useState(false);
    const timeoutRef = useRef<number | undefined>(undefined);
    return (
        <Button
            appearance="primary"
            icon={isCopied ? <Checkmark20Regular /> : <Copy20Regular />}
            onClick={async () => {
                if (timeoutRef.current) {
                    clearTimeout(timeoutRef.current);
                }
                try {
                    let text: string;
                    if (typeof textToCopy === "function") {
                        const text2 = await textToCopy();
                        if (text2 === undefined) {
                            return;
                        }
                        text = text2;
                    } else {
                        text = textToCopy;
                    }
                    void navigator.clipboard.writeText("```\n" + text + "```");
                    setIsCopied(true);
                    timeoutRef.current = setTimeout(() => {
                        setIsCopied(false);
                    }, 2000) as unknown as number;
                } catch (e) {
                    log.error("failed to copy text to clipboard");
                    log.error(e);
                }
            }}
        >
            {isCopied ? t("button.copied") : t("button.copy")}
        </Button>
    );
};
