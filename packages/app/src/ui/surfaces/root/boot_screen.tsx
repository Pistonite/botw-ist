/**
 * This is the UI for boot flow with custom image configuration dialog
 */
import { useEffect, useMemo, useState } from "react";
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
    Translator,
    Runtime,
    RuntimeWorkerInitArgs,
    RuntimeInitParams,
} from "@pistonite/skybook-api";
import { translateUI, useUITranslation } from "skybook-localization";

import { initRuntime, setCustomImageToProvide, usePersistStore } from "self::application";
import { bootLog, useStyleEngine } from "self::util";
import { ErrorBar } from "self::ui/components";

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
    runtime: Promise<Runtime>;
    /** The version of the script image, read from env block */
    scriptImageVersion?: string;
    /** Params from the script */
    params: RuntimeInitParams;
    /** State of the boot flow when initially showing the screen */
    initialState: BootScreenState;
    /**
     * Initial localized error string, if the state is "InitializeError"
     * This is a function because some errors needs to be localized,
     * which isn't available yet when the error occured
     */
    initialErrorString?: (translator: Translator) => string;
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
    const m = useStyleEngine();
    const c = useStyles();

    const isUseCustomImageByDefault = usePersistStore((state) => state.isUseCustomImageByDefault);
    const setUseCustomImageByDefaultInStore = usePersistStore(
        (state) => state.setUseCustomImageByDefault,
    );
    const storedCustomImageVersion = usePersistStore((state) => state.customImageVersion);
    const setStoredCustomImageVersion = usePersistStore((state) => state.setCustomImageVersion);

    const [dialogOpen, setDialogOpen] = useState(true);
    const [machineState, setMachineState] = useState(initialState);
    const [promptType, setPromptType] = useState<OpenSetupOrDefaultPromptType>(
        openSetupOrDefaultPromptType || "LocalNoImage",
    );
    const t = useUITranslation();
    const [errorStringGetter, setErrorStringGetter] = useState<
        ((translator: Translator) => string) | undefined
    >(() => initialErrorString);
    const errorString = useMemo(() => {
        if (!errorStringGetter) {
            return undefined;
        }
        return errorStringGetter(t);
    }, [errorStringGetter, t]);

    // setup dialog states
    const [isCustomImageSelected, setIsCustomImageSelected] = useState(true);
    const [customImageUpload, setCustomImageUpload] = useState<CustomImageFile | undefined>(
        undefined,
    );
    const [uploadError, setUploadError] = useState(false);
    const [isUseCurrentImageSelected, setIsUseCurrentImageSelected] = useState(false);
    const [isUseByDefaultSelected, setIsUseByDefaultSelected] = useState(isUseCustomImageByDefault);
    const [isDeletePreviousSelected, setIsDeletePreviousSelected] = useState(false);
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

    if (machineState === "Initializing") {
        return null;
    }

    if (machineState === "Error") {
        // unrecoverable error, the only option is to reboot the app
        // or, let the user setup the image
        return (
            <Dialog modalType="alert" open>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{t("title.error")}</DialogTitle>
                        <DialogContent>
                            <p>{t("prompt.boot.error")}</p>
                            <ErrorBar>{errorString}</ErrorBar>
                        </DialogContent>
                        <DialogActions>
                            <Button appearance="primary" onClick={() => window.location.reload()}>
                                {t("button.refresh")}
                            </Button>
                            <DialogTrigger disableButtonEnhancement>
                                <Button appearance="secondary" onClick={openSetupDialog}>
                                    {t("button.setup")}
                                </Button>
                            </DialogTrigger>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        );
    }

    // function to start booting
    const bootCallback = async (args: RuntimeWorkerInitArgs) => {
        // safety check to make sure this is only called once
        if (bootScreenSuccessCalled) {
            bootLog.warn("bootCallback called multiple times!!");
            return;
        }
        const result = await initRuntime(await runtime, args);
        if (bootScreenSuccessCalled) {
            bootLog.warn("bootCallback called multiple times!!");
            return;
        }
        if (result.err) {
            setErrorStringGetter(() => result.err);
            if (args.isCustomImage) {
                bootLog.error("failed to initialize custom image");
                // if failed to initialize custom image,
                // give the option to re-setup or use default
                setMachineState("OpenSetupOrUseDefaultImage");
                setPromptType("InitializeError");
            } else {
                bootLog.error("failed to initialize default image");
                // if failed to initialize default image, show error
                // and don't retry
                setMachineState("Error");
            }
            bootLog.error(result.err);
            return;
        }
        bootScreenSuccessCalled = true;
        onSuccess();
    };

    const isUseCustomOrDefault = machineState === "UseCustomOrUseDefaultImage";
    if (isUseCustomOrDefault || machineState === "OpenSetupOrUseDefaultImage") {
        const bodyTranslationArgs = {
            new_version: storedCustomImageVersion || "default",
            old_version: scriptImageVersion || "default",
        };
        const $Body = (
            <>
                <p>
                    {t(
                        isUseCustomOrDefault
                            ? "prompt.boot.custom_or_default"
                            : `prompt.boot.setup_or_default.${promptType}`,
                        bodyTranslationArgs,
                    )}
                </p>
                {promptType === "InitializeError" && <ErrorBar>{errorString}</ErrorBar>}
                <Link href="https://skybook.pistonite.dev/user/custom_image" target="_blank">
                    {t("prompt.boot.button.learn_more")}
                </Link>
            </>
        );
        const $ContinueButton = isUseCustomOrDefault ? (
            <Button
                appearance="primary"
                onClick={async () => {
                    setDialogOpen(false);
                    await waitForDialogClose();
                    setMachineState("Initializing");
                    await bootCallback({
                        isCustomImage: true,
                        params,
                        alwaysAskApp: customImageUpload !== undefined,
                    });
                }}
            >
                {t("button.allow")}
            </Button>
        ) : (
            <Button appearance="primary" onClick={openSetupDialog}>
                {t(
                    promptType === "InitializeError"
                        ? "prompt.boot.setup_or_default.button.setup_again"
                        : "button.setup",
                )}
            </Button>
        );
        const $SetupButton = (
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
                {t("prompt.boot.setup_or_default.button.use_default")}
            </Button>
        );

        return (
            <Dialog modalType="alert" open={dialogOpen}>
                <DialogSurface>
                    <DialogBody>
                        <DialogTitle>{t("title.custom_image")}</DialogTitle>
                        <DialogContent>{$Body}</DialogContent>
                        <DialogActions>
                            <DialogTrigger disableButtonEnhancement>
                                {$ContinueButton}
                            </DialogTrigger>
                            <DialogTrigger disableButtonEnhancement>{$SetupButton}</DialogTrigger>
                        </DialogActions>
                    </DialogBody>
                </DialogSurface>
            </Dialog>
        );
    }

    const enableSetupDialogLaunchButton = !!(
        !isCustomImageSelected ||
        (isUseCurrentImageSelected ? storedCustomImageVersion : customImageUpload)
    );

    const $SelectTypeRadioGroup = (
        <Field label={t("dialog.custom_image.field.select_type")}>
            <RadioGroup
                layout="horizontal"
                value={isCustomImageSelected ? "custom" : "default"}
                onChange={(_, { value }) => {
                    setIsCustomImageSelected(value === "custom");
                }}
            >
                <Radio value="custom" label={t("dialog.custom_image.field.select_type.custom")} />
                <Radio value="default" label={t("dialog.custom_image.field.select_type.default")} />
            </RadioGroup>
        </Field>
    );

    const $SelectImageFileButton = (
        <Button
            appearance="primary"
            onClick={async () => {
                bootLog.info("selecting custom image");
                const result = await selectImageFile();
                if (result.type === "select") {
                    setCustomImageUpload(result.file);
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
            {t("dialog.custom_image.button.select_file")}
        </Button>
    );

    const $UploadField = (
        <Field
            validationState={uploadError ? "error" : "none"}
            validationMessage={
                uploadError ? t("dialog.custom_image.button.select_file.error") : "\u00a0"
            }
        >
            <div className={m("flex-row flex-centera gap-8", c.fileUploadSection)}>
                {$SelectImageFileButton}
                {customImageUpload && <Text>{customImageUpload.name} </Text>}
            </div>
        </Field>
    );

    const $UseCurrentImageCheckbox = (
        <Field>
            <Checkbox
                checked={!!isUseCurrentImageSelected}
                disabled={!storedCustomImageVersion}
                onChange={(_, { checked }) => {
                    setIsUseCurrentImageSelected(!!checked);
                }}
                label={t("dialog.custom_image.option.select_current")}
            />
        </Field>
    );

    const $DeletePreviousCheckbox = (
        <Field>
            <Checkbox
                checked={!!isDeletePreviousSelected}
                onChange={(_, { checked }) => {
                    setIsDeletePreviousSelected(!!checked);
                }}
                label={t("dialog.custom_image.option.delete_previous")}
            />
        </Field>
    );

    const $UseCustomImageByDefaultCheckbox = (
        <Field>
            <Checkbox
                checked={!!isUseByDefaultSelected}
                onChange={(_, { checked }) => {
                    setIsUseByDefaultSelected(!!checked);
                }}
                label={t("dialog.custom_image.option.custom_by_default")}
            />
        </Field>
    );

    const doLaunch = async () => {
        if (!enableSetupDialogLaunchButton) {
            return;
        }
        bootLog.info("continuing from setup dialog");
        setDialogOpen(false);
        await waitForDialogClose();
        setMachineState("Initializing");
        if (isCustomImageSelected) {
            if (customImageUpload) {
                setCustomImageToProvide(customImageUpload.bytes);
            }
            setUseCustomImageByDefaultInStore(isUseByDefaultSelected);
            const args: RuntimeWorkerInitArgs = {
                isCustomImage: true,
                params,
                alwaysAskApp: customImageUpload !== undefined,
            };
            await bootCallback(args);
            return;
        }
        if (isDeletePreviousSelected) {
            setStoredCustomImageVersion("");
        }
        const args: RuntimeWorkerInitArgs = {
            isCustomImage: false,
            deleteCustomImage: isDeletePreviousSelected,
        };
        await bootCallback(args);
    };

    return (
        <Dialog modalType="alert" open={dialogOpen}>
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>{t("dialog.custom_image.title")}</DialogTitle>
                    <DialogContent>
                        {$SelectTypeRadioGroup}
                        <div className={c.spacer} />
                        {isCustomImageSelected && (
                            <>
                                {$UploadField}
                                {$UseCurrentImageCheckbox}
                                <div className={c.spacer} />
                                {$UseCustomImageByDefaultCheckbox}
                            </>
                        )}
                        {!isCustomImageSelected && $DeletePreviousCheckbox}
                    </DialogContent>
                    <DialogActions>
                        <DialogTrigger disableButtonEnhancement>
                            <Button
                                appearance={enableSetupDialogLaunchButton ? "primary" : "outline"}
                                disabled={!enableSetupDialogLaunchButton}
                                onClick={doLaunch}
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
                description: translateUI("dialog.custom_image.button.select_file.type_desc"),
                accept: [".bfi"],
            },
        ],
    });
    if (file.err) {
        if (file.err.code === FsErr.UserAbort) {
            return { type: "cancel" };
        }
        bootLog.error(`failed to open file: ${file.err.message}`);
        return { type: "error" };
    }
    const name = file.val.name;
    bootLog.info("reading selected file");
    const bytes = await file.val.getBytes();
    if (bytes.err) {
        bootLog.error(`failed to read file: ${bytes.err.message}`);
        return { type: "error" };
    }
    return { type: "select", file: { name, bytes: bytes.val } };
}
