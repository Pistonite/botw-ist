import type { PropsWithChildren } from "react";
import {
    FluentProvider,
    webDarkTheme,
    webLightTheme,
} from "@fluentui/react-components";
import { useDark } from "@pistonite/pure-react";

export const ThemeProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const dark = useDark();
    const theme = dark ? webDarkTheme : webLightTheme;
    return <FluentProvider theme={theme}>{children}</FluentProvider>;
};
