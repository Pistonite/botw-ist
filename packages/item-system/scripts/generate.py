import yaml
import os
import multiprocessing
import json
SELF_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
RESEARCH_SCRIPTS_DIR = os.path.join(os.path.dirname(SELF_DIR), "research-scripts")
if not os.path.exists(RESEARCH_SCRIPTS_DIR):
    raise Exception(f"botw-research-scripts not found: {RESEARCH_SCRIPTS_DIR}")
import sys
sys.path.append(os.path.join(RESEARCH_SCRIPTS_DIR, "src"))
import spp # type: ignore

HEADER = """
/**
 * This file is generated by generate-param.py
 * DO NOT EDIT MANUALLY
 */
import type { ActorData } from "../data/ActorData.ts";

export const ActorDataMap: Record<string, Partial<ActorData>> = JSON.parse(`"""

def main():
    actor_dir = os.path.join(RESEARCH_SCRIPTS_DIR, "output", "Actor")
    actor_files = [os.path.join(actor_dir, f) for f in os.listdir(actor_dir) ]

    progress = spp.printer(len(actor_files), "Load actor files")
    actor_data = {}
    with multiprocessing.Pool() as pool:
        for (i, result) in enumerate(pool.imap_unordered(process_actor, actor_files)):
            progress.update(i)
            if result:
                actor, data = result
                actor_data[actor] = data
    progress.done()

    output_path = os.path.join(SELF_DIR, "src", "generated", "ActorDataMap.ts")
    with open(output_path, "w", encoding="utf-8", newline="\n") as f:
        f.write(HEADER)
        json_data = json.dumps(actor_data, sort_keys=True, separators=(',', ':'))
        f.write(json_data)
        f.write("`);\n")

def process_actor(actor_file) -> tuple[str, dict] | None:
    with open(actor_file, "r", encoding="utf-8") as f:
        actor = yaml.safe_load(f)

    if not actor["profile"]:
        return None

    profile = actor["profile"]

    data = {}

    for tag in actor["tags"]:
        if tag == "CanStack":
            data["canStack"] = True
        if tag == "CannotSell":
            data["cannotSell"] = True

    gparam = actor["gparamlist"]
    if gparam:
        if "armorEffectEffectType" in gparam:
            effect_type = gparam["armorEffectEffectType"]
            data["armorEffectEffectType"] = effect_type
            # See item-system/src/data/enums.ts
            EFFECT_MAP = {
                "AttackUp": 8,
                "ClimbSpeed": 9,
                "ClimbSpeedAndBeamPowerUp": 9,
                "ResistAncient": 19,
                "ResistElectric": 21,
                "ResistCold": 20,
                "ResistColdAndResistAncient": 20,
                "Quietness": 17,
                "SwimSpeed": 29,
                "SwimSpeedAndResistAncient": 29,
                "SnowMove": 26,
                "ResistBurn": 13,
                "ResistBurnAndResistAncient": 13,
                "ResistElectricAndResistAncient": 21,
                "ResistHot": 23,
                "ResistHotAndWakeWind": 23,
                "SandMove": 25,
                "ResistLightning": 24,
                "ResistFreeze": 22,
            }
            if effect_type not in EFFECT_MAP:
                raise ValueError(f"Armor effect type not mapped: {effect_type}")
            data["specialStatus"] = EFFECT_MAP[effect_type]
        if "bowIsLeadShot" in gparam:
            data["specialStatus"] = 27
        if "bowIsRapidFire" in gparam and gparam["bowIsRapidFire"]:
            if "bowRapidFireNum" in gparam and gparam["bowRapidFireNum"] > 0:
                data["bowRapidFireNum"] = gparam["bowRapidFireNum"]
        if "bowLeadShotNum" in gparam and gparam["bowLeadShotNum"] > 0:
            data["bowLeadShotNum"] = gparam["bowLeadShotNum"]
        if "bowLeadShotAng" in gparam:
            data["bowLeadShotAng"] = gparam["bowLeadShotAng"]
        if "bowLeadShotInterval" in gparam:
            data["bowLeadShotInterval"] = gparam["bowLeadShotInterval"]
        if "bowArrowFirstSpeed" in gparam:
            data["bowArrowFirstSpeed"] = gparam["bowArrowFirstSpeed"]
        if "bowArrowStabilitySpeed" in gparam:
            data["bowArrowStabilitySpeed"] = gparam["bowArrowStabilitySpeed"]
        if "bowArrowGravity" in gparam:
            data["bowArrowGravity"] = gparam["bowArrowGravity"]
        if "bowIsLongRange" in gparam and gparam["bowIsLongRange"]:
            data["bowIsLongRange"] = gparam["bowIsLongRange"]
        if "bowArrowChargeRate" in gparam:
            data["bowArrowChargeRate"] = gparam["bowArrowChargeRate"]
        if "bowArrowReloadRate" in gparam:
            data["bowArrowReloadRate"] = gparam["bowArrowReloadRate"]
        if "bowIsGuardPierce" in gparam and gparam["bowIsGuardPierce"]:
            data["bowIsGuardPierce"] = gparam["bowIsGuardPierce"]
        if "attackPower" in gparam:
            data["attackPower"] = gparam["attackPower"]
        if "attackRange" in gparam:
            data["attackRange"] = gparam["attackRange"]
        if "generalLife" in gparam:
            data["generalLife"] = gparam["generalLife"]
            if "genrealIsLifeInfinite" in gparam:
                data["genrealIsLifeInfinite"] = gparam["genrealIsLifeInfinite"]
        if "itemSellingPrice" in gparam:
            data["itemSellingPrice"] = gparam["itemSellingPrice"]
        if "itemBuyingPrice" in gparam:
            data["itemBuyingPrice"] = gparam["itemBuyingPrice"]
        if "itemCreatingPrice" in gparam:
            data["itemCreatingPrice"] = gparam["itemCreatingPrice"]
        if "itemStainColor" in gparam:
            data["itemStainColor"] = gparam["itemStainColor"]
        if "weaponCommonGuardPower" in gparam:
            data["weaponCommonGuardPower"] = gparam["weaponCommonGuardPower"]
        if "armorStarNum" in gparam:
            data["armorStarNum"] = gparam["armorStarNum"]
        if "armorDefenceAddLevel" in gparam:
            data["armorDefenceAddLevel"] = gparam["armorDefenceAddLevel"]

    if not data:
        return None

    # filter out if it only has generalLife (some animals)
    if len(data) == 1 and "generalLife" in data:
        return None

    # filter out if it only has attackPower and life (some animals)
    if len(data) == 2 and "attackPower" in data and "generalLife" in data:
        return None

    data["profile"] = profile

    return actor["actor"], data
            
if __name__ == "__main__":
    main()
