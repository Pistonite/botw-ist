import type { PropsWithChildren } from "react";
import { Text, type TextProps, makeStyles, mergeClasses } from "@fluentui/react-components";
import { useDark } from "@pistonite/pure-react";

const useStyles = makeStyles({
    base: {
        padding: "0 2px",
    },
    dark: {
        backgroundColor: "#292c3c",
        color: "#ef9f76",
    },
    light: {
        backgroundColor: "#e6e9ef",
        color: "#e64553",
    },
});

/** Inline code text */
export const Code: React.FC<PropsWithChildren<TextProps>> = ({ children, ...rest }) => {
    const c = useStyles();
    const dark = useDark();
    return (
        <Text className={mergeClasses(c.base, dark ? c.dark : c.light)} font="monospace" {...rest}>
            {children}
        </Text>
    );
};
