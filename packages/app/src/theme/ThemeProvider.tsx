import { FluentProvider, makeStyles, webDarkTheme, webLightTheme } from "@fluentui/react-components";
import { useDark } from "@pistonite/pure-react";
import type { PropsWithChildren } from "react";


// const useStyles = makeStyles({
//     root: {
//         width: "100%",
//         height: "100%",
//     }
// });

export const ThemeProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const dark = useDark();
    console.log("dark", dark);
    // const styles = useStyles();
    const theme = dark ? webDarkTheme : webLightTheme;
    return (
        <FluentProvider theme={theme}>
            {children}
        </FluentProvider>
    );
}
