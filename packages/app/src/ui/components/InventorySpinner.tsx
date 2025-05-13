import { makeStyles, Spinner } from "@fluentui/react-components";

import { useStyleEngine } from "self::ui/functions";

export type InventorySpinnerProps = {
    show?: boolean;
};

const useStyles = makeStyles({
    container: {
        padding: "0 4px",
    },
});

export const InventorySpinner: React.FC<InventorySpinnerProps> = ({ show }) => {
    const m = useStyleEngine();
    const c = useStyles();
    if (!show) {
        return null;
    }

    return (
        <div className={m("flex-grow", c.container)}>
            <Spinner
                className={m("flex-end")}
                as="span"
                size="tiny"
                delay={300}
            />
        </div>
    );
};
