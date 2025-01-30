import { makeStyles } from "@fluentui/react-components";
import sheikahBg from "./SheikahBackground.png?url";

/** Get the styles for using static assets */
export const useStaticAssetStyles = makeStyles({
    /** Use the sheikah background image */
    sheikahBg: {
        backgroundImage: `url(${sheikahBg})`,
    },
});
