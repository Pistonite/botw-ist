import os
import shutil
import subprocess
import yaml
import json
from dataclasses import dataclass

DEBUG_PRINT = False

def extend_yaml():
    def dict_ctor(loader, node):
        values = loader.construct_mapping(node)
        return dict(values)

    def str_ctor(loader, node):
        values = loader.construct_scalar(node)
        return str(values)

    def int_ctor(loader, node):
        values = loader.construct_scalar(node)
        return int(values, 0)

    def list_ctor(loader, node):
        values = loader.construct_sequence(node)
        return list(values)

    yaml.add_constructor('!list', dict_ctor)
    yaml.add_constructor('!obj', dict_ctor)
    yaml.add_constructor('!io', dict_ctor)
    yaml.add_constructor('!str64', str_ctor)
    yaml.add_constructor('!str32', str_ctor)
    yaml.add_constructor('!str256', str_ctor)
    yaml.add_constructor('!vec3', list_ctor)
    yaml.add_constructor('!u', int_ctor)

def assertion(value, message = "Assertion failed"):
    if not value:
        raise AssertionError(message)


SCRIPT_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
DATA_DIR = os.path.join(SCRIPT_DIR, "data")
BOTW_DATA_PATH = os.path.join(DATA_DIR, "botw")
BOTW_DATA_REPO = "https://github.com/leoetlino/botw"
BOTW_DATA_BRANCH = "master"
BOTW_DATA_CHECKOUT = [
    "Actor/ActorLink",
    "Actor/GeneralParamList",
    "Message",
]

def download_data_if_needed():
    def check_paths():
        for path in BOTW_DATA_CHECKOUT:
            path = os.path.join(BOTW_DATA_PATH, path)
            if not os.path.exists(path):
                return False
        return True
    def sparse_checkout(repo, path, branch, checkout_paths):
        git = shutil.which("git")
        if not git:
            raise Exception("git not found")
        subprocess.run([git, "init"], cwd=path)
        subprocess.run([git, "remote", "add", "origin", repo], cwd=path)
        subprocess.run([git, "config", "core.sparseCheckout", "true"], cwd=path)
        with open(os.path.join(path, ".git", "info", "sparse-checkout"), "w", encoding="utf-8") as f:
            for checkout_path in checkout_paths:
                f.write(checkout_path + "\n")
        subprocess.run([git, "pull", "--depth=1", "origin", branch], cwd=path)
    if not check_paths():
        print("missing data files, cloning repositories...")
        try:
            if os.path.exists(DATA_DIR):
                shutil.rmtree(DATA_DIR)
            os.makedirs(BOTW_DATA_PATH)
            sparse_checkout(BOTW_DATA_REPO, BOTW_DATA_PATH, BOTW_DATA_BRANCH, BOTW_DATA_CHECKOUT)
            if not check_paths():
                print("still missing files after cloning. Exiting.")
                exit(1)
            else:
                print("data files cloned successfully.")
        except Exception as e:
            print(e)
            print()
            print("fail to get data files. Please clone them manually or try again.")
            exit(1)
    else:
        print("found data files, skipping cloning.")

def emit_actor_data(actors, gparamlists, localization):
    actor_dir = os.path.join(DATA_DIR, "actor")
    if os.path.exists(actor_dir):
        shutil.rmtree(actor_dir)
    os.makedirs(actor_dir)

    print("writing actor files...")

    for actor_name, actor in actors.items():
        if DEBUG_PRINT:
            print(f"-- {actor_name}")
        with open(os.path.join(actor_dir, f"{actor_name}.yaml"), "w", encoding="utf-8", newline="\n") as f:
            f.write(f"actor: {actor_name}\n")
            f.write(f"name_jpn: {actor.name_jpn}\n")
            if actor.tags:
                f.write("tags:\n")
                for tag in actor.tags:
                    f.write(f"  - {tag}\n")
            else:
                f.write("tags: []\n")
            if actor.model:
                f.write(f"model: {actor.model}\n")
            else:
                f.write("model: null\n")
            if actor.gparamlist:
                f.write(f"gparamlist:\n")
                f.write(f"  user: {actor.gparamlist}\n")
                f.write(f"  # ---\n")
                for key, value in gparamlists[actor.gparamlist]:
                    if isinstance(value, list):
                        data = json.dumps(value)
                        f.write(f"  {key}: {data}\n")
                    else:
                        data = yaml.dump({key: value})
                        f.write(f"  {data}")
                        if not data.endswith("\n"):
                            f.write("\n")
            else:
                f.write("gparamlist: null\n")
            if actor.profile:
                f.write(f"profile: {actor.profile}\n")
            else:
                f.write("profile: null\n")
            if actor_name in localization:
                l = localization[actor_name]
                assertion(l.profile == actor.profile, f"Profile mismatch for {actor_name}")
                f.write("localization:\n")
                for locale in LOCALE_MAP:
                    f.write(f"  {locale}:\n")
                    strings = l.strings[locale]
                    name = json.dumps(strings.name)
                    name_attr = json.dumps(strings.name_attr)
                    f.write(f"    name:\n")
                    f.write(f"      text: {name}\n")
                    f.write(f"      attr: {name_attr}\n")
                    desc = json.dumps(strings.desc)
                    f.write(f"    desc: {desc}\n")
                    album_desc = json.dumps(strings.album_desc)
                    f.write(f"    album_desc: {album_desc}\n")
            else:
                f.write("localization: null\n")


class ActorLink:
    actor: str
    name_jpn: str
    tags: list[str]
    model: str = ""
    gparamlist: str = ""
    profile: str = ""

def load_actor_links() -> dict[str, ActorLink]:
    print("loading actor links...")
    actor_links = {}
    actor_link_path = os.path.join(BOTW_DATA_PATH, "Actor", "ActorLink")
    for file in os.listdir(actor_link_path):
        if DEBUG_PRINT:
            print(f"-- {file}")
        assertion(file.endswith(".yml"), "ActorLink file must end in .yml")
        actor_name = file[:-4]
        with open(os.path.join(actor_link_path, file), "r", encoding="utf-8") as f:
            actor = ActorLink()
            actor.actor = actor_name.strip()
            actor.tags = []

            data = yaml.load(f, yaml.FullLoader)
            assertion(data, "ActorLink data must not be empty")
            assertion("param_root" in data, "ActorLink data must have 'param_root'")
            param_root = data["param_root"]
            assertion(param_root, "ActorLink data must have valid 'param_root'")
            assertion("objects" in param_root, "ActorLink param_root must have 'objects'")
            objects = param_root["objects"]
            assertion(objects, "ActorLink param_root must have valid 'objects'")
            assertion("LinkTarget" in objects, "ActorLink objects must have 'LinkTarget'")
            link_targets = objects["LinkTarget"]
            assertion(link_targets, "ActorLink objects must have valid 'LinkTarget'")
            assertion("ActorNameJpn" in link_targets, "ActorLink LinkTargets must have 'ActorNameJpn'")

            name_jpn = link_targets["ActorNameJpn"]
            assertion(name_jpn and isinstance(name_jpn, str), "ActorLink LinkTargets ActorNameJpn must be a string")
            actor.name_jpn = name_jpn

            if "ModelUser" in link_targets:
                model = link_targets["ModelUser"]
                assertion(model and isinstance(model, str), "ActorLink LinkTargets ModelUser must be a string")
                if model == "Dummy":
                    model = ""
                actor.model = model.strip()

            if "GParamUser" in link_targets:
                gparamlist = link_targets["GParamUser"]
                assertion(gparamlist and isinstance(gparamlist, str), "ActorLink LinkTargets GParamUser must be a string")
                if gparamlist == "Dummy":
                    gparamlist = ""
                actor.gparamlist = gparamlist.strip()

            if "ProfileUser" in link_targets:
                profile = link_targets["ProfileUser"]
                assertion(profile and isinstance(profile, str), "ActorLink LinkTargets ProfileUser must be a string")
                if profile == "Dummy":
                    profile = ""
                actor.profile = profile.strip()

            if "Tags" in objects:
                tags = objects["Tags"]
                # print(tags)
                assertion(tags and isinstance(tags, dict), "ActorLink objects Tags must be a dict")
                for i in range(0, 99): # at most 20 something tags
                    tag_name = f"Tag{i}"
                    if tag_name not in tags:
                        break
                    tag = tags[tag_name]
                    assertion(tag and isinstance(tag, str), f"ActorLink objects Tags {tag_name} must be a string")
                    actor.tags.append(tag.strip())

                assertion(len(actor.tags) == len(set(actor.tags)), 
                          f"{actor_name}: ActorLink tags must be unique, but got: {actor.tags}, {len(actor.tags)}, {len(set(actor.tags))}")
            actor_links[actor_name] = actor


    print(f"loaded {len(actor_links)} actors")
    return actor_links

@dataclass
class GparamKey:
    name: str
    default: str | int | float | bool | None
def load_gparam_keys() -> list[GparamKey]:
    print("loading gparamkeys...")
    gparam_path = os.path.join(BOTW_DATA_PATH, "Actor", "GeneralParamList", "Dummy.gparamlist.yml")
    keys = []
    with open(gparam_path, "r", encoding="utf-8") as f:
        # not using YAML parser for the whole thing to preserve key order
        current_key_prefix: str = ""
        current_table: list[str] = []
        for line in f:
            if not line.startswith("    "):
                continue
            line = line[4:]
            if not line.startswith(" "):
                if current_table:
                    # table end
                    data = yaml.load("\n".join(current_table), yaml.FullLoader)
                    assertion(data and isinstance(data, dict) and len(data) == 1, "Table must not be empty")
                    current_key= list(data.keys())[0]
                    current_key_prefix = current_key[0].lower() + current_key[1:]
                    data = data[current_key]
                    for key in sorted(data.keys()):
                        keys.append(GparamKey(current_key_prefix + key, data[key]))

                current_table = []
                current_key_prefix = ""

                line = line.strip()
                if line.endswith("!obj") or line.endswith(":"):
                    # line is just table name
                    key = line.split(":")[0].strip()
                    current_key_prefix = key[0].lower() + key[1:]
                else:
                    # table inline (need to parse whole table together)
                    current_table = [line]
                continue
            if current_table:
                current_table.append(line)
                continue
            line = line.strip()
            data = yaml.load(line, yaml.FullLoader)
            assertion(data and isinstance(data, dict) and len(data) == 1, "Data must not be empty")
            key = list(data.keys())[0]
            keys.append(GparamKey(current_key_prefix+key, data[key]))
    print(f"loaded {len(keys)} gparamkeys")
    if DEBUG_PRINT:
        for key in keys:
            data = yaml.dump({key.name: key.default})
            print(data)

    return keys

def parse_gpl(obj):
    out = {}
    for key in obj:
        lower = key[0].lower() + key[1:]
        for subkey in obj[key]:
            out[lower + subkey] = obj[key][subkey]
    return out

def load_gparamlist(keys: list[GparamKey]) -> dict[str, list[tuple[str, object]]]:
    print("loading gparamlists...")
    gparamlist = {}
    gparam_path = os.path.join(BOTW_DATA_PATH, "Actor", "GeneralParamList")

    for file in os.listdir(gparam_path):
        if DEBUG_PRINT:
            print(f"-- {file}")
        assertion(file.endswith(".gparamlist.yml"), "GParamList file must end in .yml")
        gparam_name = file[:-15]
        with open(os.path.join(gparam_path, file), "r", encoding="utf-8") as f:
            data = yaml.load(f, yaml.FullLoader)
            assertion(data, "GParamList data must not be empty")
            assertion("param_root" in data, "GParamList data must have 'param_root'")
            param_root = data["param_root"]
            assertion(param_root, "GParamList data must have valid 'param_root'")
            assertion("objects" in param_root, "GParamList param_root must have 'objects'")
            objects = parse_gpl(param_root["objects"])

            parsed = []

            for key in keys:
                name = key.name
                if name not in objects:
                    continue
                value = objects[name]
                del objects[name]
                if value == key.default:
                    continue
                if value is None:
                    raise ValueError(f"Key {name} in {gparam_name} is None")
                parsed.append((name, value))

            unknown_keys = set(objects.keys())
            if unknown_keys:
                print(objects)
                raise ValueError(f"Unknown keys in {gparam_name}: {unknown_keys}")

            gparamlist[gparam_name] = parsed

    print(f"loaded {len(gparamlist)} gparamlists")

    return gparamlist

@dataclass
class LocalizationStrings:
    name: str = ""
    name_attr: str = ""
    desc: str = ""
    album_desc: str = ""

@dataclass
class LocalizationEntry:
    profile: str
    strings: dict[str, LocalizationStrings]

LOCALE_MAP = {
    "en-US": "USen",
    "ja-JP": "JPja",
    "de-DE": "EUde",
    "es-ES": "EUes",
    "it-IT": "EUit",
    "fr-FR": "EUfr",
    "ru-RU": "EUru",
    "zh-CN": "CNzh",
    "zh-TW": "TWzh",
    "ko-KR": "KRko",
    "nl-NL": "EUnl",
}    
def load_actor_localization() -> dict[str, LocalizationEntry]:
    print("loading localization...")
    entries = {}
    for locale, locale_nin in LOCALE_MAP.items():
        load_l10n_for_locale(locale, locale_nin, entries)
    print(f"loaded {len(entries)} entries")
    return entries

def load_l10n_for_locale(locale: str, locale_nin: str, entries: dict[str, LocalizationEntry]):
    localization_path = os.path.join(BOTW_DATA_PATH, "Message", f"Msg_{locale_nin}.product.sarc", "ActorType")

    for file in os.listdir(localization_path):
        assertion(file.endswith(".msyt"), "Localization file must end in .msyt")
        profile = file[:-5]
        with open(os.path.join(localization_path, file), "r", encoding="utf-8") as f:
            data = yaml.load(f, yaml.FullLoader)
            load_l10n_for_locale_profile(locale, profile, data, entries)

def load_l10n_for_locale_profile(locale: str, profile: str, data, entries: dict[str, LocalizationEntry]):
    assertion(data and isinstance(data, dict) and "entries" in data, "Localization data must have 'entries'")
    entries_data = data["entries"]
    assertion(isinstance(entries_data, dict), "Localization entries must be a dict")
    for entry_name, entry_data in entries_data.items():
        if entry_name.endswith("_Name"):
            actor_name = entry_name[:-5]
            text, attr = parse_localization(entry_name, entry_data, True )
            strings = ensure_l10n_entry(entries, profile, actor_name, locale)
            strings.name = text
            strings.name_attr = attr
        elif entry_name.endswith("_Desc"):
            actor_name = entry_name[:-5]
            text, attr = parse_localization(entry_name, entry_data, False )
            strings = ensure_l10n_entry(entries, profile, actor_name, locale)
            strings.desc = text
        elif entry_name.endswith("_PictureBook"):
            actor_name = entry_name[:-13]
            text, attr = parse_localization(entry_name, entry_data, False )
            strings = ensure_l10n_entry(entries, profile, actor_name, locale)
            strings.album_desc = text

def parse_localization(name, data, allow_attr):
    assertion(isinstance(data, dict) and "contents" in data and len(data) == 1, "Localization data must have 'contents'")
    contents = data["contents"]
    text = ""
    attr = ""
    last_control = None

    for x in contents:
        if "text" in x:
            if last_control:
                text += x["text"][last_control:]
                last_control = None
            else:
                text += x["text"]
            continue
        assertion("control" in x and len(x) == 1, f"{name}: Entry must have either text or control")
        c = x["control"]
        assertion("kind" in c and isinstance(c["kind"], str), f"{name}: Control must have kind")
        kind = c["kind"]
        if kind == "raw":
            if "zero" in c:
                # katakana marking above kanji
                try:
                    last_control = c["zero"]["zero"]["field_3"] 
                except KeyError:
                    raise ValueError(f"failed to parse control for {name}")
        
                # divided by 2 because it's in bytes
                assertion(last_control % 2 == 0, "Odd number of bytes to remove")
                last_control = last_control // 2
                continue

            if "two" in c:
                # effect and effect description placeholder
                one_field_value = c["two"]["one_field"][0]
                if one_field_value == 7:
                    text += "{effect}"
                elif one_field_value == 8:
                    text += "{effect_desc}"
                elif one_field_value == 13:
                    text += "{modifier_value}"
                else:
                    raise ValueError(f"{name} invalid two.one_field0: {one_field_value}")
                continue

            if "two_hundred_one" in c:
                # See exefs/main/sub_7100AA4B4C
                if allow_attr:
                    assertion("dynamic" in c["two_hundred_one"], "two_hundred_one must have dynamic")
                    dynamic = c["two_hundred_one"]["dynamic"]
                    assertion("field_2" in dynamic[1])
                    v = dynamic[1]["field_2"]
                    assertion(len(v) == 4, "dynamic field_2 must have 4 elements")
                    plural = v[3]
                    assertion(plural == 0 or plural == 1, "plural must be 0 or 1")
                    plural = plural == 1
                    if v[0] == 0:
                        t = ""
                    elif v[0] == 1:
                        t = "masculine"
                    elif v[0] == 2:
                        t = "feminine"
                    elif v[0] == 3:
                        t = "neuter"
                    else:
                        raise ValueError(f"{name} invalid dynamic field_2: {v}")
                    if plural:
                        attr = "plural"
                    else:
                        attr = t
                continue

            raise ValueError(f"{name} invalid raw control: {c}")

    return text, attr

def ensure_l10n_entry(entries: dict[str, LocalizationEntry], profile: str, actor: str, locale: str) -> LocalizationStrings:
    if actor not in entries:
        strings = {}
        for l in LOCALE_MAP:
            strings[l] = LocalizationStrings()
        entries[actor] = LocalizationEntry(profile, strings)
    entry = entries[actor]
    return entry.strings[locale]

# unused
def filter_actor(name):
    """Return True if the actor is an item (should be processed)"""
    # Weapons
    if name.startswith("Weapon_Sword_"):
        if name.endswith("_071"): # Cutscene MS
            return False
        if name.endswith("_072"): # True MS for icon (?)
            return False
        if name.endswith("_080"): # TOTS Cutscene MS
            return False
        if name.endswith("_081"): # TOTS Cutscene True MS
            return False
        if name.endswith("_500"): # ?
            return False
        if name.endswith("_501"): # Korok Stick Cutscene
            return False
        if name.endswith("_503"): # Cutscene OHO
            return True
        return True
    if name.startswith("Weapon_Lsword_"):
        return True
    if name.startswith("Weapon_Spear_"):
        if name.endswith("_080"): # Lightscale Cutscene
            return False
        return True
    if name.startswith("Weapon_Bow_"):
        if name.endswith("_080"): # GEB Cutscene
            return False
        return True
    if name in ("NormalArrow", "FireArrow", "IceArrow", "ElectricArrow", "BombArrow_A", "AncientArrow"):
        return True
    if name.startswith("Weapon_Shield_"):
        return True
    if name.startswith("Armor_"):
        if name.startswith("Armor_Default"):
            return False
        if name.endswith("_B"): # ?
            return False
        return True
    if name in (
        "Item_Fruit_D",
        "Item_Fruit_G",
        "Item_Fruit_A",
        "Item_Fruit_B",
        "Item_Fruit_F",
        "Item_Fruit_I",
        "Item_Fruit_C",
        "Item_Fruit_E",
        "Item_Fruit_H",
        "Item_Mushroom_N",
        "Item_Mushroom_F",
        "Item_Mushroom_O",
        "Item_Mushroom_E",
        "Item_Mushroom_A",
        "Item_Mushroom_B",
        "Item_Mushroom_C",
        "Item_Mushroom_H",
        "Item_Mushroom_D",
        "Item_Mushroom_L",
        "Item_Mushroom_M",
        "Item_Mushroom_J",
    ):
        return True
    if name.startswith("Item_Enemy_"):
        if name == "Item_Enemy_Put_57": # placed octo balloon (?)
            return False
        return True
    if name.startswith("Item_FishGet_"):
        if name.endswith("L_00"): # multiple fish
            return False
        return True
    if name.startswith("Item_Fruit_"):
        if name.endswith("E_00"): # multiple lotus
            return False
        return True
    if name.startswith("Item_InsectGet_"):
        return True
    if name.startswith("Item_Material_"):
        if name.endswith("05_00"): # multiple milk
            return False
        return True
    if name.startswith("Item_Meat_"):
        return True
    if name.startswith("Item_Mushroom_"):
        if name.endswith("_00"): # multiple
            return False
        return True
    if name.startswith("Item_Ore_"):
        if name.endswith("A_00"): # multiple diamond
            return False
        return True
    return False

def emit_effects():
    def static_msg_file(locale_nin, file):
        return os.path.join(BOTW_DATA_PATH, "Message", f"Msg_{locale_nin}.product.sarc", "StaticMsg", file)
    # This is the effect displayed on the player/weapon
    # Could come from:
    #   - Weapon modifier
    #   - Meal effect
    #   - Armor effect
    # Ordered by appearance in SpecialStatus.myst
    SPECIAL_STATUS_TABLE = [
        "AddGuardPlus", # (wpmod) Yellow Shield Guard Up
        "AddGuard", # (wpmod) Blue Shield Guard Up
        "AddLifePlus", # (wpmod) Yellow Durability Up
        "AddLife", # (wpmod) Blue Durability Up
        "AddPowerPlus", # (wpmod) Yellow Attack Up
        "AddPower", # (wpmod) Blue Attack Up
        "AllSpeed", # (meal) Movement Speed Up
        "AttackUp", # (meal/armor) Attack Up
        # -- The CompletionBonus (armor set bonus) are skipped
        "ClimbSpeedUp", # (armor) Climbing Speed Up
        "Critical", # (wpmod) Critical Hit
        "DefenseUp", # (meal) Defense Up
        "ExGutsMaxUp", # (meal) Endura
        "Fireproof", # (meal) Fireproof // I think set bonus is also this?
        "GutsRecover", # (meal) Stamina recover
        "LifeMaxUp", # (meal) Hearty/yellow hearts
        "LongThrow", # (wpmod) Long Throw
        "Quietness", # (meal/armor) Stealth Up
        "RapidFire", # (wpmod) Quick Shot
        "ReduceAncientEnemyDamge", # (armor) ancient/ diamond circlet/midna, didn't spell wrong
        "ResistCold", # (meal/armor) Cold Resistance
        "ResistElectric", # (meal/armor) Shock Resistance
        "ResistFreeze", # (armor) unfreezable
        "ResistHot", # (meal/armor) Heat Resistance
        "ResistLightning", # (armor) Lightning Proof
        "SandMoveSpeedUp", # (armor) Sand Speed Up
        "SnowMovingSpeed", # (armor) Snow Speed Up
        "SpreadFire", # (wpmod) Multi-Shot
        "SurfMaster", # (wpmod) Shield surf friction minus
        "SwimSpeedUp", # (armor) Swim Speed Up
    ]
    # [
    #     hash name in CookData.System, 
    #     effect name in translation, <- used as name
    #     effect name in code,
    #     CookEffectId value,
    #     SpecialStatus name
    # ]
    # For CookData.System, see:
    #   - https://github.com/Pistonite/botw-recipe/blob/main/research/output/cookdata-system.yaml
    #     which is decoded from Cooking/CookData.yaml from leoetlino/botw
    # For code values, see cookManager.cpp and cookManager.h in botw decomp
    # Ordered by CookData.System
    #
    # Note that LifeRecover has no translation entry
    COOK_EFFECTS = [
        ["StaminaRecover",  "GutsRecover",    "GutsRecover",    14, "GutsRecover"],
        ["GutsPerformance", "ExGutsMaxUp",    "ExGutsMaxUp",    15, "ExGutsMaxUp"],
        ["LifeRecover",     "LifeRecover",    "LifeRecover",    1 , None],
        ["LifeMaxUp",       "LifeMaxUp",      "LifeMaxUp",      2 , "LifeMaxUp"],
        ["ResistHot",       "ResistHot",      "ResistHot",      4 , "ResistHot"],
        ["ResistCold",      "ResistCold",     "ResistCold",     5 , "ResistCold"],
        ["ResistElectric",  "ResistElectric", "ResistElectric", 6 , "ResistElectric"],
        ["AllSpeed",        "AllSpeed",       "MovingSpeed",    13, "AllSpeed"],
        ["AttackUp",        "AttackUp",       "AttackUp",       10, "AttackUp"],
        ["DefenseUp",       "DefenseUp",      "DefenseUp",      11, "DefenseUp"],
        ["Quietness",       "Quietness",      "Quietness",      12, "Quietness"],
        ["Fireproof",       "Fireproof",      "Fireproof",      16, "Fireproof"],
    ]
    def lookup_linked_cook_effect(special_status):
        for _, cook_effect, _, _, ss in COOK_EFFECTS:
            if ss == special_status:
                return cook_effect
        return None
            

    # [ 
    #     internal name,  <- used as name
    #     botw decomp name (WeaponModifier), 
    #     code value, 
    #     SpecialStatus name
    #     Yellow Special Status name
    # ]
    WEAPON_MODIFIERS = [
        ["None",       "None",          0,     None,         None],
        ["AddPower",   "AddAtk",        0x1,   "AddPower",   "AddPowerPlus"],
        ["AddLife",    "AddLife",       0x2,   "AddLife",    "AddLifePlus"],
        ["Critical",   "AddCrit",       0x4,   "Critical",   "Critical"],
        ["LongThrow",  "AddThrow",      0x8,   "LongThrow",  "LongThrow"],
        ["SpreadFire", "AddSpreadFire", 0x10,  "SpreadFire", "SpreadFire"],
        ["Zoom",       "AddZoomRapid",  0x20,  None,         None],
        ["RapidFire",  "AddRapidFire",  0x40,  "RapidFire",  "RapidFire"],
        ["SurfMaster", "AddSurfMaster", 0x80,  "SurfMaster", "SurfMaster"],
        ["AddGuard",   "AddGuard",      0x100, "AddGuard",   "AddGuardPlus"],
    ]
    def lookup_linked_weapon_modifier(special_status):
        for wm_name, _, _, ss, ss_yellow in WEAPON_MODIFIERS:
            if ss == special_status or ss_yellow == special_status:
                return wm_name
        return None
    print("loading special status effects...")
    special_status_localization = {}
    for locale, locale_nin in LOCALE_MAP.items():
        with open(static_msg_file(locale_nin, "SpecialStatus.msyt"), "r", encoding="utf-8") as f:
            entries_data = yaml.load(f, yaml.FullLoader)["entries"]
            for name, data in entries_data.items():
                assertion(name.endswith("_Name"), f"SpecialStatus entry must end in _Name: {name}")
                special_status_name = name[:-5]
                if special_status_name not in special_status_localization:
                    special_status_localization[special_status_name] = {}
                text, _= parse_localization(name, data, False)
                special_status_localization[special_status_name][locale] = text
    print("writing special status files...")
    SPECIAL_STATUS_DIR = os.path.join(DATA_DIR, "SpecialStatus")
    if os.path.exists(SPECIAL_STATUS_DIR):
        shutil.rmtree(SPECIAL_STATUS_DIR)
    os.makedirs(SPECIAL_STATUS_DIR)
    for special_status in SPECIAL_STATUS_TABLE:
        with open(os.path.join(SPECIAL_STATUS_DIR, f"{special_status}.yaml"), "w", encoding="utf-8", newline="\n") as f:
            f.write(f"name: {special_status}\n")
            cook_effect = lookup_linked_cook_effect(special_status)
            if cook_effect:
                f.write(f"cook_effect: {cook_effect}\n")
            else:
                f.write("cook_effect: null\n")
            weapon_modifier = lookup_linked_weapon_modifier(special_status)
            if weapon_modifier:
                f.write(f"weapon_modifier: {weapon_modifier}\n")
            else:
                f.write("weapon_modifier: null\n")
            f.write("localization:\n")
            for locale in LOCALE_MAP:
                text = special_status_localization[special_status][locale]
                f.write(f"  {locale}: {json.dumps(text)}\n")
    print("loading cook effects...")
    cook_effect_localization = {}
    for locale, locale_nin in LOCALE_MAP.items():
        with open(static_msg_file(locale_nin, "CookEffect.msyt"), "r", encoding="utf-8") as f:
            entries_data = yaml.load(f, yaml.FullLoader)["entries"]
            for name, data in entries_data.items():
                parts = name.split("_", 1)
                effect_name = parts[0]
                rest = parts[1] if len(parts) > 1 else ""
                if effect_name not in cook_effect_localization:
                    cook_effect_localization[effect_name] = {}
                    for l in LOCALE_MAP:
                        cook_effect_localization[effect_name][l] = {}
                if rest == "Name":
                    text, _ = parse_localization(name, data, False)
                    cook_effect_localization[effect_name][locale]["name"] = text
                elif rest == "Name_Feminine":
                    text, _ = parse_localization(name, data, False)
                    cook_effect_localization[effect_name][locale]["name_feminine"] = text
                elif rest == "Name_Masculine":
                    text, _ = parse_localization(name, data, False)
                    cook_effect_localization[effect_name][locale]["name_masculine"] = text
                elif rest == "Name_Neuter":
                    text, _ = parse_localization(name, data, False)
                    cook_effect_localization[effect_name][locale]["name_neuter"] = text
                elif rest == "Name_Plural":
                    text, _ = parse_localization(name, data, False)
                    cook_effect_localization[effect_name][locale]["name_plural"] = text
                elif rest.startswith("Desc"):
                    parts = rest.split("_", 1)
                    desc_level = (int(parts[1]) - 1) if len(parts) > 1 else 0
                    if "desc" not in cook_effect_localization[effect_name][locale]:
                        cook_effect_localization[effect_name][locale]["desc"] = []
                    desc_array = cook_effect_localization[effect_name][locale]["desc"]
                    if len(desc_array) <= desc_level:
                        desc_array.extend([""] * (desc_level - len(desc_array) + 1))
                    text, _ = parse_localization(name, data, False)
                    desc_array[desc_level] = text
                elif rest.startswith("MedicineDesc"):
                    parts = rest.split("_", 1)
                    desc_level = (int(parts[1]) - 1) if len(parts) > 1 else 0
                    if "elixir_desc" not in cook_effect_localization[effect_name][locale]:
                        cook_effect_localization[effect_name][locale]["elixir_desc"] = []
                    desc_array = cook_effect_localization[effect_name][locale]["elixir_desc"]
                    if len(desc_array) <= desc_level:
                        desc_array.extend([""] * (desc_level - len(desc_array) + 1))
                    text, _ = parse_localization(name, data, False)
                    desc_array[desc_level] = text
                else:
                    raise ValueError(f"Unknown cook effect entry in {name}")
    COOK_EFFECT_DIR = os.path.join(DATA_DIR, "CookEffect")
    if os.path.exists(COOK_EFFECT_DIR):
        shutil.rmtree(COOK_EFFECT_DIR)
    os.makedirs(COOK_EFFECT_DIR)
    for system_name, name, code_name, value, special_status in COOK_EFFECTS:
        with open(os.path.join(COOK_EFFECT_DIR, name + ".yaml"), "w", encoding="utf-8", newline="\n") as f:
            f.write(f"name: {name}\n")
            f.write(f"system_name: {system_name}\n")
            f.write(f"code_name: {code_name}\n")
            f.write(f"value: {value}\n")
            if special_status:
                f.write(f"special_status: {special_status}\n")
            else:
                f.write("special_status: null\n")
            if name in cook_effect_localization:
                f.write("localization:\n")
                l10n_data = cook_effect_localization[name]
                for locale, data in l10n_data.items():
                    f.write(f"  {locale}:\n")
                    f.write(f"    name: {json.dumps(data['name'])}\n")
                    f.write(f"    name_feminine: {json.dumps(data['name_feminine'])}\n")
                    f.write(f"    name_masculine: {json.dumps(data['name_masculine'])}\n")
                    f.write(f"    name_neuter: {json.dumps(data['name_neuter'])}\n")
                    f.write(f"    name_plural: {json.dumps(data['name_plural'])}\n")
                    f.write(f"    desc:\n")
                    for desc in data["desc"]:
                        f.write(f"      - {json.dumps(desc)}\n")
                    f.write(f"  elixir_desc:\n")
                    for desc in data["elixir_desc"]:
                        f.write(f"      - {json.dumps(desc)}\n")
            else:
                f.write("localization: null\n")







if __name__ == "__main__":
    extend_yaml()
    download_data_if_needed()
    # gparamkeys = load_gparam_keys()
    # actors = load_actor_links()
    # gparamlists = load_gparamlist(gparamkeys)
    # localization = load_actor_localization()

    # emit_actor_data(actors, gparamlists, localization)
    emit_effects()

