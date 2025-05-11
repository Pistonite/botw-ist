import { Spinner } from "@fluentui/react-components";

import { useStyleEngine } from "self::ui/functions";

export type InventorySpinnerProps = {
    show?: boolean;
};

export const InventorySpinner: React.FC<InventorySpinnerProps> = ({ show }) => {
    const m = useStyleEngine();
    if (!show) {
        return null;
    }

    return (
        <div className={m("flex-grow")}>
            <Spinner
                className={m("flex-end")}
                as="span"
                size="tiny"
                delay={300}
            />
        </div>
    );
};
