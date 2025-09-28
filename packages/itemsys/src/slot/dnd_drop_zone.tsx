import type { PropsWithChildren } from "react";
import { Button, makeStyles} from "@fluentui/react-components";

import type { ItemDragData } from "@pistonite/skybook-api";

import { useItemDrag } from "./dnd_context.ts";
import { dndLog as log } from "./dnd_util.ts";

export type ItemDropZoneProps = {
    /** Hint text displayed on the element while dragging in progress */
    getHint: (data: ItemDragData) => string;
    /** Callback when an item is dropped onto this target */
    onDropItem: (data: ItemDragData) => void
} & React.HTMLAttributes<HTMLDivElement>;

const useStyles = makeStyles({
    effectDiv: {
        position: "absolute",
        top: 0,
        left: 0,
        bottom: 0,
        right: 0,
        zIndex: 1000,
        boxSizing: "border-box",
        border: "2px dashed white",
        backgroundColor: "#ffffff11",
        willChange: "opacity",
        transitionDuration: "0.2s",
        pointerEvents: "none",
        display: "flex",
        flexDirection: "column",
        alignItems: "center",
    },
    hintDiv: 
                    {position: "relative", top: "-10px", opacity: 0.9}
});

/** Wrapper to make an area a drop target for dragging items */
export const ItemDropZone: React.FC<PropsWithChildren<ItemDropZoneProps>> = ({ 
    getHint,
    onDropItem, children, ...props }) => {
    const c = useStyles();
    const { data } = useItemDrag();

    return (<div {...props} style={{ position: "relative", ...props.style }}
        onDrop={() => {
            if (data) {
                log.info("dropping item");
                onDropItem(data);
            } else {
                log.warn("cannot drop item because it is undefined");
            }
        }}
    >
        {children}
        <div className={c.effectDiv} style={{opacity: data?1:0}} >
            {!!data &&
                <div className={c.hintDiv}>
                    <Button>
                        {getHint(data)}
                    </Button>
                </div>
            }
        </div>
    </div>);
};
