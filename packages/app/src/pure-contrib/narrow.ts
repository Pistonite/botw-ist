import { cell } from "@pistonite/pure/memory";
import { useSyncExternalStore } from "react";

const narrow = cell({ initial: false });

/** Options for narrow mode detection */
export type NarrowOptions = {
    /**
     * Width threshold to start displaying the app in narrow mode
     */
    threshold: number;

    /**
     * A function called to override the detected value
     */
    override?: (detected: boolean) => boolean;

    /**
     * Set the initial value, if the platform doesn't support detecting
     * viewport width.
     */
    initial?: boolean;
};

/**
 * Initialize narrow mode detection
 *
 * Note that this is not necessary in some simple use cases. For example,
 * adjusting styles based on the viewport width can be done with CSS:
 * ```css
 * @media screen and (max-width: 800px) {
 *    /* styles for narrow mode * /
 * }
 * ```
 *
 * Use this only if narrow mode needs to be detected programmatically.
 */
export const initNarrow = (options: NarrowOptions) => {
    const threshold = options.threshold;
    const override = options.override;
    if (window && window.addEventListener && window.innerWidth !== undefined) {
        const callback = () => {
            const newNarrow = window.innerWidth < threshold;
            if (override) {
                narrow.set(override(newNarrow));
            } else {
                narrow.set(newNarrow);
            }
        };
        window.addEventListener("resize", callback);
        callback();
    } else if (options.initial !== undefined) {
        narrow.set(options.initial);
    }
};

/** Subscribe to narrow mode changes */
export const addNarrowSubscriber = (
    subscriber: (narrow: boolean) => void,
    notifyImmediately?: boolean,
) => {
    return narrow.subscribe(subscriber, notifyImmediately);
};

/** Check if the app is in narrow mode */
export const isNarrow = (): boolean => {
    return narrow.get();
};

/** React hook to use narrow mode detection */
export const useNarrow = () => {
    return useSyncExternalStore(addNarrowSubscriber, isNarrow);
};
