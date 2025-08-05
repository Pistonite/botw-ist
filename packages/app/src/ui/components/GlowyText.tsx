import { Text, makeStyles, type TextProps, mergeClasses } from "@fluentui/react-components";

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

export type GlowyTextProps = TextProps & {
    dark?: boolean;
};

export const GlowyText: React.FC<GlowyTextProps> = ({ children, dark, ...props }) => {
    const styles = useStyles();
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
