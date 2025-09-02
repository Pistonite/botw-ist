import { makeStyles, mergeClasses } from "@fluentui/react-components";
import { PresenceBlocked24Regular } from "@fluentui/react-icons";

import { ModifierSprite } from "../sprite";
import { PouchCategory, PouchCategoryNames } from "../data";

export type TabNode = {
    /** The slot number of the item in the tab, corresponding to where it will be displayed */
    slot: number;
    /** The node to render for this item */
    element: React.ReactNode;
};

export type ItemTabProps = {
    /** The category icon to display. PouchCategory.Invalid will not display anything. */
    category: PouchCategory;
    /** Show border around the item tab. */
    border?: boolean;
    /** Stuff to display in the tab */
    nodes: TabNode[];
    /** Whether the tab is undiscovered (not accessible in game UI) */
    undiscovered?: boolean;
};

const useStyles = makeStyles({
    container: {
        padding: "0 8px",
        boxSizing: "border-box",
        width: "376px",
    },
    categoryIcon: {
        filter: "drop-shadow(0 0 2px #ccc)",
        display: "flex",
        flexDirection: "row",
        justifyContent: "center",
        padding: "8px 0",
        margin: "0 16px",
        boxSizing: "border-box",
    },
    categoryIconInner: {
        position: "relative",
    },
    blockIcon: {
        position: "absolute",
        top: "4px",
        left: "4px",
        right: "4px",
        bottom: "4px",
        color: "red",
    },
    iconHeight: {
        height: "32px",
    },

    borderVisible: {
        borderBottom: "1px solid #ccc",
    },
    borderHidden: {
        borderBottom: "1px solid transparent",
    },
    itemContainer: {
        position: "relative",
        width: "360px",
    },
});

export const ItemTab: React.FC<ItemTabProps> = ({ category, border, nodes, undiscovered }) => {
    const styles = useStyles();

    const $CategoryIcon = category !== PouchCategory.Invalid && (
        <span className={styles.categoryIconInner}>
            <ModifierSprite status={`Category${PouchCategoryNames[category]}`} size={32} />
            {undiscovered && (
                <span className={styles.blockIcon}>
                    <PresenceBlocked24Regular />
                </span>
            )}
        </span>
    );
    let height = 0;
    const $Nodes = nodes.map(({ slot, element }) => {
        const h = Math.floor(slot / 5) * 72;
        if (h + 72 > height) {
            height = h + 72;
        }
        return (
            <div
                key={slot}
                style={{
                    position: "absolute",
                    top: `${h}px`,
                    left: `${(slot % 5) * 72}px`,
                }}
            >
                {element}
            </div>
        );
    });

    return (
        <div className={styles.container}>
            <div
                className={mergeClasses(
                    styles.categoryIcon,
                    border ? styles.borderVisible : styles.borderHidden,
                )}
            >
                <div className={mergeClasses(border && styles.iconHeight)}>{$CategoryIcon}</div>
            </div>
            <div className={styles.itemContainer} style={{ height: `${height}px` }}>
                {$Nodes}
            </div>
        </div>
    );
};
