import { memo, useEffect, useState } from "react";
import {
    Textarea,
    Text,
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    Field,
} from "@fluentui/react-components";
import { InlineLink } from "@pistonite/shared-controls";

import { useUITranslation } from "skybook-localization";

import {
    type CustomExtension,
    getCustomExtensionConfigText,
    useExtensionStore,
} from "self::application/store";

import { useUIStore } from "./store.ts";
import { Code } from "./components/Code.tsx";
import { Interpolate } from "./components/Interpolate.tsx";

const FORMAT = "NAME=URL";

const CustomExtensionDialogImpl: React.FC = () => {
    const t = useUITranslation();

    const open =
        useUIStore((state) => state.openedDialogId) === "custom-extension";
    const setOpen = useUIStore((state) => state.setOpenedDialog);
    const setCustomExtensions = useExtensionStore(
        (state) => state.setCustomExtensions,
    );
    const [configText, setConfigText] = useState<string>(
        getCustomExtensionConfigText,
    );
    // when opening the dialog, re-initialize the text area
    useEffect(() => {
        if (open) {
            setConfigText(getCustomExtensionConfigText());
        }
    }, [open]);

    const handleSave = () => {
        const lines = configText.split("\n");
        const customExtensions: CustomExtension[] = [];
        for (const line of lines) {
            const l = line.trim();
            if (!l) {
                continue;
            }
            const parts = l.split("=", 2);
            if (parts.length !== 2) {
                continue;
            }
            const name = parts[0].trim();
            const url = parts[1].trim();
            if (!name || !url) {
                continue;
            }
            customExtensions.push({ name, url });
        }
        setCustomExtensions(customExtensions);
    };

    const $TextAreaTitle = (
        <Interpolate format={<Code>{FORMAT}</Code>}>
            {t("dialog.custom_extensions.format_desc")}
        </Interpolate>
    );

    return (
        <Dialog
            open={open}
            onOpenChange={(_, { open }) =>
                setOpen(open ? "custom-extension" : undefined)
            }
        >
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t("menu.custom_extensions")}</DialogTitle>
                    <DialogContent>
                        <Text block>
                            {t("dialog.custom_extensions.desc")}{" "}
                            <InlineLink href="https://skybook.pistonite.dev">
                                {t("button.learn_more")}
                            </InlineLink>
                        </Text>
                        {$TextAreaTitle}
                        <Field
                            hint={t("dialog.custom_extensions.help").replace(
                                "{{open_button}}",
                                t("menu.open_configure_extension"),
                            )}
                        >
                            <Textarea
                                value={configText}
                                onChange={(_, { value }) =>
                                    setConfigText(value)
                                }
                                rows={10}
                                textarea={{
                                    style: {
                                        fontFamily: "monospace",
                                    },
                                }}
                            />
                        </Field>
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">
                                {t("button.cancel")}
                            </Button>
                        </DialogTrigger>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="primary" onClick={handleSave}>
                                {t("button.ok")}
                            </Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};

export const CustomExtensionDialog = memo(CustomExtensionDialogImpl);
