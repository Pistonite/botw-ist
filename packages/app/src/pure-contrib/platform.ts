import { cell } from "@pistonite/pure/sync";

import { isMobile, isSmartTV, isWearable } from "mobile-device-detect";

const lessProductive = cell({ initial: isMobile || isSmartTV || isWearable })

export type PlatformOptions = {
    /** Width threshold to start displaying*/
    portraitWidth: number;
};


export const initPlatform = async (options: PlatformOptions) => {}

/** 
 * Detect if the platform is less productive than the conventional
 * KVM setup - for example, mobile, tablet, wearable, etc.
 */
export const isLessProductive= (): boolean => {
}
