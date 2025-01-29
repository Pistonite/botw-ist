import { makeStaticStyles, makeStyles, mergeClasses } from "@griffel/react";
import { memo } from "react";

import {
    ModifierChunkClasses,
    ModifierMetadata,
} from "./generated/ModifierMetadata.ts";

export type ModifierSpriteProps = {
    /** Name of the special status to show */
    status: string;

    /** Optional size of the sprite, default is 20 */
    size?: number;
};

const useChunkClasses = makeStaticStyles(ModifierChunkClasses);

const useStyles = makeStyles({
    sprite: {
        backgroundRepeat: "no-repeat",
        display: "block",
    },
});

const SpriteImpl: React.FC<ModifierSpriteProps> = ({ size, status }) => {
    size = size || 20;
    useChunkClasses();
    const styles = useStyles();
    if (!ModifierMetadata[status]) {
        return null;
    }
    const [_, position] = ModifierMetadata[status];
    const backgroundPosition = getBackgroundPosition(position, size);

    return (
        <div
            aria-hidden
            className={mergeClasses("sprite-modifiers", styles.sprite)}
            style={{
                backgroundPosition,
                width: size,
                height: size,
                backgroundSize: size * NUM,
            }}
        ></div>
    );
};

export const ModifierSprite = memo(SpriteImpl);

const NUM = 8;
const getBackgroundPosition = (position: number, size: number) => {
    const x = position % NUM;
    const y = Math.floor(position / NUM);
    return `-${x * size}px -${y * size}px`;
};
