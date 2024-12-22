import os
import yaml

SCRIPT_DIR = os.path.dirname(os.path.realpath(__file__))
OUTPUT_DIR = os.path.join(SCRIPT_DIR, "output")

actor_files = os.listdir(OUTPUT_DIR)

ignore_name = set([
    # Not Items (don't have icon anyway)
    "Korok",
    "Traveler",
    "???",
    "Remote Bomb",
    "Remote Bomb +",
    "Monument Shard",
    "Finley",
    "Great Fairy Fountain",

    "Thunder Helm",

    # "Light Arrows",
    # "Sheikah Slate",
])

name_to_actor = {}
name_to_icon = {}

for actor_file in actor_files:
    actor_path = os.path.join(OUTPUT_DIR, actor_file)
    with open(actor_path, "r", encoding="utf-8") as f:
        actor = yaml.safe_load(f)
        if not actor["localization"]:
            continue
        strings = actor["localization"]["strings"]["en-US"]
        if not strings["name"]:
            continue
        name = strings["name"]
        if name in ignore_name:
            continue
        actor_name = actor["actor"]

        if name in name_to_actor:
            name_to_actor[name].append(actor_name)
        else:
            name_to_actor[name] = [actor_name]

        icon_actor = actor_name
        if actor["gparamlist"]:
            if "itemUseIconActorName" in actor["gparamlist"]:
                icon_actor = actor["gparamlist"]["itemUseIconActorName"]

        if name in name_to_icon:
            name_to_icon[name].add(icon_actor)
        else:
            name_to_icon[name] = set([icon_actor])

for name in name_to_actor:
    if len(name_to_actor[name]) > 1:
        print(f"{name}:")
        print(f"  actors: {name_to_actor[name]}")
        print(f"  icons: {list(name_to_icon[name])}")


