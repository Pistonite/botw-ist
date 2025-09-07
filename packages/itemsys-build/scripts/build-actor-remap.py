import os
import yaml
import json
SELF_DIR = os.path.dirname(os.path.dirname(os.path.realpath(__file__)))

# requires research-scripts package to be built
RESEARCH_SCRIPTS_DIR = os.path.join(os.path.dirname(SELF_DIR), "research-scripts")
if not os.path.exists(RESEARCH_SCRIPTS_DIR):
    raise Exception(f"botw-research-scripts not found: {RESEARCH_SCRIPTS_DIR}")

HEADER = """
/**
 * This file is generated, see scripts/build-actor-remap.py
 * DO NOT EDIT MANUALLY
 */

/** Actor name -> icon actor name, if different */
export type ActorRemap = Record<string, string>;
"""

OUTPUT_DIR = os.path.join(os.path.dirname(SELF_DIR), "itemsys", "src", "generated")

def main():
    actor_icon_remap_path = os.path.join(RESEARCH_SCRIPTS_DIR, "output", "actor-icon-remap.yaml")
    with open(actor_icon_remap_path, "r", encoding="utf-8") as f:
        icon_remap = yaml.safe_load(f)
    if not os.path.exists(OUTPUT_DIR):
        os.makedirs(OUTPUT_DIR, exist_ok=True)
    output_path = os.path.join(OUTPUT_DIR, "actor_remap.ts")
    with open(output_path, "w", encoding="utf-8", newline="\n") as f:
        f.write(HEADER)
        f.write("\n")
        f.write("export const ActorRemap: ActorRemap = JSON.parse(`")
        json.dump(icon_remap, f, sort_keys=True, separators=(',', ':')) # minify
        f.write("`);\n")
    print(output_path)

if __name__ == "__main__":
    main()


