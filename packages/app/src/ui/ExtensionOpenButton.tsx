import {
    Text,
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    Tooltip,
    Field,
    Combobox,
    RadioGroup,
    Radio,
    Checkbox,
} from "@fluentui/react-components";
import { WindowDevTools20Regular } from "@fluentui/react-icons";
import type { ExtensionOpenMode } from "application/extensionStore";
import { useNarrow } from "pure-contrib/narrow";
import { useState } from "react";
import { useUITranslation } from "skybook-localization";
import { isLessProductive } from "pure-contrib/platform";

export const ExtensionOpenButton: React.FC = () => {
    const t = useUITranslation();

    const narrow = useNarrow();

    const [selectedOpenMode, setSelectedOpenMode] =
        useState<ExtensionOpenMode>("secondary");

    let displayedOpenMode = selectedOpenMode;
    const secondaryAvailable = !narrow && !isLessProductive;
    if (!secondaryAvailable && selectedOpenMode === "secondary") {
        displayedOpenMode = "primary";
    }

    return (
        <Dialog>
            <DialogTrigger disableButtonEnhancement>
                <Tooltip
                    content={t("button.open_extension")}
                    relationship="label"
                >
                    <Button
                        icon={<WindowDevTools20Regular />}
                        appearance="subtle"
                    />
                </Tooltip>
            </DialogTrigger>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t("dialog.extensions.title")}</DialogTitle>
                    <DialogContent>
                        <Text>{t("dialog.extensions.desc")} </Text>
                        <Field label={t("field.select_extension")}>
                            <Combobox></Combobox>
                        </Field>
                        <RadioGroup
                            value={displayedOpenMode}
                            onChange={(_, { value }) => {
                                setSelectedOpenMode(value as ExtensionOpenMode);
                            }}
                        >
                            <Radio
                                value="primary"
                                label={t("radio.extension_open_mode.primary")}
                            />
                            <Field
                                validationState={
                                    !secondaryAvailable ? "warning" : undefined
                                }
                                validationMessage={
                                    narrow
                                        ? t("status.not_available_window_size")
                                        : isLessProductive
                                          ? t("status.not_available_platform")
                                          : undefined
                                }
                            >
                                <Radio
                                    value="secondary"
                                    label={t(
                                        "radio.extension_open_mode.secondary",
                                    )}
                                    disabled={!secondaryAvailable}
                                />
                            </Field>
                            <Field
                                validationState={
                                    isLessProductive ? "warning" : undefined
                                }
                                validationMessage={
                                    isLessProductive
                                        ? t("status.not_available_platform")
                                        : undefined
                                }
                            >
                                <Radio
                                    value="popout"
                                    label={t(
                                        "radio.extension_open_mode.popout",
                                    )}
                                    disabled={isLessProductive}
                                />
                            </Field>
                        </RadioGroup>
                        <Field hint={"Only effective after pressing Open"}>
                            <Checkbox label={t("field.persist")} />
                        </Field>
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">Close</Button>
                        </DialogTrigger>
                        <Button appearance="primary">Do Something</Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
