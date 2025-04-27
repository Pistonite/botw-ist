import {
    type PropsWithChildren,
    useRef,
    useState,
    useCallback,
    useEffect,
} from "react";
import { makeStyles } from "@fluentui/react-components";

import { ItemTooltipContent } from "./ItemTooltipContent.tsx";
import {
    ItemTooltipContext,
    type SetItemTooltipFn,
} from "./ItemTooltipContext.ts";
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

/** Provider for the ItemTooltipContext */
export const ItemTooltipProvider: React.FC<
    PropsWithChildren<ItemTooltipProviderProps>
> = ({ backgroundUrl, children }) => {
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
    const [tooltipProps, setTooltipProps] = useState<
        ItemTooltipWithContextProps | undefined
    >();
    const [tooltipTarget, setTooltipTarget] = useState<
        HTMLElement | undefined
    >();
    const setTooltip: SetItemTooltipFn = useCallback(
        (x, y, props, target, verbose) => {
            if (!tooltipDivRef.current) {
                return;
            }
            const tooltipDiv = tooltipDivRef.current;
            if (!props || !target) {
                tooltipDiv.style.display = "none";
                return;
            }
            tooltipDiv.style.display = "unset";
            // This might initially be wrong the first time
            // the info is changed. However, most of the time, it will be
            // called again with the correct x and y when the mouse moves.
            positionTooltipDiv(tooltipDiv, x, y);
            setTooltipProps(props);
            setTooltipTarget(target);
            setVerbose(verbose);
        },
        [],
    );

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
                setTooltipProps(undefined);
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
                {tooltipProps && (
                    <ItemTooltipContent {...tooltipProps} verbose={verbose} />
                )}
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
