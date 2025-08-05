import { useState } from "react";
import { Text, Button, Field } from "@fluentui/react-components";
import { ArrowDownload20Regular } from "@fluentui/react-icons";
import { fsSave } from "@pistonite/pure/fs";

import { useUITranslation } from "skybook-localization";

import { Code, CopyButton, Interpolate } from "self::ui/components";
import { useStyleEngine } from "self::util";

export type ExporterProps = {
    getScript: () => Promise<string | undefined>;
    getDirectUrl: () => Promise<string | undefined>;
};

export const Exporter: React.FC<ExporterProps> = ({ getScript, getDirectUrl }) => {
    const m = useStyleEngine();
    const t = useUITranslation();
    const [isTooLong, setIsTooLong] = useState(false);
    return (
        <div className={m("pad-8 flex-col gap-16")}>
            <div>
                <Field
                    label={t("exporter.direct_url")}
                    hint={t("exporter.direct_url.desc")}
                    validationState={isTooLong ? "warning" : undefined}
                    validationMessage={
                        isTooLong ? t("exporter.direct_url.too_long_warning") : undefined
                    }
                >
                    <CopyButton
                        textToCopy={async () => {
                            const url = await getDirectUrl();
                            if (url) {
                                setIsTooLong(url.length > 2000);
                            }
                            return url;
                        }}
                    />
                </Field>
            </div>
            <div>
                <Field label={t("exporter.save_as_file")} hint={t("exporter.save_as_file.desc")}>
                    <Button
                        icon={<ArrowDownload20Regular />}
                        onClick={async () => {
                            const script = await getScript();
                            if (script !== undefined) {
                                fsSave(script, "script.txt");
                            }
                        }}
                    >
                        {t("button.download")}
                    </Button>
                </Field>
            </div>
            <div>
                <Field label={t("exporter.github")}>
                    <Text>{t("exporter.github.desc")}</Text>
                    <Code wrap style={{ wordBreak: "break-word" }}>
                        {`${window.location.origin}/github/USER/REPO/BRANCH/PATH`}
                    </Code>
                </Field>
            </div>
            <div>
                <Field
                    label={t("exporter.other_service")}
                    hint={t("exporter.other_service.example").replace(
                        "{{origin}}",
                        window.location.origin,
                    )}
                >
                    <Text>
                        <Interpolate
                            https={<Code>https://</Code>}
                            https_replaced={<Code>{`${window.location.origin}/-/`}</Code>}
                        >
                            {t("exporter.other_service.desc")}
                        </Interpolate>
                    </Text>
                </Field>
            </div>
        </div>
    );
};
