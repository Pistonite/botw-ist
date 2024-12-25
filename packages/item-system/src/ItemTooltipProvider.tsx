import { type PropsWithChildren, createContext, useRef, useState, useCallback, useEffect, useContext } from "react";
import { makeStyles, mergeClasses } from "@griffel/react";

import { useStaticAssetStyles } from "skybook-item-assets";

import type { ItemSlotInfo } from "./ItemSlotInfo.ts";
import { ItemTooltipContent } from "./ItemTooltipContent.tsx";

export type SetItemTooltipFn = (x: number, y: number, info: ItemSlotInfo | undefined) => void;

const useStyles = makeStyles({
    container: {
        position: "absolute",
    }
});

const ItemTooltipContext = createContext<SetItemTooltipFn>(() => {
    /* empty */
});

/** Provider for the ItemTooltipContext */
export const ItemTooltipProvider: React.FC<PropsWithChildren> = ({ children }) => {
    const staticAssets = useStaticAssetStyles();
    const styles = useStyles();

    const toolTipDivRef = useRef<HTMLDivElement>(null);
    const [tooltipInfo, setTooltipInfo] = useState<ItemSlotInfo | undefined>();
    const [tooltipX, setTooltipX] = useState<number>(0);
    const [tooltipY, setTooltipY] = useState<number>(0);
    const setTooltip: SetItemTooltipFn = useCallback(
        (x , y, info) => {
            setTooltipX(x + 10);
            setTooltipY(y + 10);
            setTooltipInfo(info);
        },
        [],
    );

    useEffect(() => {
        if (toolTipDivRef.current) {
            const rect = toolTipDivRef.current.getBoundingClientRect();
            if (rect.bottom > window.innerHeight) {
                setTooltipY(tooltipY - rect.height - 20);
            }
            if (rect.right > window.innerWidth) {
                setTooltipX(tooltipX - rect.width - 20);
            }
        }
    }, [
        tooltipX,
        tooltipY,
        toolTipDivRef.current &&
            toolTipDivRef.current.getBoundingClientRect().width,
        toolTipDivRef.current &&
            toolTipDivRef.current.getBoundingClientRect().height,
    ]);

    return (
        <ItemTooltipContext.Provider value={setTooltip}>
            {children}
            {!!tooltipInfo && (
                <div
                    ref={toolTipDivRef}
                    className={mergeClasses(staticAssets.sheikahBg, styles.container)}
                    style={{
                        left: tooltipX,
                        top: tooltipY,
                    }}
                >
                    <ItemTooltipContent info={tooltipInfo} />
                </div>
            )}
        </ItemTooltipContext.Provider>
    );
};

export const useSetItemTooltip = () => {
    return useContext(ItemTooltipContext);
}
