import { makeStyles } from "@fluentui/react-components";
import { ExtensionWrapper } from "extensions/ExtensionWrapper.tsx";

export type ExtensionWindowProps = {
    /** Ids of the extensions loaded in this window */
    ids: string[];

    /** Id of the currently displayed extension */
    currentId: string;
};

const useStyles = makeStyles({
    container: {
        // take up all available space in parent container
        flex: 1,
        width: "100%",
        height: "100%",
        minHeight: "0px",
    },
    window: {
        width: "100%",
        height: "100%",
    },
});

export const ExtensionWindow: React.FC<ExtensionWindowProps> = ({
    ids,
    currentId,
}) => {
    const styles = useStyles();
    return (
        <div className={styles.container}>
            {ids.map((id, i) => (
                <div
                    key={i}
                    data-extension-id={id}
                    className={styles.window}
                    style={{
                        display: id === currentId ? "block" : "none",
                    }}
                >
                    <ExtensionWrapper id={id} />
                </div>
            ))}
        </div>
    );
};
