import type { PropsWithChildren } from "react";
import { FluentProvider, webDarkTheme, webLightTheme } from "@fluentui/react-components";

import { useStyleEngine } from "#util";

import { GlowyText } from "./GlowyText.tsx";

const useStyles = useStyleEngine.extend({
    title: {
        margin: "0 4px",
    },
    "title-dark": {
        color: "#b7f1ff",
    },
    "title-light": {
        color: "#000000",
    },
    container: {
        padding: "4px 0px 8px 0px",
    },
    "no-bg": {
        backgroundColor: "transparent",
    },
});

export type InventoryTitleProps = {
    /** Title of the section of the inventory. */
    title: string;
    /** Whether to display the title in dark mode theme (which means the title is light-colored) */
    dark?: boolean;
};

/** Header of the inventory display section. */
export const InventoryTitle: React.FC<PropsWithChildren<InventoryTitleProps>> = ({
    title,
    dark,
    children,
}) => {
    const m = useStyles();
    return (
        <FluentProvider className={m("c-no-bg")} theme={dark ? webDarkTheme : webLightTheme}>
            <div className={m("flex-row flex-centera gap-2 c-container")}>
                <span className={m("flex gap-2", dark ? "c-title-dark" : "c-title-light")}>
                    <GlowyText size={500} weight="bold" dark={dark}>
                        {title}
                    </GlowyText>
                </span>
                {children}
            </div>
        </FluentProvider>
    );
};
