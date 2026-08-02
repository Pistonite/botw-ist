import { makeStyles, mergeClasses } from "@fluentui/react-components";
import { memo } from "react";

import { MODIFIER_NUM_PER_SIDE, ModifierMetadata } from "#codegen";

export interface ModifierSpriteProps {
    /** Name of the special status to show */
    status: string;

    /** Optional size of the sprite, default is 20 */
    size?: number;
}

const useStyles = makeStyles({
    sprite: {
        backgroundRepeat: "no-repeat",
        display: "block",
    },
});

const SpriteImpl: React.FC<ModifierSpriteProps> = ({ size, status }) => {
    size = size || 20;
    const styles = useStyles();
    if (!ModifierMetadata[status]) {
        return null;
    }
    const [_, position] = ModifierMetadata[status];
    const backgroundPosition = getBackgroundPosition(position, size);

    return (
        <div
            aria-hidden
            className={mergeClasses("bia--sprite-modifiers", styles.sprite)}
            style={{
                backgroundPosition,
                width: size,
                height: size,
                backgroundSize: size * MODIFIER_NUM_PER_SIDE,
            }}
        />
    );
};

export const ModifierSprite = memo(SpriteImpl);

const getBackgroundPosition = (position: number, size: number) => {
    const x = position % MODIFIER_NUM_PER_SIDE;
    const y = Math.floor(position / MODIFIER_NUM_PER_SIDE);
    return `-${x * size}px -${y * size}px`;
};
