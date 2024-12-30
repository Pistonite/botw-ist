import { Divider, makeStyles, mergeClasses, tokens } from "@fluentui/react-components";
import { Children, useRef, useState, type PropsWithChildren } from "react";


export type ResizeLayoutProps = {
    /** If the resize layout should be horizontal or vertical */
    vertical?: boolean;

    /** The minimum pixel width of the first child */
    minWidth?: number;

    /** The minimum pixel height of the first child */
    minHeight?: number;

    /** The current percentage size of the first child */
    valuePercent: number;

    /** Callback to set the percentage size of the first child */
    setValuePercent: (percent: number) => void;

    /** If the resizing is disabled */
    disabled?: boolean;

    /** Use the natural size instead of valuePercent */
    naturalSize?: boolean;

    /** Optimize for touch screen */
    touch?: boolean;
} & Omit<React.HTMLAttributes<HTMLDivElement>, "ref" | "onMouseUp" | "onMouseLeave">;

const useStyles = makeStyles({
    container: {
        display: "flex",
    },
    containerVertical: {
        flexDirection: "column",
    },
    containerHorizontal: {
        flexDirection: "row",
    },
    childContainer: {
        // needed to anchor drag handle with position: absolute
        position: "relative",
    },
    secondChild: {
        flex: 1,
    },
    dragHandle: {
        position: "absolute",
        transition: "background-color 0.2s",
    },
    dragHandleFirst: {
        bottom: 0,
        right: 0,
    },
    dragHandleSecond: {
        top: 0,
        left: 0,
    },
    dragHandleVertical: {
        // for vertical resize, the handle itself is "horizontal"
        width: "100%",
        height: "3px",
        cursor: "ns-resize",
    },
    dragHandleHorizontal: {
        // for horizontal resize, the handle itself is "vertical"
        width: "3px",
        height: "100%",
        cursor: "ew-resize",
    },
    dragHandleResizing: {
        backgroundColor: "rgba(0, 0, 0, 0.1)",
        zIndex: 10000,
    },
    // dragHandleVisible: {
    //     transition: "background-color 0.2s",
    //     display: "flex",
    // },
    dragHandleTouchVertical: {
        height: "20px",
        width: "100%",
    },
    dragHandleTouchHorizontal: {
        width: "20px",
        height: "100%",
    },
});

/** 
 * A flex-box layout of 2 children, with a draggable divider between them.
 */
export const ResizeLayout: React.FC<PropsWithChildren<ResizeLayoutProps>> = ({
    vertical, valuePercent, setValuePercent, disabled, naturalSize, minWidth,
    minHeight,
    touch,
    children, ...props
}) => {
    const [firstChild, secondChild] = Children.toArray(children);

    const styles = useStyles();
    const containerRef = useRef<HTMLDivElement>(null);
    const firstRef = useRef<HTMLDivElement>(null);
    // [startX, startY, startWidth, startHeight]
    const [resizing, setResizing] = useState<number[] | undefined>(undefined);

    const startResize = (e: React.MouseEvent | React.TouchEvent) => {
        if (disabled || !firstRef.current) {
            return;
        }
        const coords = getEventClientCoords(e);
        if (!coords) {
            return;
        }
        try {
            e.preventDefault();
            e.stopPropagation();
        } catch {
            // prevent default will fail for touch events
        }
        const first = firstRef.current.getBoundingClientRect();
        setResizing([coords.clientX, coords.clientY, first.width, first.height])
    };

    const handleResize = (e: React.MouseEvent | React.TouchEvent) => {
        if (disabled || !resizing || !containerRef.current || !firstRef.current) {
            return;
        }
        const coords = getEventClientCoords(e);
        if (!coords) {
            return;
        }
        const { width: containerWidth, height: containerHeight } = containerRef.current.getBoundingClientRect();
        const [startX, startY, startWidth, startHeight] = resizing;
        const deltaX = coords.clientX - startX;
        const deltaY = coords.clientY - startY;
        if (vertical) {
            const newHeight = startHeight + deltaY;
            setValuePercent((newHeight / containerHeight) * 100)
        } else {
            const newWidth = startWidth + deltaX;
            setValuePercent((newWidth / containerWidth) * 100)
        }
    };
    return (
        <div
            ref={containerRef}
            {...props}
            className={mergeClasses(
                props.className, 
                styles.container,
                vertical ? styles.containerVertical : styles.containerHorizontal,
            )}
            onMouseUp={() => { setResizing(undefined) }}
            onMouseLeave={() => { setResizing(undefined) }}
            onTouchEnd={() => { setResizing(undefined) }}
            onMouseMove={handleResize}
            onTouchMove={handleResize}
        >
            <div ref={firstRef} className={styles.childContainer} style={{
                minWidth: naturalSize ? undefined : minWidth,
                minHeight: naturalSize ? undefined : minHeight,
                width: naturalSize ? undefined : vertical ? "100%" : `${valuePercent}%`,
                height: naturalSize ? undefined : vertical ? `${valuePercent}%` : "100%",
            }}>
                {firstChild}
                {
                    !disabled && (
                        <div className={mergeClasses(
                            styles.dragHandle, 
                            styles.dragHandleFirst, 
                            vertical ? 
                                (touch ? styles.dragHandleTouchVertical: styles.dragHandleVertical)
                                : 
                                (touch ? styles.dragHandleTouchHorizontal : styles.dragHandleHorizontal),
                            resizing && styles.dragHandleResizing,
                        )}
                            onMouseDown={startResize}
                            onTouchStart={startResize}
                        />
                    )
                }
            </div>
            <div className={mergeClasses(styles.childContainer, styles.secondChild)}>
                {
                    !disabled && (
                        <div className={mergeClasses(
                            styles.dragHandle, 
                            styles.dragHandleSecond, 
                            vertical ? 
                                (touch ? styles.dragHandleTouchVertical: styles.dragHandleVertical)
                                : 
                                (touch ? styles.dragHandleTouchHorizontal : styles.dragHandleHorizontal),
                            resizing && styles.dragHandleResizing
                        )}
                            onMouseDown={startResize}
                            onTouchStart={startResize}
                        />
                    )
                }
                {secondChild}
            </div>
        </div>
    );
};

const getEventClientCoords = (e: React.TouchEvent | React.MouseEvent) => {
    if ("touches" in e && e.touches[0]) {
        const touch = e.touches[0];
        return {
            clientX: touch.clientX, 
            clientY: touch.clientY
        };
    }
    if ("clientX" in e && "clientY" in e) {
        return {
            clientX: e.clientX, 
            clientY: e.clientY
        };
    }
    return  undefined;
}
