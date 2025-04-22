import type { PropsWithChildren } from "react";
import {
    Spinner,
    Tooltip,
    makeStyles,
    mergeClasses,
} from "@fluentui/react-components";
import { Info16Regular } from "@fluentui/react-icons";
import { useDark } from "@pistonite/pure-react";

import { GlowyText } from "./GlowyText.tsx";

const useStyles = makeStyles({
    title: {
        margin: "0 4px",
        display: "flex",
        gap: "2px",
    },
    titleColorDark: {
        color: "#b7f1ff",
    },
    titleColorLight: {
        color: "#000000",
    },
    supertitle: {
        display: "inline-flex",
        alignItems: "start",
        gap: "2px",
    },
    infoIcon: {
        paddingTop: "1px",
    },
    container: {
        display: "flex",
        flexDirection: "row",
        alignItems: "center",
        padding: "4px 4px 8px 0px",
        gap: "4px",
    },
    end: {
        flexGrow: 1,
    },
    loadingSpinner: {
        justifyContent: "flex-end",
    },
});

export type InventoryTitleProps = {
    /** Title of the section of the inventory. */
    title: string;
    /** Description of the section of the inventory. */
    description?: string;
    /** Super text after the title and description tooltip icon */
    supertitle?: JSX.Element;
    /** Whether to display a loading spinner. */
    loading?: boolean;
};

/** Header of the inventory display section. */
export const InventoryTitle: React.FC<
    PropsWithChildren<InventoryTitleProps>
> = ({ title, description, supertitle, loading, children }) => {
    const styles = useStyles();
    const dark = useDark();
    return (
        <div className={styles.container}>
            <span
                className={mergeClasses(
                    styles.title,
                    dark ? styles.titleColorDark : styles.titleColorLight,
                )}
            >
                <GlowyText size={500} weight="bold">
                    {title}
                </GlowyText>
                <span className={styles.supertitle}>
                    {description && (
                        <Tooltip relationship="label" content={description}>
                            <Info16Regular className={styles.infoIcon} />
                        </Tooltip>
                    )}
                    {supertitle}
                </span>
            </span>
            {children}
            <span className={styles.end}>
                {loading && (
                    <Spinner
                        className={styles.loadingSpinner}
                        as="span"
                        size="tiny"
                        delay={300}
                    />
                )}
            </span>
        </div>
    );
};
