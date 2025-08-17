import { log } from "./constants.ts";

let searchableActors: string[] = [];
export const getSearchableActors = (keys: string[]) => {
    if (searchableActors.length === 0) {
        const actors = keys
            .map((key) => {
                if (key.startsWith("actor.") && key.endsWith(".name")) {
                    return filterActorName(key.slice(6, -5));
                }
                return undefined;
            })
            .filter((x) => x !== undefined) as string[];
        searchableActors = Array.from(new Set(actors));
        log.info(`initialized ${searchableActors.length} searchable actors`);
    }
    return searchableActors;
};

export const isArrow = (actor: string): boolean => {
    return actor.endsWith("Arrow") || actor === "BombArrow_A";
};

export const isMaterial = (actor: string): boolean => {
    if (actor.startsWith("Item_")) {
        return !actor.startsWith("Item_Cook_");
    }
    return actor.startsWith("Animal_Insect_") || actor === "BeeHome";
};

const AdditionalSearchableActors = new Set([
    "AncientArrow",
    "BombArrow_A",
    // do we need these? could lead to inaccurate search
    // "BrightArrow",
    // "BrightArrowTP",
    "ElectricArrow",
    "FireArrow",
    "IceArrow",
    "NormalArrow",

    // itemized animals
    "Animal_Insect_A",
    "Animal_Insect_AA",
    "Animal_Insect_AB",
    "Animal_Insect_B",
    "Animal_Insect_C",
    "Animal_Insect_E",
    "Animal_Insect_F",
    "Animal_Insect_G",
    "Animal_Insect_H",
    "Animal_Insect_I",
    "Animal_Insect_M",
    "Animal_Insect_N",
    "Animal_Insect_P",
    "Animal_Insect_Q",
    "Animal_Insect_R",
    "Animal_Insect_S",
    "Animal_Insect_T",
    "Animal_Insect_X",
    "BeeHome",

    "Get_TwnObj_DLC_MemorialPicture_A_01",
    "Obj_Armor_115_Head",
    "Obj_DungeonClearSeal",
    "Obj_FireWoodBundle",
    "Obj_KorokNuts",
    "Obj_Maracas",
    "Obj_ProofBook",
    "Obj_ProofGiantKiller",
    "Obj_ProofSandwormKiller",
    "Obj_ProofGolemKiller",
    "Obj_ProofKorok",
    "Obj_WarpDLC",
    "Obj_DRStone_Get",
    "PlayerStole2",
]);

/**
 * Check if the actor name should be searchable
 */
const filterActorName = (actor: string): string | undefined => {
    if (!actor) {
        return undefined;
    }
    if (actor.endsWith("_00")) {
        return undefined;
    }
    if (actor.startsWith("Weapon_Sword_")) {
        if (actor.endsWith("_071")) {
            // Cutscene MS
            return undefined;
        }
        if (actor.endsWith("_072")) {
            // True MS for icon (?)
            return undefined;
        }
        if (actor.endsWith("_080")) {
            // TOTS Cutscene MS
            return undefined;
        }
        if (actor.endsWith("_081")) {
            // TOTS Cutscene True MS
            return undefined;
        }
        if (actor.endsWith("_500")) {
            // ?
            return undefined;
        }
        if (actor.endsWith("_501")) {
            // Korok Stick
            return undefined;
        }
        if (actor.endsWith("_503")) {
            // Cutscene OHO
            return undefined;
        }
        return actor;
    }
    if (actor.startsWith("Weapon_Bow_") || actor.startsWith("Weapon_Spear_")) {
        if (actor.endsWith("_080")) {
            return undefined;
        }
    }

    if (actor.startsWith("Weapon_")) {
        return actor;
    }

    if (actor.startsWith("Armor_")) {
        if (actor === "Armor_140_Lower") {
            return undefined; // borrowed snow boots
        }
        if (actor.startsWith("Armor_Default")) {
            return undefined;
        }
        if (actor.endsWith("_B")) {
            return undefined;
        }
        return actor;
    }

    if (actor.startsWith("Item_")) {
        if (actor === "Item_Enemy_Put_57") {
            // placed octo balloon?
            return undefined;
        }
        return actor;
    }

    if (actor.startsWith("GameRomHorseReins_")) {
        return actor;
    }

    if (actor.startsWith("GameRomHorseSaddle_")) {
        return actor;
    }

    if (actor.startsWith("Obj_DLC_HeroSeal_")) {
        return actor;
    }

    if (actor.startsWith("Obj_DLC_HeroSoul_")) {
        return actor;
    }

    if (actor.startsWith("Obj_HeroSoul_")) {
        return actor;
    }

    if (AdditionalSearchableActors.has(actor)) {
        return actor;
    }

    return undefined;
};
