import { makeStaticStyles, makeStyles, mergeClasses } from "@griffel/react";
import { memo } from "react";

import {
    ModifierChunkClasses,
    ModifierMetadata,
} from "./generated/ModifierMetadata.ts";

export type ModifierSpriteProps = {
    /** Name of the special status to show */
    status: string;
};

const useChunkClasses = makeStaticStyles(ModifierChunkClasses);

const useStyles = makeStyles({
    sprite: {
        backgroundRepeat: "no-repeat",
        backgroundSize: "160px 160px", // 20px * 8
        width: "20px",
        height: "20px",
        display: "block",
    },
});

const SpriteImpl: React.FC<ModifierSpriteProps> = ({ status }) => {
    useChunkClasses();
    const styles = useStyles();
    if (!ModifierMetadata[status]) {
        return null;
    }
    const [_, position] = ModifierMetadata[status];
    const backgroundPosition = getBackgroundPosition(position);

    return (
        <div
            aria-hidden
            className={mergeClasses("sprite-modifiers", styles.sprite)}
            style={{ backgroundPosition }}
        ></div>
    );
};

export const ModifierSprite = memo(SpriteImpl);

const getBackgroundPosition = (position: number) => {
    const NUM = 8;
    const SIZE = 20;
    const x = position % NUM;
    const y = Math.floor(position / NUM);
    return `-${x * SIZE}px -${y * SIZE}px`;
};
