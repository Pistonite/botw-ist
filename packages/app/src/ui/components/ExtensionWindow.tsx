import { useQuery } from "@tanstack/react-query";
import { makeStyles } from "@fluentui/react-components";

import { connectExtensionToApp } from "self::application/extension";
import { getExtensionComponent } from "self::extensions";

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

export type ExtensionWrapperProps = {
    id: string;
};

/**
 * Component that wraps and loads an extension component with the given id.
 */
export const ExtensionWrapper: React.FC<ExtensionWrapperProps> = ({ id }) => {
    const { isPending, data: ExtComp } = useQuery({
        queryKey: ["extension", id],
        queryFn: () => {
            return getExtensionComponent(id);
        },
    });
    if (isPending || !ExtComp) {
        return <div>Loading...</div>;
    }
    return <ExtComp standalone={false} connect={connectExtensionToApp} />;
};
