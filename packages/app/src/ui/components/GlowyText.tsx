import { Text, type TextProps } from "@fluentui/react-components";

import { useStyleEngine } from "#util";

const useStyles = useStyleEngine.extend({
    text: {
        fontFamily: "CalamitySans",
        fontSynthesis: "initial",
        textShadow: "0 0 5px #3aa0ff, 0 0 5px #3aa0ff, 0 0 5px #3aa0ff",
    },
    dark: {
        color: "#b7f1ff",
    },
    light: {
        color: "#000000",
    },
});

export type GlowyTextProps = TextProps & {
    dark?: boolean;
};

export const GlowyText: React.FC<GlowyTextProps> = ({ children, dark, ...props }) => {
    const m = useStyles();
    return (
        <Text className={m("c-text", dark ? "c-dark" : "c-light")} {...props}>
            {children}
        </Text>
    );
};
