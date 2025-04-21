import type { PropsWithChildren } from "react";
import { Text, makeStyles, mergeClasses } from "@fluentui/react-components";
import { useDark } from "@pistonite/pure-react";

const useStyles = makeStyles({
    base: {
        padding: "0 2px",
    },
    dark: {
        backgroundColor: "#292c3c",
        color: "#ef9f76"
    },
    light: {
        backgroundColor: "#e6e9ef",
        color: "#e64553"
    }
});

/** Inline code text */
export const Code: React.FC<PropsWithChildren> = ({children}) => {
    const styles = useStyles();
    const dark = useDark();
    return (
        <Text 
            className={mergeClasses(styles.base, dark?styles.dark:styles.light)}
            font="monospace"
        >
        {children}
    </Text>
    );
}
