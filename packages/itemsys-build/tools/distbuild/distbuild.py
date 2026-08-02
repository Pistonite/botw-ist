import base64
import hashlib
import os
import shutil
SELF_DIR = os.path.dirname(os.path.dirname(os.path.dirname(os.path.realpath(__file__))))
DIST_DIR = os.path.join(SELF_DIR, "dist")
SRCGEN_DIR = os.path.join(os.path.dirname(SELF_DIR), "itemsys", "src", "generated")

def main():
    os.makedirs(DIST_DIR, exist_ok=True)
    os.makedirs(SRCGEN_DIR, exist_ok=True)
    copy_srcgen("target/codegen")
    copy_srcgen("target/sprite/actor_sprite_meta.ts")
    copy_srcgen("target/sprite/modifier_sprite_meta.ts")
    copy_dist([
        "public/fonts",
        "target/images",
        "target/encode",
        "target/sprite/chunk0x32.webp",
        "target/sprite/chunk0x64.webp",
        "target/sprite/chunk1x32.webp",
        "target/sprite/chunk1x64.webp",
        "target/sprite/chunk2x32.webp",
        "target/sprite/chunk2x64.webp",
        "target/sprite/modifiers.webp",
    ])

def copy_dist(files: list[str]):
    mapping: dict[str, str] = {}
    for file in files:
        copy_dist_one(os.path.join(SELF_DIR, file), mapping)

    # file mapping typescript for mapping to the right file in itemsys code
    lines = ["export const DistFileMapping = {"]
    for name in sorted(mapping):
        lines.append(f"    \"{name}\": \"{mapping[name]}\",")
    lines.append("} as const;")
    lines.append("export type DistFileKey = keyof typeof DistFileMapping;")
    lines.append("")
    output = os.path.join(SRCGEN_DIR, "dist_file_mapping.ts")
    with open(output, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(lines)+"\n")
    print("srcgen: dist_file_mapping.ts")

    # index of the files in dist
    index = os.path.join(DIST_DIR, "dist_index")
    with open(index, "w", encoding="utf-8", newline="\n") as f:
        f.write("\n".join(sorted(mapping.values())) + "\n")
    print("dist: dist_index")

def copy_dist_one(path: str, mapping: dict[str, str]):
    check_exists(path)
    if os.path.isdir(path):
        for entry in sorted(os.listdir(path)):
            copy_dist_one(os.path.join(path, entry), mapping)
        return

    name = os.path.basename(path)
    if name in mapping:
        raise Exception(f"duplicated file name in dist: {name}")

    with open(path, "rb") as f:
        digest = hashlib.sha256(f.read()).digest()
    hash = base64.urlsafe_b64encode(digest).decode("ascii")[:8]

    stem, ext = os.path.splitext(name)
    hashed_name = f"{stem}-{hash}{ext}"
    shutil.copyfile(path, os.path.join(DIST_DIR, hashed_name))
    mapping[name] = hashed_name
    print(f"dist: {name} -> {hashed_name}")

def copy_srcgen(file: str):
    path = os.path.join(SELF_DIR, file)
    check_exists(path)
    if os.path.isdir(path):
        for entry in sorted(os.listdir(path)):
            copy_srcgen(os.path.join(file, entry))
        return

    name = os.path.basename(path)
    shutil.copyfile(path, os.path.join(SRCGEN_DIR, name))
    print(f"srcgen: {name}")

def check_exists(path: str):
    if not os.path.exists(path):
        raise Exception(f"file not found: {path}, did you pull the deps?")

if __name__ == "__main__":
    main()
