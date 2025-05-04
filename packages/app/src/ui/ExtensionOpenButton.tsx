import { memo, useState } from "react";
import {
    Text,
    Option,
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
    MenuItem,
    Dropdown,
    makeStyles,
    OptionGroup,
    Switch,
} from "@fluentui/react-components";
import { WindowDevTools20Regular } from "@fluentui/react-icons";

import { useUITranslation } from "skybook-localization";

import { BuiltinExtensionIds, useExtensionStore, type ExtensionOpenMode, getCustomExtensionId, getOpenModeForExtension, useExtensionName } from "self::application/store";
import { isLessProductive, useNarrow } from "self::pure-contrib";
import { openExtensionPopup } from "../application/extension/ExtensionManager";

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

const ExtensionOpenButtonImpl: React.FC = () => {
    const t = useUITranslation();
    const narrow = useNarrow();
    const styles = useStyles();

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

    const [remember, setRemember] = useState(true);

    const [selectedOpenMode, setSelectedOpenModeInState] = useState<ExtensionOpenMode>(() => {
        return getOpenModeForExtension(selectedExtensionId);
    });
    const setSelectedOpenMode = (value: ExtensionOpenMode) => {
        setSelectedOpenModeInState(value);
    };

    return (
        <Dialog>
            <DialogTrigger disableButtonEnhancement>
                <Tooltip
                    content={t("menu.open_configure_extension.desc")}
                    relationship="description"
                    positioning="after"
                >
                    <MenuItem icon={<WindowDevTools20Regular />}
                        persistOnClick
                    >
                        {t("menu.open_configure_extension")}
                    </MenuItem>
                </Tooltip>
            </DialogTrigger>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{"Launch Extension"}</DialogTitle>
                    <DialogContent>
                        <div className={styles.dialogBody}>
                            <div>
                                <Field label={t("field.select_extension")}>
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
                                </Field>
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
                            </div>
                            <Field label={"Launch options"}
                                validationState={isSelectedExtensionCustom ? "warning" : undefined}
                                validationMessage={isSelectedExtensionCustom ? "Custom extensions can only be opened as popout" : undefined}
                            >
                                {
                                    !narrow && (
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
                                            <Radio
                                                value="secondary"
                                                label={t(
                                                    "radio.extension_open_mode.secondary",
                                                )}
                                            />
                                            <Radio
                                                value="popout"
                                                label={t(
                                                    "radio.extension_open_mode.popout",
                                                )}
                                            />
                                        </RadioGroup>
                                    )
                                }
                                {
                                    narrow && (
                                        <Checkbox 
                                            label={"Open as popout"}
                                            checked={selectedOpenMode === "popout"}
                                            onChange={(_, {checked}) =>{
                                                setSelectedOpenMode(checked ? "popout" : "primary");
                                            }}
                                        />
                                    )
                                }
                            </Field>
                            <div className={styles.otherOptions}>
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
                            </div>
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button appearance="secondary">{t("button.cancel")}</Button>
                        </DialogTrigger>
                        <Button appearance="primary"
                            onClick={() => {
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
                            }}
                        >{t("button.launch")}</Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};
export const ExtensionOpenButton = memo(ExtensionOpenButtonImpl);
