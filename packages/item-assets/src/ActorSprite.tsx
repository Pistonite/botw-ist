import { memo } from "react";
import { makeStaticStyles, makeStyles, mergeClasses } from "@griffel/react";

import { ActorChunkClasses, ActorMetadata } from "./generated/ActorMetadata.ts";
import { ActorRemap } from "./generated/ActorRemap.ts";

export type ActorSpriteProps = {
    /** Name of the Actor to display */
    actor: string;

    /**
     * Name of the Cook effect if any (as in msyt translation files)
     *
     * This is to display the correct icon for elixirs.
     * Ignored if Actor is not "Item_Cook_C_17"
     */
    effect?: string;

    /**
     * Use the low resolution/quality version of images. Also
     * disables animations
     */
    cheap?: boolean;

    /**
     * Disable all animations even if `cheap` is false
     */
    disableAnimation?: boolean;

    /**
     * Use styles to indicate that this slot is supposed to
     * not have an icon in the game (like multiple animated slots)
     */
    blank?: boolean;

    /**
     * Display the red effect for badly damaged
     *
     * The implementation only shows for non-animated images, which
     * is true for all weapons
     */
    badlyDamaged?: boolean;

    /**
     * Use the deactivated state of certain actors
     * - Master Sword (any): The state where the sword is recharging
     * - One-hit Obliterator (502): The state where weapon is recharging
     * - Champion Abilities: disabled state
     */
    deactive?: boolean;

    /**
     * Use the "powered up" state for Master Sword
     */
    powered?: boolean;
};

const useChunkClasses = makeStaticStyles(ActorChunkClasses);

const useStyles = makeStyles({
    sprite: {
        backgroundRepeat: "no-repeat",
        width: "64px",
        height: "64px",
        display: "block",
    },
    cheap: {
        backgroundSize: "1024px", // for some reason 200% doesn't work
    },
    spriteSoulImage: {
        position: "relative",
    },
    animatedSimple: {
        backgroundSize: "64px",
    },
    soulOffset: {
        top: "-11.4px",
    },
    soulOffsetDLC: {
        top: "-19px",
        left: "0px",
    },
    blank: {
        filter: "grayscale(100%)",
        opacity: 0.8,
    },
    deactiveMasterSword: {
        opacity: 0.5,
    },
    damageContainer: {
        overflow: "hidden",
    },
    damage: {
        width: "1024px",
        height: "1024px",
        backgroundColor: "rgba(255, 0, 0, 0.6)",
    },
    damageCheap: {
        transformOrigin: "top left",
        scale: 2,
        width: "512px",
        height: "512px",
        backgroundColor: "rgba(255, 0, 0, 0.6)",
    },
    damageAnimation: {
        animationIterationCount: "infinite",
        animationDuration: "1s",
        animationName: {
            "0%": {
                opacity: 1,
            },
            "25%": {
                opacity: 0.9,
            },
            "50%": {
                opacity: 0.7,
            },
            "75%": {
                opacity: 0.4,
            },
            "100%": {
                opacity: 0,
            },
        },
    },
});

const SpriteImpl: React.FC<ActorSpriteProps> = ({
    actor,
    effect,
    cheap,
    disableAnimation,
    deactive,
    powered,
    blank,
    badlyDamaged,
}) => {
    useChunkClasses();
    const styles = useStyles();

    let baseClass = mergeClasses(styles.sprite, blank && styles.blank);

    disableAnimation = disableAnimation || cheap;

    // Handle simple animated images - Travel Medallion, 5 orbs
    // if not animated, it's in the sprite sheet
    if (!disableAnimation) {
        if (
            /Obj_(WarpDLC|DungeonClearSeal|HeroSeal_(Gerudo|Goron|Rito|Zora))/.test(
                actor,
            )
        ) {
            return (
                <div
                    aria-hidden
                    className={mergeClasses(baseClass, styles.animatedSimple)}
                    style={{
                        backgroundImage: `url(${new URL(`./special/${actor}.webp`, import.meta.url).href})`,
                    }}
                />
            );
        }
    }

    const iconActor = mapActor(actor, !!deactive, !!powered, effect);
    if (iconActor === "Weapon_Sword_070_Disabled") {
        baseClass = mergeClasses(baseClass, styles.deactiveMasterSword);
    }

    // Special handling for Champion Abilities:
    // - Active ones are larger and needs to be offseted
    // - Deactive ones has animation
    if (isChampionAbility(iconActor)) {
        const dlc = iconActor.includes("DLC");
        // active - either animated or not, with offset
        if (!deactive) {
            const ext = disableAnimation ? "png" : "webp";
            return (
                <div aria-hidden className={baseClass}>
                    <img
                        className={mergeClasses(
                            styles.spriteSoulImage,
                            dlc ? styles.soulOffsetDLC : styles.soulOffset,
                        )}
                        src={
                            new URL(
                                `./special/${iconActor}.${ext}`,
                                import.meta.url,
                            ).href
                        }
                        width={64}
                    />
                </div>
            );
        }
        // inactive - if animated, use different sprite
        if (!disableAnimation) {
            return (
                <div
                    aria-hidden
                    className={mergeClasses(baseClass, styles.animatedSimple)}
                    style={{
                        backgroundImage: `url(${new URL(`./special/${iconActor}.webp`, import.meta.url).href})`,
                    }}
                />
            );
        }
    }

    const [chunk, position] = ActorMetadata[iconActor];
    const backgroundPosition = getBackgroundPosition(position);

    const chunkClass = `chunk${chunk}x${cheap ? "32" : "64"}`;

    return (
        <div
            aria-hidden
            className={mergeClasses(
                `sprite-${chunkClass}`,
                baseClass,
                cheap && styles.cheap,
                badlyDamaged && styles.damageContainer,
            )}
            style={{ backgroundPosition }}
        >
            {badlyDamaged && (
                <div
                    className={mergeClasses(
                        `sprite-mask-${chunkClass}`,
                        cheap ? styles.damageCheap : styles.damage,
                        !disableAnimation && styles.damageAnimation,
                    )}
                    style={{
                        translate: backgroundPosition,
                    }}
                />
            )}
        </div>
    );
};

export const ActorSprite = memo(SpriteImpl);

/**
 * Remap an actor to its icon actor name
 */
const mapActor = (
    actor: string,
    deactive: boolean,
    powered: boolean,
    effect: string | undefined,
): string => {
    // cannot manually pass in a "Disabled" actor
    if (actor.endsWith("_Disabled")) {
        return "Dummy";
    }
    // Remap actor name to icon actor name
    if (actor in ActorRemap) {
        actor = ActorRemap[actor];
    }

    // Master Sword
    if (actor === "Weapon_Sword_070") {
        if (deactive) {
            return "Weapon_Sword_070_Disabled";
        }
        if (powered) {
            return "Weapon_Sword_072";
        }
    }

    // regular OHO, use powered up icon unless deactivated
    if (!deactive && actor === "Weapon_Sword_502") {
        actor = "Weapon_Sword_503";
    }

    // Champion Abilities
    if (isChampionAbility(actor)) {
        // need to return here because animated images are not
        // in Dummy
        if (deactive) {
            return `${actor}_Disabled`;
        }
        return actor;
    }

    // Elixirs - they are the same actor, but icon
    // depends on the effect
    if (effect && actor === "Item_Cook_C_17") {
        actor = `${actor}_${effect}`;
    }

    if (!(actor in ActorMetadata)) {
        return "Dummy";
    }
    return actor;
};

const getBackgroundPosition = (position: number) => {
    const NUM = 16;
    const SIZE = 64;
    const x = position % NUM;
    const y = Math.floor(position / NUM);
    return `-${x * SIZE}px -${y * SIZE}px`;
};

const isChampionAbility = (actor: string) => {
    return /^Obj_(DLC_)?HeroSoul_(Gerudo|Goron|Rito|Zora)(_Disabled)?$/.test(
        actor,
    );
};
