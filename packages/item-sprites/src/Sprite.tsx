import { memo } from "react";
import { ChunkMap, Metadata } from "./sprites/metadata.ts";
import { makeStaticStyles, makeStyles, mergeClasses } from "@griffel/react";

export type SpriteProps = {
    /** Name of the Actor to display */
    actor: string;

    /**
     * Cook effect if any. This is to display the correct icon
     * for elixirs. Ignored if Actor is not "Item_Cook_C_17"
     */
    effect?: string;

    /** 
     * Use the deactivated state of certain actors
     * - Master Sword (any): The state where the sword is recharging
     * - One-hit Obliterator (502): The state where weapon is recharging
     * - Champion Abilities: disabled state
     */
    deactivated?: boolean;

    /**
     * Use the "powered up" state for Master Sword
     */
    powered?: boolean;

    /**
     * If true, display the flashing red effect
     */
    badlyDamaged?: boolean;

    /**
     * Use low resolution images
     */
    lowRes?: boolean;
};

const useChunkClasses = makeStaticStyles(ChunkMap);

const useStyles = makeStyles({
    sprite: {
        backgroundRepeat: "no-repeat",
        width: "64px",
        height: "64px",
        display: "inline-block",
    },
    spriteLowRes: {
        backgroundSize: "1024px", // for some reason 200% doesn't work
    }
});

const SpriteImpl: React.FC<SpriteProps> = ({ actor, effect, deactivated, powered, badlyDamaged, lowRes }) => {
    useChunkClasses();
    // TODO: animated sprites
    // TODO: disabled
    const iconActor = mapActor(actor, !!deactivated, !!powered, effect);
    const [chunk, position] = Metadata[iconActor];
    const backgroundPosition = getBackgroundPosition(position, !!lowRes);
    const styles = useStyles();

    const chunkClass = `sprite-chunk${chunk}x${lowRes ? "32" : "64"}`

    return (
    <div
            aria-hidden
            className={mergeClasses(chunkClass, styles.sprite, lowRes && styles.spriteLowRes)}
            style={{
                // backgroundImage: `url(${ChunkMap[chunk][lowRes ? 0 : 1]})`,
                backgroundPosition,

    }} />
    );
};

export const Sprite = memo(SpriteImpl);

const mapActor = (actor: string, 
    deactivated: boolean, powered: boolean, effect: string | undefined) => {
    // cannot manually pass in a "Disabled" actor
    if (actor.endsWith("_Disabled")) {
        return "Dummy";
    }
    if (deactivated || powered) {
        if (isMasterSword(actor)) {
            if (deactivated) {
                return "Weapon_Sword_070_Disabled";
            }
            // powered
            return "Weapon_Sword_072";
        }
    }
    if (!deactivated && actor === "Weapon_Sword_502") {
        // regular OHO, use powered up icon unless deactivated
        actor = "Weapon_Sword_503";
    }
    if (deactivated) {
        const souls = ["Gerudo", "Goron", "Rito", "Zora"];
        for (const soul of souls) {
            if (actor === `Obj_HeroSoul_${soul}`) {
                return `Obj_HeroSoul_${soul}_Disabled`;
            }
            if (actor === `Obj_DLC_HeroSoul_${soul}`) {
                return `Obj_DLC_HeroSoul_${soul}_Disabled`;
            }
        }
    }
    if (effect && actor === "Item_Cook_C_17") {
        actor = `${actor}_${effect}`;
    }
    // TODO: same icon remaps
    if (!(actor in Metadata)) {
        return "Dummy";
    }
    return actor;
};

const isMasterSword = (actor: string) => {
    return actor === "Weapon_Sword_070"; // regular MS
    // not sure what happens with these, just ignore for now
    // || actor === "Weapon_Sword_071" // cutscene MS
    //     || actor === "Weapon_Sword_072" // True MS for icon
    //     || actor === "Weapon_Sword_080" // TOTS Cutscene MS
    //     || actor === "Weapon_Sword_081"; // TOTS Cutscene True MS
}


const getBackgroundPosition = (position: number, lowRes: boolean) => {
    const NUM = 16;
    const SIZE = 64;//lowRes ? 32 : 64;
    const x = position % NUM;
    const y = Math.floor(position / NUM);
    return `-${x * SIZE}px -${y * SIZE}px`;
}

type SpriteDetail = {
    url: string;
    top: number;
    left: number;
    size: number;
}
