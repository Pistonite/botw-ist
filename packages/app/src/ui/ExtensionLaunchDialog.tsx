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
import { BuiltinExtensionIds, useExtensionStore, type ExtensionOpenMode, getCustomExtensionId, getOpenModeForExtension, useExtensionName } from "self::application/store";
import { openExtensionPopup } from "self::application/extension";
import { useUIStore } from "./store";

const useStyles = makeStyles({
    dialogBody: {
        display: "flex",
        flexDirection: "column",
        gap: "16px",
    },
    otherOptions: {
        display: "flex",
        flexDirection: "column",
    }
});

const ExtensionLaunchDialogImpl: React.FC = () => {
    const t = useUITranslation();
    const narrow = useNarrow();
    const styles = useStyles();

    const open = useUIStore(state => state.openedDialogId) === "extension-launch";
    const setOpen = useUIStore(state => state.setOpenedDialog);

    const customExtensions = useExtensionStore(state => state.custom);
    const pinnedExtensions = useExtensionStore(state => state.pinnedIds);
    const setPinnedIds = useExtensionStore(state => state.setPinnedIds);
    const updateOpenMode = useExtensionStore(state => state.updateOpenMode);
    const updateRecency = useExtensionStore(state => state.updateRecency);
    const openExtension = useExtensionStore(state => state.open);

    const [selectedExtensionId, setSelectedExtensionId] = useState<string>(() => {
        const { currentPrimary, currentSecondary } = useExtensionStore.getState();
        return currentPrimary || currentSecondary || "editor";
    });
    const isSelectedExtensionCustom = selectedExtensionId.startsWith("custom-");
    const selectedExtensionText = useExtensionName(selectedExtensionId);

    const [remember, setRemember] = useState(false);

    const [selectedOpenMode, setSelectedOpenModeInState] = useState<ExtensionOpenMode>(() => {
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
            {
                BuiltinExtensionIds.map((id) => (
                    <Option key={id} value={id}>
                        {t(`extension.${id}.name`)}
                    </Option>
                ))
            }
            {
                customExtensions.length > 0 && (
                    <OptionGroup label="Custom Extensions">
                        {
                            customExtensions.map((extension) => (
                                <Option key={extension.url} value={getCustomExtensionId(extension.url)}>
                                    {extension.name}
                                </Option>
                            ))}
                    </OptionGroup>
                )
            }
        </Dropdown>
    );

    const $PinnedSwitch = (
        <Switch 
            label={"Pinned"}
            checked={pinnedExtensions.includes(selectedExtensionId)}
            onChange={(_, {checked}) => {
                if (checked) {
                    setPinnedIds([...pinnedExtensions, selectedExtensionId]);
                } else {
                    setPinnedIds(pinnedExtensions.filter(id => id !== selectedExtensionId));
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
                label={t("radio.extension_open_mode.primary")}
            />
            <Field
                validationState={selectedExtensionId === "editor" ? "warning" : undefined}
                validationMessage={selectedExtensionId === "editor" ? "Script Editor cannot be opened in secondary view" : undefined}
            >
                <Radio
                    disabled={isSelectedExtensionCustom || selectedExtensionId === "editor"}
                    value="secondary"
                    label={t(
                        "radio.extension_open_mode.secondary",
                    )}
                />
            </Field>
            <Radio
                value="popout"
                label={t(
                    "radio.extension_open_mode.popout",
                )}
            />
        </RadioGroup>
    );

    const $OpenModePopoutCheckbox = narrow && (
        <Checkbox 
            label={"Open as popout"}
            checked={selectedOpenMode === "popout"}
            onChange={(_, {checked}) =>{
                setSelectedOpenMode(checked ? "popout" : "primary");
            }}
        />
    );

    const $RememberCheckbox = (
        <Checkbox 
            label={
                narrow ? 
                    "Open as popout by default"
                    :
                    "Remember the open mode for this extension"
            }
            disabled={isSelectedExtensionCustom || (narrow && selectedOpenMode !== "popout")}
            checked={remember && (!narrow || selectedOpenMode === "popout")}
            onChange={(_, {checked}) =>{
                setRemember(!!checked);
            }}
        />
    );

    const handleLaunch = () => {
        // block custom extensions on non-PC platforms
        if (isLessProductive && selectedExtensionId.startsWith("custom-")) {
            return;
        }
        updateRecency(selectedExtensionId);
        // make sure open mode is valid for the current behavior
        const openMode = narrow ?
            selectedOpenMode === "popout" ? "popout" : "primary"
            :
            selectedOpenMode;

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
        <Dialog open={open} onOpenChange={(_, { open }) => setOpen(open ? "extension-launch" : undefined)}>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{"Launch Extension"}</DialogTitle>
                    <DialogContent>
                        <div className={styles.dialogBody}>
                            <div>
                                <Field label={t("field.select_extension")}>
                                    {$SelectExtensionDropDown}
                                </Field>
                                {$PinnedSwitch}
                            </div>
                            <Field label={"Launch options"}
                                validationState={isSelectedExtensionCustom ? "warning" : undefined}
                                validationMessage={isSelectedExtensionCustom ? "Custom extensions can only be opened as popout" : undefined}
                            >
                                {$OpenModeRadioGroup}
                                {$OpenModePopoutCheckbox}
                            </Field>
                            <div className={styles.otherOptions}>
                                {$RememberCheckbox}
                            </div>
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">{t("button.cancel")}</Button>
                        </DialogTrigger>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="primary" onClick={handleLaunch} >{t("button.launch")}</Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
export const ExtensionLaunchDialog = memo(ExtensionLaunchDialogImpl);
