import { Button, Dropdown, Option, Tooltip, makeStyles, mergeClasses } from "@fluentui/react-components";
import { Dismiss20Regular, WindowNew20Regular } from "@fluentui/react-icons";

import { useUITranslation } from "skybook-localization";

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

    /**
     * If enabled, will add flex: 1 to the container and dropdown container
     * so it's full width in the flex box parent
     */
    fullWidth?: boolean;
};

const useStyles = makeStyles({
    container: {
        display: "flex",
        flexDirection: "row",
        gap: "4px",
    },
    fullWidth: {
        flex: 1,
        },
    selectorButton: {
        // truncate text
        overflowX: "hidden",
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
        maxWidth: "200px",
    }
});

export const ExtensionToolbar: React.FC<ExtensionToolbarProps> = ({ 
    id, allIds, onClickPopout, onClickClose, onSelect, fullWidth
}) => {
    const t = useUITranslation();
    const styles = useStyles();
    return (
        <div className={mergeClasses(styles.container, fullWidth && styles.fullWidth)}>
            <Dropdown
                className={mergeClasses(fullWidth && styles.fullWidth)}
                appearance="filled-darker"
                button={
                    <span
                        className={styles.selectorButton}
                    >
                        {t(`extension.${id}.name`)}
                    </span>
                }
                selectedOptions={[id]}
                onOptionSelect={(_, {optionValue}) => {
                    if (optionValue && optionValue !== id) {
                        onSelect(optionValue);
                    }
                }}
            >
                {
                    allIds.map((id, i) => (
                        <Tooltip key={i}
                            withArrow
                            positioning="after"
                            content={t(`extension.${id}.desc`)} 
                            relationship="description">
                            <Option value={id}>
                                {t(`extension.${id}.name`)}
                            </Option>
                        </Tooltip>
                    ))
                }
            </Dropdown>
            { onClickPopout && (
                <Tooltip content={t("button.popout")} relationship="label">
                    <Button 
                        onClick={onClickPopout}
                        icon={<WindowNew20Regular />}
                        appearance="subtle"
                    />
                </Tooltip> )
            }
            {
                onClickClose && (
                    <Tooltip content={t("button.close")} relationship="label">
                        <Button 
                            onClick={onClickClose}
                            appearance="subtle"
                            icon={<Dismiss20Regular  />}
                        />
                    </Tooltip>
                )
            }
        </div>

    );
};
