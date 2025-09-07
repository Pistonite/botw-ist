import { makeStyles } from "@fluentui/react-components";

import { ModifierSprite, type PouchCategory, PouchCategoryNames } from "@pistonite/skybook-itemsys";

import { useStyleEngine } from "self::util";

export type InventoryTabButtonProps = {
    category: PouchCategory;
    onClick?: () => void;
};

const useStyles = makeStyles({
    innerContainer: {
        width: "48px",
        height: "48px",
        filter: "brightness(0.6)",
        borderBottom: "2px solid #fff",
        "&:hover": {
            filter: "drop-shadow(0 0 2px #ccc)",
        },
    },
});

export const InventoryTabButton: React.FC<InventoryTabButtonProps> = ({ category, onClick }) => {
    const m = useStyleEngine();
    const c = useStyles();
    return (
        <div className={m("flex-noshrink border-box cursor-pointer")} onClick={onClick}>
            <div className={m("flex flex-center", c.innerContainer)}>
                <ModifierSprite
                    status={`Category${PouchCategoryNames[category as number]}`}
                    size={24}
                />
            </div>
        </div>
    );
};
