import { type PropsWithChildren, useRef, useState, useCallback, useEffect } from "react";
import { makeStyles } from "@fluentui/react-components";

import { ItemTooltipContent } from "./ItemTooltipContent.tsx";
import { ItemTooltipContext, type SetItemTooltipFn } from "./ItemTooltipContext.ts";
import type { ItemTooltipWithContextProps } from "./ItemTooltip.tsx";

const useStyles = makeStyles({
    container: {
        position: "absolute",
        zIndex: 1000,
    },
    childrenContainer: {
        display: "contents",
    },
});

export type ItemTooltipProviderProps = {
    /** Url for the image for the background of the tooltip */
    backgroundUrl: string;
};

/**
 * Provider for the ItemTooltipContext
 *
 * Since item tooltips are very volatile (they move around by the user).
 * We really optimize the re-render. Otherwise, users will see CPU
 * spikes when moving the mouse really quickly
 */
export const ItemTooltipProvider: React.FC<PropsWithChildren<ItemTooltipProviderProps>> = ({
    backgroundUrl,
    children,
}) => {
    const styles = useStyles();

    const [verbose, setVerbose] = useState(false);

    // toggle verbose based on Shift key
    useEffect(() => {
        const controller = new AbortController();
        window.addEventListener(
            "keydown",
            (event: KeyboardEvent) => {
                if (event.key === "Shift") {
                    setVerbose(true);
                }
            },
            { signal: controller.signal },
        );
        window.addEventListener(
            "keyup",
            (event: KeyboardEvent) => {
                if (event.key === "Shift") {
                    setVerbose(false);
                }
            },
            { signal: controller.signal },
        );
        return () => {
            controller.abort();
        };
    }, []);
    const tooltipDivRef = useRef<HTMLDivElement>(null);
    const childrenContainerRef = useRef<HTMLDivElement>(null);
    const [tooltipProps, setTooltipProps] = useState<ItemTooltipWithContextProps | undefined>();
    const [tooltipTarget, setTooltipTarget] = useState<HTMLElement | undefined>();
    const lastPosition = useRef<[number, number]>([0, 0]);
    const setTooltip: SetItemTooltipFn = useCallback(
        (x, y, props, target, verboseNew) => {
            if (!tooltipDivRef.current) {
                return;
            }
            const tooltipDiv = tooltipDivRef.current;
            if (!props || !target) {
                tooltipDiv.style.display = "none";
                return;
            }
            const visibilityChanged = (tooltipDiv.style.display === "none") !== !props;
            tooltipDiv.style.display = "unset";
            // This is incorrect for the first render. See below
            positionTooltipDiv(tooltipDiv, x, y);
            lastPosition.current = [x, y];
            // if the tooltip target didn't change, we don't need to rerender
            if (visibilityChanged || tooltipTarget !== target || verboseNew !== verbose) {
                setTooltipProps(props);
                setTooltipTarget(target);
                setVerbose(verboseNew);
            }
        },
        [tooltipTarget, verbose],
    );

    // This is needed because the positioning in the setTooltip function
    // is incorrect - this is because the tooltip div is not in the DOM yet
    // or still has the dimension of the previous tooltip. This is usually
    // fine on PC since the mouse moves around and the tooltip position is updated.
    // However, on touch devices, the tooltip is shown on click and does not
    // change position. Therefore, we need this effect to position the tooltip
    // on the first render. Once rendered, setTooltip will handle the positioning
    // until the tooltip changes
    useEffect(() => {
        if (!tooltipDivRef.current) {
            return;
        }
        const tooltipDiv = tooltipDivRef.current;
        if (!tooltipDiv.isConnected || tooltipDiv.style.display === "none") {
            return;
        }
        const [x, y] = lastPosition.current;
        positionTooltipDiv(tooltipDiv, x, y);
    });

    // auto-hide the tooltip to prevent showing the wrong tooltip
    // for the current thing you are hovering on, because of the optimization
    // we did
    useEffect(() => {
        if (!childrenContainerRef.current || !tooltipTarget || !tooltipTarget.isConnected) {
            return;
        }
        const hide = () => {
            const tooltipDiv = tooltipDivRef.current;
            if (tooltipDiv?.isConnected) {
                tooltipDiv.style.display = "none";
            } else {
                setTooltipProps(undefined);
            }
        };
        // 1. if the target is removed
        const observer = new MutationObserver(() => {
            if (tooltipTarget && !tooltipTarget.isConnected) {
                hide();
            }
        });
        observer.observe(childrenContainerRef.current, {
            childList: true,
            subtree: true,
        });
        // 2. if the target itself changed
        const observer2 = new MutationObserver(() => {
            hide();
        });
        observer2.observe(tooltipTarget, {
            attributes: true,
            childList: true,
            subtree: true,
        });
        return () => {
            observer.disconnect();
            observer2.disconnect();
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
                {tooltipProps && <ItemTooltipContent {...tooltipProps} verbose={verbose} />}
            </div>
        </ItemTooltipContext.Provider>
    );
};

const positionTooltipDiv = (tooltipDiv: HTMLDivElement, x: number, y: number) => {
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
