import { initDisplayMode as celeraInitDisplayMode,
    useDisplayMode as celeraUseDisplayMode,
    getDisplayMode as celeraGetDisplayMode,
} from "@pistonite/celera";

export type DisplayMode = "wide" | "narrow" | "mobile-wide" | "mobile-narrow";

const NARROW_THRESHOLD = 800;

export const initDisplayMode = () => {
    celeraInitDisplayMode<DisplayMode>({
        initial: "wide",
        detect: (width, height, isMobile) => {
            if (isMobile) {
                if (width < height) {
                    return "mobile-narrow";
                }
                const newNarrow = width < NARROW_THRESHOLD;
                if (newNarrow && height < width) {
                    return "mobile-wide";
                }
                return "mobile-narrow";
            }
            if (height > width * 1.5) {
                return "narrow";
            }
            if (width < NARROW_THRESHOLD) {
                return "narrow";
            }
            return "wide";
        }
    });
}

export const useDisplayMode = celeraUseDisplayMode<DisplayMode>;
export const getDisplayMode = celeraGetDisplayMode<DisplayMode>;

export const useIsMobile = (): boolean => {
    return isMobileDisplayMode(useDisplayMode());
}
export const useIsNarrow = (): boolean => {
    return isNarrowDisplayMode(useDisplayMode());
}
export const isMobile = (): boolean => {
    return isMobileDisplayMode(getDisplayMode());
}
export const isNarrow = (): boolean => {
    return isNarrowDisplayMode(getDisplayMode());
}

export const isMobileDisplayMode = (mode: DisplayMode): mode is "mobile-wide" | "mobile-narrow" => mode.startsWith("m");
export const isNarrowDisplayMode = (mode:DisplayMode): mode is "narrow" | "mobile-narrow" => mode.endsWith("w");
