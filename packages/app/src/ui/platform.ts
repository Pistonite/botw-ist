
import { isMobile, isSmartTV, isWearable } from "mobile-device-detect";
/** 
 * Detect if the platform is less productive than the conventional
 * KVM setup - for example, mobile, tablet, wearable, etc.
 */
export const isLessProductive= isMobile || isSmartTV || isWearable;
