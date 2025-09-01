import yaml
import os
import multiprocessing
SELF_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))
RESEARCH_SCRIPTS_DIR = os.path.join(os.path.dirname(SELF_DIR), "research-scripts")
import sys
sys.path.append(os.path.join(RESEARCH_SCRIPTS_DIR, "src"))
import msyt # type: ignore
import spp # type: ignore

OUTPUT_DIR = os.path.join(os.path.dirname(SELF_DIR), "itemsys", "src", "generated")

def main():

    actor_dir = os.path.join(RESEARCH_SCRIPTS_DIR, "output", "Actor")
    actor_files = [os.path.join(actor_dir, f) for f in os.listdir(actor_dir) ]

    cook_effect_dir = os.path.join(RESEARCH_SCRIPTS_DIR, "output", "CookEffect")
    cook_effect_files = [os.path.join(cook_effect_dir, f) for f in os.listdir(cook_effect_dir) ]

    special_status_dir = os.path.join(RESEARCH_SCRIPTS_DIR, "output", "SpecialStatus")
    special_status_files = [os.path.join(special_status_dir, f) for f in os.listdir(special_status_dir) ]
    
    data = {}
    for locale in msyt.locale_map:
        data[locale] = {}

    with multiprocessing.Pool() as pool:
        progress = spp.printer(len(actor_files), "Load actor files")
        for (i, result) in enumerate(pool.imap_unordered(load_entries_for_actor, actor_files)):
            progress.update(i)
            if result:
                for locale in result:
                    data[locale].update(result[locale])
        progress.done()
        progress = spp.printer(len(cook_effect_files), "Load cook effect files")
        for (i, result) in enumerate(pool.imap_unordered(load_entries_for_cook_effect, cook_effect_files)):
            progress.update(i)
            if result:
                for locale in result:
                    data[locale].update(result[locale])
        progress.done()
        progress = spp.printer(len(special_status_files), "Load special status files")
        for (i, result) in enumerate(pool.imap_unordered(load_entries_for_special_status, special_status_files)):
            progress.update(i)
            if result:
                for locale in result:
                    data[locale].update(result[locale])
        progress.done()

    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR, exist_ok=True)

    for locale in data:
        print(f"writing {locale}... ({len(data[locale])})")
        output_file = os.path.join(OUTPUT_DIR, f"{locale}.yaml")
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
    for locale in msyt.locale_map:
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
    for locale in msyt.locale_map:
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
    for locale in msyt.locale_map:
        value = l10n[locale]
        # Some modifiers doesn't show the value in game
        # we add it
        if name in ["RapidFire", "LongThrow", "SpreadFire", "SurfMaster"]:
            value += " [{{modifier_value}}]"

        data[locale] = {
            f"status.{name}": value,
        }
    return data

if __name__ == "__main__":
    main()

