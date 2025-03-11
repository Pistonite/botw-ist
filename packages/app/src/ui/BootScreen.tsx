import { useEffect, useState } from "react";
import { FsErr, fsOpenFile } from "@pistonite/pure/fs";
import {
    Text,
    Button,
    Checkbox,
    Dialog,
    DialogActions,
    DialogBody,
    DialogContent,
    DialogSurface,
    DialogTitle,
    DialogTrigger,
    Field,
    Link,
    Radio,
    RadioGroup,
    makeStyles,
} from "@fluentui/react-components";

import type {
    RuntimeInitArgs,
    RuntimeInitParams,
} from "@pistonite/skybook-api";
import type { RuntimeClient } from "@pistonite/skybook-api/interfaces/Runtime.send";
import { translateUI, useUITranslation } from "skybook-localization";

import { useApplicationStore } from "self::application/store";
import {
    initRuntime,
    setCustomImageToProvide,
} from "self::application/runtime";

export type BootScreenState =
    | "OpenSetupOrUseDefaultImage"
    | "UseCustomOrUseDefaultImage"
    | "SetupDialog"
    | "Initializing"
    | "Error";

export type OpenSetupOrDefaultPromptType =
    | "LocalVersionMismatch"
    | "LocalNoImage"
    | "DirectLoadVersionMismatch"
    | "DirectLoadNoImage"
    | "InitializeError";

export type BootScreenProps = {
    /** The runtime client */
    runtime: Promise<RuntimeClient>;
    /** The version of the script image, read from env block */
    scriptImageVersion?: string;
    /** Params from the script */
    params: RuntimeInitParams;
    /** State of the boot flow when initially showing the screen */
    initialState: BootScreenState;
    /** Initial error string, if the state is "InitializeError" */
    initialErrorString?: string;
    /** If the initial state is "OpenSetupOrUseDefaultImage", this is the prompt type */
    openSetupOrDefaultPromptType?: OpenSetupOrDefaultPromptType;
    /** Callback to call when booting is successful */
    onSuccess: () => void;
};

let bootScreenSuccessCalled = false;

const useStyles = makeStyles({
    spacer: {
        height: "32px",
    },
    fileUploadSection: {
        display: "flex",
        flexDirection: "row",
        gap: "8px",
        alignItems: "center",
        "& span": {
            flex: 1,
            overflow: "hidden",
            textOverflow: "ellipsis",
            minWidth: 0,
        },
    },
});

export const BootScreen: React.FC<BootScreenProps> = ({
    runtime,
    scriptImageVersion,
    params,
    initialState,
    initialErrorString,
    openSetupOrDefaultPromptType,
    onSuccess,
}) => {
    const styles = useStyles();

    const isUseCustomImageByDefault = useApplicationStore(
        (state) => state.isUseCustomImageByDefault,
    );
    const setUseCustomImageByDefaultInStore = useApplicationStore(
        (state) => state.setUseCustomImageByDefault,
    );
    const storedCustomImageVersion = useApplicationStore(
        (state) => state.customImageVersion,
    );
    const setStoredCustomImageVersion = useApplicationStore(
        (state) => state.setCustomImageVersion,
    );

    const [dialogOpen, setDialogOpen] = useState(true);
    const [machineState, setMachineState] = useState(initialState);
    const [promptType, setPromptType] = useState<OpenSetupOrDefaultPromptType>(
        openSetupOrDefaultPromptType || "LocalNoImage",
    );
    const [errorString, setErrorString] = useState(initialErrorString || "");

    // setup dialog states
    const [isCustomImageSelected, setIsCustomImageSelected] = useState(true);
    const [customImageUpload, setCustomImageUpload] = useState<
        CustomImageFile | undefined
    >(undefined);
    const [uploadError, setUploadError] = useState(false);
    const [isUseCurrentImageSelected, setIsUseCurrentImageSelected] =
        useState(false);
    const [isUseByDefaultSelected, setIsUseByDefaultSelected] = useState(
        isUseCustomImageByDefault,
    );
    const [isDeletePreviousSelected, setIsDeletePreviousSelected] =
        useState(false);
    const openSetupDialog = async () => {
        setDialogOpen(false);
        setIsCustomImageSelected(true);
        setCustomImageUpload(undefined);
        setIsUseCurrentImageSelected(false);
        setIsUseByDefaultSelected(isUseCustomImageByDefault);
        setIsDeletePreviousSelected(false);
        await waitForDialogClose();
        setMachineState("SetupDialog");
    };

    useEffect(() => {
        setDialogOpen(machineState !== "Initializing");
    }, [machineState]);

    const t = useUITranslation();

    if (machineState === "Initializing") {
        return null;
    }

    if (machineState === "Error") {
        // unrecoverable error, the only option is to reboot the app
        return (
            <Dialog modalType="alert" open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{t("title.error")}</DialogTitle>
                        <DialogContent>
                            <p>{t("prompt.boot.error")}</p>
                            <p>{errorString}</p>
                        </DialogContent>
                        <DialogActions>
                            <Button
                                appearance="primary"
                                onClick={() => window.location.reload()}
                            >
                                {t("button.refresh")}
                            </Button>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        );
    }

    // function to start booting
    const bootCallback = async (args: RuntimeInitArgs) => {
        // safety check to make sure this is only called once
        if (bootScreenSuccessCalled) {
            console.warn("[boot] bootCallback called multiple times!!");
            return;
        }
        const result = await initRuntime(await runtime, args);
        if (bootScreenSuccessCalled) {
            console.warn("[boot] bootCallback called multiple times!!");
            return;
        }
        if (result.err) {
            setErrorString(result.err);
            if (args.isCustomImage) {
                console.log("[boot] failed to initialize custom image");
                // if failed to initialize custom image,
                // give the option to re-setup or use default
                setMachineState("OpenSetupOrUseDefaultImage");
                setPromptType("InitializeError");
            } else {
                console.log("[boot] failed to initialize default image");
                // if failed to initialize default image, show error
                // and don't retry
                setMachineState("Error");
            }
            return;
        }
        bootScreenSuccessCalled = true;
        onSuccess();
    };

    const isUseCustomOrDefault = machineState === "UseCustomOrUseDefaultImage";
    if (isUseCustomOrDefault || machineState === "OpenSetupOrUseDefaultImage") {
        return (
            <Dialog modalType="alert" open={dialogOpen}>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{t("title.custom_image")}</DialogTitle>
                        <DialogContent>
                            <p>
                                {t(
                                    isUseCustomOrDefault
                                        ? "prompt.boot.custom_or_default"
                                        : `prompt.boot.setup_or_default.${promptType}`,
                                    {
                                        new_version:
                                            storedCustomImageVersion ||
                                            "default",
                                        old_version:
                                            scriptImageVersion || "default",
                                    },
                                )}
                            </p>
                            <Link
                                href="https://ist.pistonite.dev/user/custom_image"
                                target="_blank"
                            >
                                {t("prompt.boot.button.learn_more")}
                            </Link>
                        </DialogContent>
                        <DialogActions>
                            <DialogTrigger disableButtonEnhancement>
                                {isUseCustomOrDefault ? (
                                    <Button
                                        appearance="primary"
                                        onClick={async () => {
                                            setDialogOpen(false);
                                            await waitForDialogClose();
                                            setMachineState("Initializing");
                                            await bootCallback({
                                                isCustomImage: true,
                                                params,
                                            });
                                        }}
                                    >
                                        {t("button.allow")}
                                    </Button>
                                ) : (
                                    <Button
                                        appearance="primary"
                                        onClick={openSetupDialog}
                                    >
                                        {t(
                                            promptType === "InitializeError"
                                                ? "prompt.boot.setup_or_default.button.setup_again"
                                                : "button.setup",
                                        )}
                                    </Button>
                                )}
                            </DialogTrigger>
                            <DialogTrigger disableButtonEnhancement>
                                <Button
                                    appearance="secondary"
                                    onClick={async () => {
                                        setDialogOpen(false);
                                        await waitForDialogClose();
                                        setMachineState("Initializing");
                                        await bootCallback({
                                            isCustomImage: false,
                                            deleteCustomImage: false,
                                        });
                                    }}
                                >
                                    {t(
                                        "prompt.boot.setup_or_default.button.use_default",
                                    )}
                                </Button>
                            </DialogTrigger>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        );
    }

    const enableSetupDialogLaunchButton = !!(
        !isCustomImageSelected ||
        (isUseCurrentImageSelected
            ? storedCustomImageVersion
            : customImageUpload)
    );

    return (
        <Dialog modalType="alert" open={dialogOpen}>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t("dialog.custom_image.title")}</DialogTitle>
                    <DialogContent>
                        <Field
                            label={t("dialog.custom_image.field.select_type")}
                        >
                            <RadioGroup
                                layout="horizontal"
                                value={
                                    isCustomImageSelected ? "custom" : "default"
                                }
                                onChange={(_, { value }) => {
                                    setIsCustomImageSelected(
                                        value === "custom",
                                    );
                                }}
                            >
                                <Radio
                                    value="custom"
                                    label={t(
                                        "dialog.custom_image.field.select_type.custom",
                                    )}
                                />
                                <Radio
                                    value="default"
                                    label={t(
                                        "dialog.custom_image.field.select_type.default",
                                    )}
                                />
                            </RadioGroup>
                        </Field>
                        <div className={styles.spacer} />
                        {isCustomImageSelected && (
                            <>
                                <Field
                                    validationState={
                                        uploadError ? "error" : "none"
                                    }
                                    validationMessage={
                                        uploadError
                                            ? t(
                                                  "dialog.custom_image.button.select_file.error",
                                              )
                                            : "\u00a0"
                                    }
                                >
                                    <div className={styles.fileUploadSection}>
                                        <Button
                                            appearance="primary"
                                            onClick={async () => {
                                                console.log(
                                                    "[boot] selecting custom image",
                                                );
                                                const result =
                                                    await selectImageFile();
                                                if (result.type === "select") {
                                                    setCustomImageUpload(
                                                        result.file,
                                                    );
                                                    setUploadError(false);
                                                    return;
                                                }
                                                if (result.type === "cancel") {
                                                    setUploadError(false);
                                                    return;
                                                }
                                                setCustomImageUpload(undefined);
                                                setUploadError(true);
                                            }}
                                            disabled={isUseCurrentImageSelected}
                                        >
                                            {t(
                                                "dialog.custom_image.button.select_file",
                                            )}
                                        </Button>
                                        {customImageUpload && (
                                            <Text>
                                                {customImageUpload.name}
                                            </Text>
                                        )}
                                    </div>
                                </Field>
                                <Field>
                                    <Checkbox
                                        checked={!!isUseCurrentImageSelected}
                                        disabled={!storedCustomImageVersion}
                                        onChange={(_, { checked }) =>
                                            setIsUseCurrentImageSelected(
                                                !!checked,
                                            )
                                        }
                                        label={t(
                                            "dialog.custom_image.option.select_current",
                                        )}
                                    />
                                </Field>
                                <div className={styles.spacer} />
                                <Field>
                                    <Checkbox
                                        checked={!!isUseByDefaultSelected}
                                        onChange={(_, { checked }) =>
                                            setIsUseByDefaultSelected(!!checked)
                                        }
                                        label={t(
                                            "dialog.custom_image.option.custom_by_default",
                                        )}
                                    />
                                </Field>
                            </>
                        )}
                        {!isCustomImageSelected && (
                            <>
                                <Field>
                                    <Checkbox
                                        checked={!!isDeletePreviousSelected}
                                        onChange={(_, { checked }) =>
                                            setIsDeletePreviousSelected(
                                                !!checked,
                                            )
                                        }
                                        label={t(
                                            "dialog.custom_image.option.delete_previous",
                                        )}
                                    />
                                </Field>
                            </>
                        )}
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button
                                appearance={
                                    enableSetupDialogLaunchButton
                                        ? "primary"
                                        : "outline"
                                }
                                disabled={!enableSetupDialogLaunchButton}
                                onClick={async () => {
                                    if (!enableSetupDialogLaunchButton) {
                                        return;
                                    }
                                    console.log(
                                        "[boot] continuing from setup dialog",
                                    );
                                    setDialogOpen(false);
                                    await waitForDialogClose();
                                    setMachineState("Initializing");
                                    if (isCustomImageSelected) {
                                        if (customImageUpload) {
                                            setCustomImageToProvide(
                                                customImageUpload.bytes,
                                            );
                                        }
                                        setUseCustomImageByDefaultInStore(
                                            isUseByDefaultSelected,
                                        );
                                        const args: RuntimeInitArgs = {
                                            isCustomImage: true,
                                            params,
                                        };
                                        await bootCallback(args);
                                        return;
                                    }
                                    if (isDeletePreviousSelected) {
                                        setStoredCustomImageVersion("");
                                    }
                                    const args: RuntimeInitArgs = {
                                        isCustomImage: false,
                                        deleteCustomImage:
                                            isDeletePreviousSelected,
                                    };
                                    await bootCallback(args);
                                }}
                            >
                                {t("button.launch")}
                            </Button>
                        </DialogTrigger>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
};

async function waitForDialogClose() {
    await new Promise((resolve) => setTimeout(resolve, 100));
}

type CustomImageFile = {
    name: string;
    bytes: Uint8Array;
};

type CustomImageSelection =
    | {
          type: "cancel" | "error";
      }
    | {
          type: "select";
          file: CustomImageFile;
      };

async function selectImageFile(): Promise<CustomImageSelection> {
    const file = await fsOpenFile({
        id: "skybook-custom-image",
        types: [
            {
                description: translateUI(
                    "dialog.custom_image.button.select_file.type_desc",
                ),
                accept: [".blfm"],
            },
        ],
    });
    if (file.err) {
        if (file.err.code === FsErr.UserAbort) {
            return { type: "cancel" };
        }
        console.error(`[boot] failed to open file: ${file.err.message}`);
        return { type: "error" };
    }
    const name = file.val.name;
    console.log("[boot] reading selected file");
    const bytes = await file.val.getBytes();
    if (bytes.err) {
        console.error(`[boot] failed to read file: ${bytes.err.message}`);
        return { type: "error" };
    }
    return { type: "select", file: { name, bytes: bytes.val } };
}
