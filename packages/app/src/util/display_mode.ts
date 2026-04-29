import {
    initDisplayMode as celeraInitDisplayMode,
    useDisplayMode as celeraUseDisplayMode,
    getDisplayMode as celeraGetDisplayMode,
} from "@pistonite/celera";

export type DisplayMode = "wide" | "narrow";

const NARROW_THRESHOLD = 800;

export const initDisplayMode = () => {
    celeraInitDisplayMode<DisplayMode>({
        initial: "wide",
        detect: (width, height, isMobile) => {
            if (isMobile) {
                if (width < height) {
                    return "narrow";
                }
                const newNarrow = width < NARROW_THRESHOLD;
                if (newNarrow && height < width) {
                    return "wide";
                }
                return "narrow";
            }
            if (height > width * 1.5) {
                return "narrow";
            }
            if (width < NARROW_THRESHOLD) {
                return "narrow";
            }
            return "wide";
        },
    });
};

export const useDisplayMode = celeraUseDisplayMode<DisplayMode>;
export const getDisplayMode = celeraGetDisplayMode<DisplayMode>;

export const useIsNarrow = (): boolean => {
    return isNarrowDisplayMode(useDisplayMode());
};
export const isNarrow = (): boolean => {
    return isNarrowDisplayMode(getDisplayMode());
};
export const isNarrowDisplayMode = (mode: DisplayMode): mode is "narrow" => mode[0] === "n";
