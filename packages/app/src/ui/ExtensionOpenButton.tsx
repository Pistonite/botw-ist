import { Text, Button, Dialog, DialogActions, DialogBody, DialogContent, DialogSurface, DialogTitle, DialogTrigger, Tooltip, Field, Combobox } from "@fluentui/react-components";
import { WindowDevTools20Regular } from "@fluentui/react-icons";
import { ExtensionOpenMode } from "application/extensionStore";
import { useState } from "react";
import { useUITranslation } from "skybook-localization";

export const ExtensionOpenButton: React.FC = () => {
    const t = useUITranslation();

    const [selectedId, setSelectedId] = useState<string>("");
    const [selectedOpenMode, setSelectedOpenMode] = useState<ExtensionOpenMode>("secondary");
    const [isPersistChecked, setIsPersistChecked] = useState(true);


    return (
        <Dialog>
            <DialogTrigger disableButtonEnhancement>
                <Tooltip content={t("button.open_extension")} relationship="label">
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
                            <Combobox>
                            </Combobox>
                        </Field>
                        <Field label={t("field.select_extension")}>
                            <Combobox>
                            </Combobox>
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
