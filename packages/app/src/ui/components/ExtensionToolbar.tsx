import { Button, Dropdown, Option, Tooltip, makeStyles } from "@fluentui/react-components";
import { Dismiss20Regular, WindowNew20Regular } from "@fluentui/react-icons";
import type { PropsWithChildren } from "react";

import { useUITranslation } from "skybook-localization";

import { useStyleEngine } from "self::util";

export type ExtensionToolbarProps = {
    /** Id of the current opened extension */
    id: string;

    /**
     * Id of all extensions selectable from this toolbar
     *
     * The options will be displayed using the `extension.${id}.name` translation
     */
    allIds: string[];

    /**
     * Callback when the pop out button is pressed
     *
     * If not provided, the pop out button will be hidden
     */
    onClickPopout?: () => void;

    /**
     * Callback when the close button is pressed
     *
     * If not provided, the close button will be hidden
     */
    onClickClose?: () => void;

    /**
     * Callback when an extension is selected from the drop down
     *
     * Will not be invoked if the current extension is selected again
     * from the drop down
     */
    onSelect: (id: string) => void;
};

const useStyles = makeStyles({
    selectorButton: {
        // truncate text
        overflowX: "hidden",
        textOverflow: "ellipsis",
        whiteSpace: "nowrap",
        maxWidth: "200px",
    },
});

/**
 * The toolbar for selecting and controlling an extension window
 */
export const ExtensionToolbar: React.FC<PropsWithChildren<ExtensionToolbarProps>> = ({
    id,
    allIds,
    onClickPopout,
    onClickClose,
    onSelect,
    children,
}) => {
    const m = useStyleEngine();
    const t = useUITranslation();
    const c = useStyles();
    return (
        <div className={m("flex-row gap-4")}>
            <Dropdown
                className={m("flex-1")}
                appearance="filled-darker"
                button={<span className={c.selectorButton}>{t(`extension.${id}.name`)}</span>}
                selectedOptions={[id]}
                onOptionSelect={(_, { optionValue }) => {
                    if (optionValue && optionValue !== id) {
                        onSelect(optionValue);
                    }
                }}
            >
                {allIds.map((id, i) => (
                    <Option key={`${id}-${i}`} value={id}>
                        {t(`extension.${id}.name`)}
                    </Option>
                ))}
            </Dropdown>
            {onClickPopout && (
                <Tooltip content={t("button.popout")} relationship="label">
                    <Button
                        onClick={onClickPopout}
                        icon={<WindowNew20Regular />}
                        appearance="subtle"
                    />
                </Tooltip>
            )}
            {onClickClose && (
                <Tooltip content={t("button.close")} relationship="label">
                    <Button
                        onClick={onClickClose}
                        appearance="subtle"
                        icon={<Dismiss20Regular />}
                    />
                </Tooltip>
            )}
            {children}
        </div>
    );
};
