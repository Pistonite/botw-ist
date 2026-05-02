import type { PropsWithChildren } from "react";
import { Text, type TextProps } from "@fluentui/react-components";
import { useDark } from "@pistonite/celera";

import { useStyleEngine } from "#util";

const useStyles = useStyleEngine.extend({
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
    const m = useStyles();
    const dark = useDark();
    return (
        <Text className={m("c-base", dark ? "c-dark" : "c-light")} font="monospace" {...rest}>
            {children}
        </Text>
    );
};
