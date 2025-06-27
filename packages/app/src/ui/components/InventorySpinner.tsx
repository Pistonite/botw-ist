import { makeStyles, Spinner } from "@fluentui/react-components";
import { useDebounce } from "@uidotdev/usehooks";

import { useStyleEngine } from "self::ui/functions";

export type InventorySpinnerProps = {
    show?: boolean;
};

const useStyles = makeStyles({
    container: {
        // padding: "0 4px",
        backgroundColor: "#00000044",
        zIndex: 100,
    },
});

export const InventorySpinner: React.FC<InventorySpinnerProps> = ({ show }) => {
    const m = useStyleEngine();
    const c = useStyles();
    const showReal = useDebounce(show, 300);
    if (!show || !showReal) {
        return null;
    }

    return (
        <div className={m("pos-abs all-sides-0 flex flex-center", c.container)}>
            <Spinner
                // className={m("flex-end")}
                as="span"
                size="medium"
                // delay={300}
            />
        </div>
    );
};
