import { Codicon, SimpleEditor, StatusItemPreset } from "@pistonite/intwc";

import { useUITranslation } from "skybook-localization";
import { useCopyToClipboard } from "#ui/hooks";

export type CrashViewerProps = {
    crashInfo: string;
};

export const CrashViewer: React.FC<CrashViewerProps> = ({ crashInfo }) => {
    const t = useUITranslation();
    const { copyFn: copyCrashInfo, isJustCopied } = useCopyToClipboard("```\n" + crashInfo + "```");

    return (
        <SimpleEditor
            value={crashInfo || `<${t("crash_viewer.no_crash")}>`}
            onValueChange={() => {}}
            editorOptions={{
                readOnly: true,
            }}
            language={crashInfo ? "cpp" : "text"}
            statusLeft={[
                {
                    onClick: copyCrashInfo,
                    body: isJustCopied ? (
                        <>
                            <Codicon icon="check" />
                            {t("button.copied")}
                        </>
                    ) : (
                        <>
                            <Codicon icon="copy" />
                            {t("button.copy")}
                        </>
                    ),
                },
            ]}
            statusRight={[StatusItemPreset.Position, StatusItemPreset.WordWrap]}
        />
    );
};
