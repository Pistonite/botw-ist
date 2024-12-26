
import yaml
import os
import multiprocessing
import shutil

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

def main():
    home = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
    actor_dir = os.path.join(home, "data", "Actor")
    actor_files = [os.path.join(actor_dir, f) for f in os.listdir(actor_dir) ]
    cook_effect_dir = os.path.join(home, "data", "CookEffect")
    cook_effect_files = [os.path.join(cook_effect_dir, f) for f in os.listdir(cook_effect_dir) ]
    special_status_dir = os.path.join(home, "data", "SpecialStatus")
    special_status_files = [os.path.join(special_status_dir, f) for f in os.listdir(special_status_dir) ]
    
    data = {}
    for locale in LOCALE_MAP:
        data[locale] = {}

    with multiprocessing.Pool() as pool:
        print("loading actor files...")
        for result in pool.imap_unordered(load_entries_for_actor, actor_files):
            if result:
                for locale in result:
                    data[locale].update(result[locale])
        print("loading cook effect files...")
        for result in pool.imap_unordered(load_entries_for_cook_effect, cook_effect_files):
            if result:
                for locale in result:
                    data[locale].update(result[locale])
        print("loading special status files...")
        for result in pool.imap_unordered(load_entries_for_special_status, special_status_files):
            if result:
                for locale in result:
                    data[locale].update(result[locale])

    output_dir = os.path.join(os.path.dirname(home), "localization", "src", "generated")
    if os.path.exists(output_dir):
        shutil.rmtree(output_dir)
    os.makedirs(output_dir)

    for locale in data:
        print(f"writing {locale}... ({len(data[locale])})")
        output_file = os.path.join(output_dir, f"{locale}.yaml")
        with open(output_file, "w", encoding="utf-8", newline="\n") as f:
            yaml.dump(data[locale], f, sort_keys=True)


def load_entries_for_actor(actor_file) -> dict[str, dict[str, str]] | None:
    """Load l10n entries from Actor/*.yaml"""

    with open(actor_file, "r", encoding="utf-8") as f:
        actor = yaml.safe_load(f)
    l10n = actor["localization"]
    if not l10n:
        return None
    data = {}
    for locale in LOCALE_MAP:
        # name, replace {effect} with the attributed one
        # e.g. feminine, masculine, neuter, plural
        name = l10n[locale]["name"]["text"]
        name_attr = l10n[locale]["name"]["attr"]
        if name_attr:
            name = name.replace("{{effect}}", "{{effect_"+name_attr+"}}")
        desc = l10n[locale]["desc"]
        album_desc = l10n[locale]["album_desc"]
        # use album desc if desc is not provided
        if not desc and album_desc:
            desc = album_desc

        data[locale] = {
            f"actor.{actor["actor"]}.name": name,
        }
        # not all actors have description
        if desc:
            data[locale][f"actor.{actor["actor"]}.desc"] = desc
    return data

def load_entries_for_cook_effect(cook_effect_file) -> dict[str, dict[str, str]] | None:
    """Load l10n entries from CookEffect/*.yaml"""

    with open(cook_effect_file, "r", encoding="utf-8") as f:
        cook_effect = yaml.safe_load(f)
    l10n = cook_effect["localization"]
    if not l10n:
        return None
    data = {}
    for locale in LOCALE_MAP:
        name = l10n[locale]["name"]
        name_feminine = l10n[locale]["name_feminine"]
        name_masculine = l10n[locale]["name_masculine"]
        name_neuter = l10n[locale]["name_neuter"]
        name_plural = l10n[locale]["name_plural"]
        data[locale] = {
            f"cook.{cook_effect["name"]}.name": name,
            f"cook.{cook_effect["name"]}.name_feminine": name_feminine,
            f"cook.{cook_effect["name"]}.name_masculine": name_masculine,
            f"cook.{cook_effect["name"]}.name_neuter": name_neuter,
            f"cook.{cook_effect["name"]}.name_plural": name_plural,
        }
        for i, d in enumerate(l10n[locale]["desc"]):
            data[locale][f"cook.{cook_effect["name"]}.desc_{i+1}"] = d
        for i, d in enumerate(l10n[locale]["elixir_desc"]):
            data[locale][f"cook.{cook_effect["name"]}.elixir_desc_{i+1}"] = d

    return data

def load_entries_for_special_status(special_status_file) -> dict[str, dict[str, str]] | None:
    """Load l10n entries from SpecialStatus/*.yaml"""

    with open(special_status_file, "r", encoding="utf-8") as f:
        special_status = yaml.safe_load(f)
    l10n = special_status["localization"]
    if not l10n:
        return None
    data = {}
    name = special_status["name"]
    for locale in LOCALE_MAP:
        value = l10n[locale]
        # Some modifiers doesn't show the value in game
        # we add it
        if name in ["RapidFire", "LongThrow", "SpreadFire", "SurfMaster"]:
            value += " [{{modifier_value}}]"
        if name == "SurfMaster" and (locale == "zh-CN" or locale == "zh-TW"):
            value = "\u76fe\u6ed1\u884c\u63d0\u5347 [{{modifier_value}}]"

        data[locale] = {
            f"status.{name}": value,
        }

    # patch for missing entries for Shield Surf Up
    return data

if __name__ == "__main__":
    main()

