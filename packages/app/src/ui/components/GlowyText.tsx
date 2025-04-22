import {
    Text,
    makeStyles,
    type TextProps,
    mergeClasses,
} from "@fluentui/react-components";
import { useDark } from "@pistonite/pure-react";

const useStyles = makeStyles({
    text: {
        fontFamily: "CalamitySans",
        fontSynthesis: "initial",
        textShadow: "0 0 5px #3aa0ff, 0 0 5px #3aa0ff, 0 0 5px #3aa0ff",
    },
    glowColorDark: {
        color: "#b7f1ff",
    },
    glowColorLight: {
        color: "#000000",
    },
});

export const GlowyText: React.FC<TextProps> = ({ children, ...props }) => {
    const styles = useStyles();
    const dark = useDark();
    return (
        <Text
            className={mergeClasses(
                styles.text,
                dark ? styles.glowColorDark : styles.glowColorLight,
            )}
            {...props}
        >
            {children}
        </Text>
    );
};
