import { makeStyles, mergeClasses } from "@fluentui/react-components";
import { memo } from "react";

import { ActorRemap } from "../generated/actor_remap.ts";
import { getSpecialIconUrl } from "../asset_registry.ts";

import { ActorMetadata } from "./actor_meta.ts";

export type ActorSpriteProps = {
    /** Name of the Actor to display */
    actor: string;

    /** Optional size of the sprite, default is 64 */
    size?: number;

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

const useStyles = makeStyles({
    sprite: {
        backgroundRepeat: "no-repeat",
        display: "block",
    },
    spriteSoulImage: {
        position: "relative",
    },
    blank: {
        filter: "grayscale(100%)",
        opacity: 0.8,
    },
    deactiveMasterSword: {
        opacity: 0.5,
    },
    damageContainer: {
        // Only the damage overlay should have overflow hidden,
        // since animated sprites can overlay by design
        overflow: "hidden",
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
    size,
    effect,
    cheap,
    disableAnimation,
    deactive,
    powered,
    blank,
    badlyDamaged,
}) => {
    const styles = useStyles();

    size = size || 64;

    let baseClass = mergeClasses(styles.sprite, blank && styles.blank);

    disableAnimation = disableAnimation || cheap;

    // Handle simple animated images - Travel Medallion, 5 orbs
    // if not animated, it's in the sprite sheet
    const isSimpleAnimated =
        /Obj_(WarpDLC|DungeonClearSeal|HeroSeal_(Gerudo|Goron|Rito|Zora))/.test(actor);
    if (!disableAnimation && isSimpleAnimated) {
        return <div aria-hidden className={baseClass} style={getSpecialActorStyle(actor, size)} />;
    }

    const iconActor = mapActor(actor, !!deactive, !!powered, effect);
    if (iconActor === "Weapon_Sword_070_Disabled") {
        baseClass = mergeClasses(baseClass, styles.deactiveMasterSword);
    }

    // Special handling for Champion Abilities:
    // - Active ones are larger and needs to be offseted
    // - Deactive ones has animation
    const isAbility = isChampionAbilityIcon(actor);
    if (isAbility) {
        const dlc = iconActor.includes("DLC");
        // active - either animated or not, with offset
        if (!deactive) {
            const ext = disableAnimation ? "png" : "webp";
            const topOffset = ((dlc ? -19 : -11.4) / 64) * size;
            return (
                <div aria-hidden className={baseClass}>
                    <img
                        className={mergeClasses(styles.spriteSoulImage)}
                        style={{
                            top: topOffset,
                        }}
                        src={getSpecialIconUrl(`${iconActor}.${ext}`)}
                        width={size}
                    />
                </div>
            );
        }
        // inactive - if animated, use different sprite
        if (!disableAnimation) {
            return (
                <div
                    aria-hidden
                    className={baseClass}
                    style={getSpecialActorStyle(iconActor, size)}
                />
            );
        }
    }

    // don't show badly damaged effect for images that would be animated (but disabled)
    badlyDamaged = badlyDamaged && !isSimpleAnimated && !isAbility;

    const meta = ActorMetadata[iconActor];
    if (!meta) {
        return (
            <div
                aria-hidden
                className={mergeClasses(baseClass)}
                style={{
                    width: size,
                    height: size,
                }}
            >
                {iconActor}
            </div>
        );
    }
    const [chunk, position] = meta;
    const backgroundPosition = getBackgroundPosition(position, size);
    const spriteSize = cheap ? 32 : 64;

    const chunkClass = `chunk${chunk}x${spriteSize}`;

    const damageOverlayScale = size / spriteSize;

    return (
        <div
            aria-hidden
            className={mergeClasses(
                `bia--sprite-${chunkClass}`,
                baseClass,
                badlyDamaged && styles.damageContainer,
            )}
            style={{
                backgroundPosition,
                backgroundSize: NUM * size,
                width: size,
                height: size,
            }}
        >
            {badlyDamaged && (
                <div
                    className={mergeClasses(
                        `bia--sprite-mask-${chunkClass}`,
                        !disableAnimation && styles.damageAnimation,
                    )}
                    style={{
                        translate: backgroundPosition,
                        transform: `scale(${damageOverlayScale},${damageOverlayScale})`,
                        transformOrigin: "top left",
                        width: 1024,
                        height: 1024,
                        backgroundColor: "rgba(255, 0, 0, 0.6)",
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
    if (isChampionAbilityIcon(actor)) {
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

const NUM = 16; // number of sprites in a row/column
const getBackgroundPosition = (position: number, size: number) => {
    const x = position % NUM;
    const y = Math.floor(position / NUM);
    return `-${x * size}px -${y * size}px`;
};

const isChampionAbilityIcon = (iconActor: string) => {
    return /^Obj_(DLC_)?HeroSoul_(Gerudo|Goron|Rito|Zora)(_Disabled)?$/.test(iconActor);
};

const getSpecialActorStyle = (actor: string, size: number) => {
    return {
        width: size,
        height: size,
        backgroundImage: `url("${getSpecialIconUrl(actor + ".webp")}")`,
        backgroundSize: size,
    };
};
