import type { ActorSpriteProps } from "botw-item-assets";

import { useApplicationStore } from "./ApplicationStore.ts";

export const useItemSlotPropsFromSettings = () => {
    const enableHighRes = useApplicationStore(
        (state) => state.enableHighQualityIcons,
    );
    const enableAnimations = useApplicationStore(
        (state) => state.enableAnimations,
    );
    return {
        cheap: !enableHighRes,
        disableAnimation: !enableAnimations,
    } satisfies Omit<ActorSpriteProps, "actor">;
};
