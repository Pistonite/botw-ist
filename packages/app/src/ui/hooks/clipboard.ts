import { useRef, useState } from "react";

import { log } from "#util";

export interface CopyToClipboardHook {
    copyFn: () => Promise<void>;
    isJustCopied: boolean;
}

export const useCopyToClipboard = (textToCopy: string | (() => Promise<string | undefined>)) => {
    const [isCopied, setIsCopied] = useState(false);
    const timeoutRef = useRef<number | undefined>(undefined);
    return {
        copyFn: async () => {
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
                void navigator.clipboard.writeText(text);
                setIsCopied(true);
                timeoutRef.current = setTimeout(() => {
                    setIsCopied(false);
                }, 2000) as unknown as number;
            } catch (e) {
                log.error("failed to copy text to clipboard");
                log.error(e);
            }
        },
        isJustCopied: isCopied,
    };
};
