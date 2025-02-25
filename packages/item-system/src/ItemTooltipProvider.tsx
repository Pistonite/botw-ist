import {
    type PropsWithChildren,
    useRef,
    useState,
    useCallback,
    useEffect,
} from "react";
import { makeStyles, mergeClasses } from "@fluentui/react-components";

import type { ItemSlotInfo } from "@pistonite/skybook-api";

import { ItemTooltipContent } from "./ItemTooltipContent.tsx";
import {
    ItemTooltipContext,
    type SetItemTooltipFn,
} from "./ItemTooltipContext.ts";

const useStyles = makeStyles({
    container: {
        position: "absolute",
    },
    childrenContainer: {
        display: "contents",
    },
});

export type ItemTooltipProviderProps = {
    /** Url for the image for the background of the tooltip */
    backgroundUrl: string;
};

/** Provider for the ItemTooltipContext */
export const ItemTooltipProvider: React.FC<
    PropsWithChildren<ItemTooltipProviderProps>
> = ({ backgroundUrl, children }) => {
    const styles = useStyles();

    const tooltipDivRef = useRef<HTMLDivElement>(null);
    const childrenContainerRef = useRef<HTMLDivElement>(null);
    const [tooltipInfo, setTooltipInfo] = useState<ItemSlotInfo | undefined>();
    const [tooltipTarget, setTooltipTarget] = useState<
        HTMLElement | undefined
    >();
    const setTooltip: SetItemTooltipFn = useCallback((x, y, info, target) => {
        if (!tooltipDivRef.current) {
            return;
        }
        const tooltipDiv = tooltipDivRef.current;
        if (!info || !target) {
            tooltipDiv.style.display = "none";
            return;
        }
        tooltipDiv.style.display = "unset";
        // This might initially be wrong the first time
        // the info is changed. However, most of the time, it will be
        // called again with the correct x and y when the mouse moves.
        positionTooltipDiv(tooltipDiv, x, y);
        setTooltipInfo(info);
        setTooltipTarget(target);
    }, []);

    // hide the tooltip if the target is removed
    useEffect(() => {
        if (
            !childrenContainerRef.current ||
            !tooltipTarget ||
            !tooltipTarget.isConnected
        ) {
            return;
        }
        const observer = new MutationObserver(() => {
            if (!tooltipTarget) {
                observer.disconnect();
                return;
            }
            if (!tooltipTarget.isConnected) {
                observer.disconnect();
                setTooltipInfo(undefined);
            }
        });
        observer.observe(childrenContainerRef.current, {
            childList: true,
            subtree: true,
        });
        return () => {
            observer.disconnect();
        };
    }, [tooltipTarget]);

    return (
        <ItemTooltipContext.Provider
            value={{
                setItemTooltip: setTooltip,
                tooltipTarget,
            }}
        >
            <div ref={childrenContainerRef}>{children}</div>
            <div
                ref={tooltipDivRef}
                className={styles.container}
                style={{
                    backgroundImage: `url(${backgroundUrl})`,
                }}
            >
                {tooltipInfo && <ItemTooltipContent info={tooltipInfo} />}
            </div>
        </ItemTooltipContext.Provider>
    );
};

const positionTooltipDiv = (
    tooltipDiv: HTMLDivElement,
    x: number,
    y: number,
) => {
    x += 10;
    y += 10;
    const oldX = x;
    const oldY = y;
    tooltipDiv.style.left = `${x}px`;
    tooltipDiv.style.top = `${y}px`;
    const rect = tooltipDiv.getBoundingClientRect();
    if (rect.bottom > window.innerHeight) {
        y -= rect.height + 20;
    }
    if (rect.right > window.innerWidth) {
        x -= rect.width + 20;
    }
    if (x < 0) {
        x = 0;
    }
    if (y < 0) {
        y = 0;
    }
    if (x !== oldX) {
        tooltipDiv.style.left = `${x}px`;
    }
    if (y !== oldY) {
        tooltipDiv.style.top = `${y}px`;
    }
};
