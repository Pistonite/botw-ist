import yaml

ITEM_YAML = "../../../legacy/src/data/item/all.items.yaml"
TRANSLATION_YAML = "../../localization/src/generated/en-US.yaml"

with open(ITEM_YAML, "r") as f:
    items = yaml.safe_load(f)

with open(TRANSLATION_YAML, "r") as f:
    translations = yaml.safe_load(f)

actor_to_english = {}

for key in translations:
    if key.startswith("actor.") and key.endswith(".name"):
        actor_name = key.split(".")[1]
        name = (
            translations[key]
            .replace(" ", "")
            .replace("'", "")
            .replace("-", "")
            .replace(":", "")
            .replace("\u2019", "")
            .replace("{{effect}}", "")
            .replace("\xE9", "e")
            .replace("\xE8", "e")
        )
        actor_to_english[actor_name] = name

# print(actor_to_english)

item_name_to_search_string={}
material_item_names = set()

for category_name in items:
    category = items[category_name]
    is_material = category_name == "material"
    entries = category["entries"]
    for item in entries:
        if isinstance(item, dict):
            item_name = next(iter(item))
            # if "priority" in item[item_name]:
                # print(item_name)
        else:
            item_name = item
        if ":" in item_name:
            parts = item_name.split(":")
            item_name = parts[0]
            search_string = parts[1]
        else:
            search_string = ""
        item_name_to_search_string[item_name] = search_string
        if is_material:
            material_item_names.add(item_name)

del item_name_to_search_string["Weapon"]
del item_name_to_search_string["Bow"]
del item_name_to_search_string["Shield"]
del item_name_to_search_string["ThunderHelmKey"]

item_name_to_actor_name = {}
item_name_to_actor_name["NormalArrow"] = "NormalArrow"
item_name_to_actor_name["WarmMilk"] = "Item_Cook_K_09"

elixir_keys = []
for name in item_name_to_search_string:
    if "Elixir" in name and name != "Elixir":
        elixir_keys.append(name)

for key in elixir_keys:
    del item_name_to_search_string[key]


replacement = [
    ("Plus", "+"),
]


for item_name in item_name_to_search_string:
    if item_name.endswith("Elixir") and item_name != "Elixir":
        continue
    if item_name == "NormalArrow":
        continue
    if item_name == "WarmMilk":
        continue
    if item_name == "ThunderHelmKey":
        continue
    found = False
    for actor_name in actor_to_english:
        if actor_to_english[actor_name].lower() == item_name.lower():
            item_name_to_actor_name[item_name] = actor_name
            found = True
            break
        for old_str, new_str in replacement:
            if actor_to_english[actor_name].lower() == item_name.replace(old_str, new_str).lower():
                item_name_to_actor_name[item_name] = actor_name
                found = True
                break

    if not found:
        raise Exception(f"Could not find actor name for item {item_name}")

output = []

for item_name in item_name_to_search_string:
    actor_name = item_name_to_actor_name[item_name]
    search_string = item_name_to_search_string[item_name]
    output.append(((item_name+search_string).lower(), actor_name, item_name in material_item_names, len(item_name)))

output.append(("thunderhelmkey", "Obj_Armor_115_Head", False, len("ThunderHelmKey")))
output.append(("lightarrow", "BrightArrow", False, len("LightArrow")))
output.append(("lightarrowtp", "BrightArrowTP", False, len("LightArrowTP")))

output.sort(key=lambda x: x[0])
for a, b, c,d in output:
    print(f"{a}:")
    print(f"  actor: {b}")
    print(f"  is_material: {c}")
    print(f"  id_len: {d}")
    # print("(\"" + a + "\", \"" + b + "\", " + ("true" if c else "false") + "),")

# for item_name in item_name_to_search_string:
#     search_string = item_name_to_search_string[item_name]
#     if search_string:
#         print(item_name, search_string)
#
#
