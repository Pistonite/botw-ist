use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codize::cconcat;
use cu::pre::*;

use crate::canvas::Canvas;
use crate::config::{self, Config, RenderGroupConfig};
use crate::sprite_sheet::{Metadata, SpriteSheet};

pub fn run() -> cu::Result<()> {
    let config = cu::check!(config::load(), "failed to load config")?;
    cu::debug!("config: {config:?}");

    let chunks = cu::check!(load_chunks(&config), "failed to load chunks")?;
    cu::debug!("chunks: {chunks:?}");

    for (group_name, group) in &config.render.groups {
        render_group(&config, &chunks, group_name, group)?;
    }

    // let itemsys_dir = home.parent_abs()?.join("itemsys");
    // let src_dir = itemsys_dir.join("src").join("generated");
    // cu::fs::make_dir(&src_dir)?;
    // let sprites_dir = home.join("public").join("sprites");
    // cu::fs::make_dir(&sprites_dir)?;
    //
    // generate_actors(&icons_dir, &sprites_dir, &src_dir)?;
    // generate_modifiers(&icons_dir, &sprites_dir, &src_dir)?;

    cu::info!("done!");
    Ok(())
}

fn load_chunks(config: &Config) -> cu::Result<BTreeMap<String, Vec<PathBuf>>> {
    let mut out = BTreeMap::default();
    let mut seen_paths = BTreeSet::default();
    let mut seen_names = BTreeSet::default();

    for (chunk_name, globs) in &config.chunks {
        cu::debug!("loading files for chunk '{chunk_name}'");
        let mut paths = vec![];
        let mut walker = cu::fs::walker(&config.target);
        walker.glob_includes(globs)?;
        for e in walker.walk()? {
            let e = e?;
            let path = e.path().to_path_buf();
            if !seen_paths.insert(path.clone()) {
                cu::bail!("path '{}' exists in multiple chunks!", path.display());
            }
            let name = cu::check!(path.file_stem(), "cannot get file stem for path '{}'", path.display())?;
            let name = name.as_utf8()?;
            if !seen_names.insert(name.to_string()) {
                cu::bail!("file stem '{name}' exists in multiple paths!");
            }
            paths.push(path);
        }
        if paths.is_empty() {
            cu::bail!("chunk '{chunk_name}' did not match any files; did you pull the deps?");
        }
        cu::info!("chunk '{chunk_name}': {} files", paths.len());
        paths.sort_by_key(|x| x.file_stem().map(|x|x.to_os_string()));
        out.insert(chunk_name.clone(), paths);
    }
    Ok(out)
}

fn render_group(
    config: &Config,
    chunks: &BTreeMap<String, Vec<PathBuf>>,
    group_name: &str,
    group: &[RenderGroupConfig],
) -> cu::Result<()> {
    cu::info!("rendering group '{group_name}'");
    if group_name.is_empty() {
        cu::bail!("group name cannot be empty");
    }

    let mut metadata = Metadata::default();
    let mut seen_emit_paths = BTreeSet::default();

    for (i, chunk_config) in group.iter().enumerate() {
        let i = u16::try_from(i)?;
        let chunk_name = &chunk_config.chunk;
        let input_files = cu::check!(
            chunks.get(chunk_name),
            "group '{group_name}' references an invalid chunk name '{chunk_name}'"
        )?;
        let len = input_files.len();
        let mut spritesheet = SpriteSheet::new(i);

        for emit_config in &chunk_config.emit {
            let emit_path = &emit_config.path;
            if !seen_emit_paths.insert(emit_path.to_string()) {
                cu::bail!("duplicated emit path: '{emit_path}'");
            }
            let emit_profile = &emit_config.profile;

            let profile = cu::check!(
                config.render.profiles.get(emit_profile),
                "group '{group_name}', chunk '{chunk_name}' references an invalid profile '{emit_profile}'"
            )?;
            let sprites_per_side = profile.sprites_per_side;
            let max_files = (sprites_per_side * sprites_per_side) as usize;
            if max_files < len {
                cu::bail!(
                    "group '{group_name}', chunk '{chunk_name}', sprite '{emit_path}' has too many files: {len}, max {max_files}"
                );
            }

            let emit_path = cu::path!(&(&config.target) / "sprite" / emit_path);
            let canvas = Canvas::new(
                emit_path,
                sprites_per_side,
                profile.outer_size,
                profile.inner_size,
                profile.quality,
            );
            spritesheet.add_canvas(canvas);
        }

        for file in input_files {
            let name = cu::check!(file.file_stem(), "failed to get file stem name from '{}'", file.display())?;
            let name = name.as_utf8()?;
            spritesheet.add_sprite(name, file)?;
        }

        cu::info!("encoding '{group_name}': '{chunk_name}' ({len} files)");

        let sizes = spritesheet.write()?;
        for (size, emit_config) in std::iter::zip(&sizes, &chunk_config.emit) {
            let emit_path = &emit_config.path;
            cu::info!("-- {emit_path}: {size} bytes");
        }

        spritesheet.add_metadata(&mut metadata)?;

    }

    let meta_emit_name = format!("{group_name}_sprite_meta.ts");
    let meta_emit_path = cu::path!(&(&config.target) / "sprite" / meta_emit_name);
    let meta_type_name = snake_to_pascal(group_name);
    cu::info!("emitting metadata for '{group_name}': {meta_emit_name}");

    let ts_chunk_type = (0..group.len())
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let metadata = json::stringify(&metadata)?;
    let metadata_ts = cconcat![
        "/** FileName => [ChunkId, Position]*/",
        format!(
            "export type {meta_type_name} = Record<string,[{ts_chunk_type},number]>;",
        ),
        format!(
            "export const {meta_type_name}: {meta_type_name} = JSON.parse(`{metadata}`)",
        ),
    ];

    cu::fs::write(meta_emit_path, metadata_ts.to_string())?;

    Ok(())
}

// fn generate_actors(icons_dir: &Path, sprites_dir: &Path, src_dir: &Path) -> cu::Result<()> {
//     let ts_chunk_type = (0..sprite_sheets.len())
//         .map(|i| i.to_string())
//         .collect::<Vec<_>>()
//         .join("|");
//     let metadata = json::stringify(&metadata)?;
//     let metadata_ts = cconcat![
//         // metadata for finding where an actor is
//         "/** Actor => [Chunk, Position]*/",
//         format!(
//             "export type ActorMetadata = Record<string,[{},number]>;",
//             ts_chunk_type
//         ),
//         format!(
//             "export const ActorMetadata: ActorMetadata = JSON.parse(`{}`)",
//             metadata
//         ),
//     ];
//
//     cu::fs::write(
//         src_dir.join("actor_sprite_meta.ts"),
//         metadata_ts.to_string(),
//     )?;
//
//     Ok(())
// }
//
// fn generate_modifiers(icons_dir: &Path, sprites_dir: &Path, src_dir: &Path) -> cu::Result<()> {
//     cu::info!("configuring modifier chunks...");
//     const SPRITES_PER_SIDE: u32 = 8;
//     let modifier_chunk = find_images(icons_dir, &["SpecialStatus"])?;
//     let len = modifier_chunk.len();
//     let max = (SPRITES_PER_SIDE * SPRITES_PER_SIDE) as usize;
//     cu::info!("modifier chunk: {len}/{max} images");
//     cu::ensure!(len <= max, "too many modifiers")?;
//
//     let mut modifier_sheet = SpriteSheet::new(0);
//     let modifier_path = sprites_dir.join("modifiers.webp");
//     let modifier_canvas = Canvas::new(modifier_path, SPRITES_PER_SIDE, 48, 48, 90f32);
//     modifier_sheet.add_canvas(modifier_canvas);
//
//     for file in modifier_chunk {
//         let name = file.file_stem().unwrap().to_string_lossy().into_owned();
//         modifier_sheet.add_sprite(&name, file)?;
//     }
//
//     cu::debug!("encoding modifier sprite sheet...");
//     let sizes = modifier_sheet.write()?;
//     cu::debug!("     modifiers: {} bytes", sizes[0]);
//
//     cu::debug!("writing modifier metadata...");
//     let mut metadata = Metadata::default();
//     modifier_sheet.add_metadata(&mut metadata)?;
//     let metadata = json::stringify(&metadata)?;
//     let metadata_ts = cconcat![
//         "/** Modifier => [Chunk, Position]*/",
//         "export type ModifierMetadata = Record<string,[0,number]>;",
//         format!(
//             "export const ModifierMetadata: ModifierMetadata = JSON.parse(`{}`)",
//             metadata
//         ),
//     ];
//
//     std::fs::write(
//         src_dir.join("modifier_sprite_meta.ts"),
//         metadata_ts.to_string(),
//     )?;
//     Ok(())
// }
//
// fn find_images(data_dir: &Path, profiles: &[&str]) -> cu::Result<Vec<PathBuf>> {
//     // we need to synchronously list all images to guarantee
//     // consistent ordering in the output
//     let mut out = Vec::new();
//     for profile in profiles {
//         let profile_dir = data_dir.join(profile);
//         cu::ensure!(
//             profile_dir.exists(),
//             "profile directory does not exist: '{}'",
//             profile_dir.display()
//         )?;
//
//         let mut images = Vec::new();
//         for entry in profile_dir.read_dir()? {
//             let entry = entry?;
//             let path = entry.path();
//             cu::ensure!(path.is_file(), "'{}'", path.display())?;
//             images.push(path);
//         }
//         cu::debug!("profile: {} ({} actors)", profile, images.len());
//         images.sort();
//         out.extend(images);
//     }
//     Ok(out)
// }

// cu::info!("configuring actor chunks...");
// let mut chunks = vec![
//     // chunk 0
//     find_images(icons_dir, &["CapturedActor", "Item", "PlayerItem"])?,
//     // chunk 1
//     find_images(
//         icons_dir,
//         &[
//             "Bullet",
//             "WeaponBow",
//             "WeaponLargeSword",
//             "WeaponShield",
//             "WeaponSmallSword",
//             "WeaponSpear",
//         ],
//     )?,
//     // chunk 2
//     find_images(
//         icons_dir,
//         &[
//             "ArmorHead",
//             "ArmorLower",
//             "ArmorUpper",
//             "HorseReins",
//             "HorseSaddle",
//             "CookResult",
//         ],
//     )?,
// ];
//
// let special_dir = icons_dir.join("SP");
//
// // add the fallback "dummy" image
// let dummy_path = special_dir.join("Dummy.png");
// cu::ensure!(
//     dummy_path.exists(),
//     "cannot find dummy image: '{}'",
//     dummy_path.display()
// )?;
// chunks.last_mut().unwrap().push(dummy_path);
//
// const SPRITES_PER_SIDE: u32 = 16;
//
// // print stat
// for (i, chunk) in chunks.iter().enumerate() {
//     let len = chunk.len();
//     let max = (SPRITES_PER_SIDE * SPRITES_PER_SIDE) as usize;
//     cu::info!("actor chunk {i}: {len}/{max} images");
//     cu::ensure!(len <= max, "actor chunk {i} is too big")?;
// }
    // // load the individual icons into sprite sheets
    // let mut sprite_sheets = (0..chunks.len())
    //     .map(|i| {
    //         let mut sprite_sheet = SpriteSheet::new(i as u16);
    //         let lo_res_path = sprites_dir.join(format!("chunk{i}x32.webp"));
    //         let lo_res = Canvas::new(lo_res_path, SPRITES_PER_SIDE, 32, 28, 75f32);
    //         let hi_res_path = sprites_dir.join(format!("chunk{i}x64.webp"));
    //         let hi_res = Canvas::new(hi_res_path, SPRITES_PER_SIDE, 64, 56, 90f32);
    //         sprite_sheet.add_canvas(lo_res);
    //         sprite_sheet.add_canvas(hi_res);
    //         sprite_sheet
    //     })
    //     .collect::<Vec<_>>();
    //
    // for (sheet, chunk) in sprite_sheets.iter_mut().zip(chunks) {
    //     for file in chunk {
    //         let name = file.file_stem().unwrap().to_string_lossy().into_owned();
    //         sheet.add_sprite(&name, file)?;
    //     }
    // }
    //
    // cu::debug!("encoding actor sprite sheets...");
    // for (i, sheet) in sprite_sheets.iter().enumerate() {
    //     cu::debug!("-- chunk {i}");
    //     let sizes = sheet.write()?;
    //     cu::debug!("     low resolution: {} bytes", sizes[0]);
    //     cu::debug!("     high resolution: {} bytes", sizes[1]);
    // }
    //
    // let mut metadata = Metadata::default();
    // for sheet in &sprite_sheets {
    //     sheet.add_metadata(&mut metadata)?;
    // }
fn snake_to_pascal(input: &str) -> String {
    input
        .split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}
