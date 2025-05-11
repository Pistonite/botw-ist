import { memo, useState } from "react";
import {
    Option,
    Button,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    Field,
    RadioGroup,
    Radio,
    Checkbox,
    Dropdown,
    makeStyles,
    OptionGroup,
    Switch,
} from "@fluentui/react-components";

import { useUITranslation } from "skybook-localization";

import { isLessProductive, useNarrow } from "self::pure-contrib";
import {
    BuiltinExtensionIds,
    useExtensionStore,
    type ExtensionOpenMode,
    getCustomExtensionId,
    getOpenModeForExtension,
    useExtensionName,
} from "self::application/store";
import { openExtensionPopup } from "self::application/extension";
import { useStyleEngine, useUIStore } from "self::ui/functions";

const useStyles = makeStyles({
    dialogBody: {
        gap: "16px",
    },
});

const ExtensionLaunchDialogImpl: React.FC = () => {
    const m = useStyleEngine();
    const t = useUITranslation();
    const c = useStyles();
    const narrow = useNarrow();

    const open =
        useUIStore((state) => state.openedDialogId) === "extension-launch";
    const setOpen = useUIStore((state) => state.setOpenedDialog);

    const customExtensions = useExtensionStore((state) => state.custom);
    const pinnedExtensions = useExtensionStore((state) => state.pinnedIds);
    const setPinnedIds = useExtensionStore((state) => state.setPinnedIds);
    const updateOpenMode = useExtensionStore((state) => state.updateOpenMode);
    const updateRecency = useExtensionStore((state) => state.updateRecency);
    const openExtension = useExtensionStore((state) => state.open);

    const [selectedExtensionId, setSelectedExtensionId] = useState<string>(
        () => {
            const { currentPrimary, currentSecondary } =
                useExtensionStore.getState();
            return currentPrimary || currentSecondary || "editor";
        },
    );
    const isSelectedExtensionScriptEditor = selectedExtensionId === "editor";
    const isSelectedExtensionCustom = selectedExtensionId.startsWith("custom-");
    const selectedExtensionText = useExtensionName(selectedExtensionId);

    const [remember, setRemember] = useState(false);

    const [selectedOpenMode, setSelectedOpenModeInState] =
        useState<ExtensionOpenMode>(() => {
            return getOpenModeForExtension(selectedExtensionId);
        });
    const setSelectedOpenMode = (value: ExtensionOpenMode) => {
        setSelectedOpenModeInState(value);
    };

    const $SelectExtensionDropDown = (
        <Dropdown
            value={selectedExtensionText}
            onOptionSelect={(_, { selectedOptions }) => {
                const selectedId = selectedOptions[0] || "editor";
                setSelectedExtensionId(selectedId);
                setSelectedOpenMode(getOpenModeForExtension(selectedId));
            }}
        >
            {BuiltinExtensionIds.map((id) => (
                <Option key={id} value={id}>
                    {t(`extension.${id}.name`)}
                </Option>
            ))}
            {customExtensions.length > 0 && (
                <OptionGroup label={t("menu.custom_extensions")}>
                    {customExtensions.map((extension) => (
                        <Option
                            key={extension.url}
                            value={getCustomExtensionId(extension.url)}
                        >
                            {extension.name}
                        </Option>
                    ))}
                </OptionGroup>
            )}
        </Dropdown>
    );

    const $PinnedSwitch = (
        <Switch
            label={t("dialog.extension_launch.pinned")}
            checked={pinnedExtensions.includes(selectedExtensionId)}
            onChange={(_, { checked }) => {
                if (checked) {
                    setPinnedIds([...pinnedExtensions, selectedExtensionId]);
                } else {
                    setPinnedIds(
                        pinnedExtensions.filter(
                            (id) => id !== selectedExtensionId,
                        ),
                    );
                }
            }}
        />
    );

    const $OpenModeRadioGroup = !narrow && (
        <RadioGroup
            value={isSelectedExtensionCustom ? "popout" : selectedOpenMode}
            disabled={isSelectedExtensionCustom}
            onChange={(_, { value }) => {
                setSelectedOpenMode(value as ExtensionOpenMode);
            }}
        >
            <Radio
                value="primary"
                label={t("dialog.extension_launch.option.primary")}
            />
            <Field
                validationState={
                    isSelectedExtensionScriptEditor ? "warning" : undefined
                }
                validationMessage={
                    isSelectedExtensionScriptEditor
                        ? t(
                              "dialog.extension_launch.error.script_editor_no_secondary",
                          ).replace(
                              "{{script_editor}}",
                              t("extension.editor.name"),
                          )
                        : undefined
                }
            >
                <Radio
                    disabled={
                        isSelectedExtensionCustom ||
                        isSelectedExtensionScriptEditor
                    }
                    value="secondary"
                    label={t("dialog.extension_launch.option.secondary")}
                />
            </Field>
            <Radio
                value="popout"
                label={t("dialog.extension_launch.option.popout")}
            />
        </RadioGroup>
    );

    const $OpenModePopoutCheckbox = narrow && (
        <Checkbox
            label={t("dialog.extension_launch.option.popout")}
            disabled={isSelectedExtensionCustom}
            checked={isSelectedExtensionCustom || selectedOpenMode === "popout"}
            onChange={(_, { checked }) => {
                setSelectedOpenMode(checked ? "popout" : "primary");
            }}
        />
    );

    const $RememberCheckbox = (
        <Checkbox
            label={
                narrow
                    ? t("dialog.extension_launch.remember_popout")
                    : t("dialog.extension_launch.remember")
            }
            disabled={
                isSelectedExtensionCustom ||
                (narrow && selectedOpenMode !== "popout")
            }
            checked={remember && (!narrow || selectedOpenMode === "popout")}
            onChange={(_, { checked }) => {
                setRemember(!!checked);
            }}
        />
    );

    const handleLaunch = () => {
        // block custom extensions on non-PC platforms
        if (isLessProductive && isSelectedExtensionCustom) {
            return;
        }
        updateRecency(selectedExtensionId);
        // make sure open mode is valid for the current behavior
        const openMode = narrow
            ? selectedOpenMode === "popout"
                ? "popout"
                : "primary"
            : selectedOpenMode;

        if (openMode === "popout") {
            if (remember) {
                updateOpenMode(selectedExtensionId, "popout");
            }
            openExtensionPopup(selectedExtensionId);
            return;
        }

        openExtension(selectedExtensionId, openMode, !narrow && remember);
    };

    return (
        <Dialog
            open={open}
            onOpenChange={(_, { open }) =>
                setOpen(open ? "extension-launch" : undefined)
            }
        >
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>
                        {t("dialog.extension_launch.title")}
                    </DialogTitle>
                    <DialogContent>
                        <div className={m("flex-col", c.dialogBody)}>
                            <div>
                                <Field
                                    label={t(
                                        "dialog.extension_launch.select_title",
                                    )}
                                >
                                    {$SelectExtensionDropDown}
                                </Field>
                                {$PinnedSwitch}
                            </div>
                            <Field
                                label={t(
                                    "dialog.extension_launch.options_title",
                                )}
                                validationState={
                                    isSelectedExtensionCustom
                                        ? "warning"
                                        : undefined
                                }
                                validationMessage={
                                    isSelectedExtensionCustom
                                        ? t(
                                              "dialog.extension_launch.error.custom_popout_only",
                                          )
                                        : undefined
                                }
                            >
                                {$OpenModeRadioGroup}
                                {$OpenModePopoutCheckbox}
                            </Field>
                            <div className={m("flex-col")}>
                                {$RememberCheckbox}
                            </div>
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">
                                {t("button.cancel")}
                            </Button>
                        </DialogTrigger>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="primary" onClick={handleLaunch}>
                                {t("button.launch")}
                            </Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
export const ExtensionLaunchDialog = memo(ExtensionLaunchDialogImpl);
