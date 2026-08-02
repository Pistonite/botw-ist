use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use codize::cconcat;
use cu::pre::*;
use itertools::Itertools as _;

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
            let name = cu::check!(
                path.file_stem(),
                "cannot get file stem for path '{}'",
                path.display()
            )?;
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
        paths.sort_by_key(|x| x.file_stem().map(|x| x.to_os_string()));
        out.insert(chunk_name.clone(), paths);
    }
    Ok(out)
}

fn render_group(
    config: &Config,
    chunks: &BTreeMap<String, Vec<PathBuf>>,
    group_name: &str,
    group: &RenderGroupConfig,
) -> cu::Result<()> {
    cu::info!("rendering group '{group_name}'");
    if group_name.is_empty() {
        cu::bail!("group name cannot be empty");
    }

    let mut metadata = Metadata::default();
    let mut seen_emit_paths = BTreeSet::default();
    let sprites_per_side = group.sprites_per_side;

    for (i, chunk_config) in group.chunks.iter().enumerate() {
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
            )?;
            spritesheet.add_canvas(canvas);
        }

        for file in input_files {
            let name = cu::check!(
                file.file_stem(),
                "failed to get file stem name from '{}'",
                file.display()
            )?;
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
    let meta_type_name = {
        let mut x = snake_to_pascal(group_name);
        x.push_str("Metadata");
        x
    };
    let meta_per_side_constant = format!("{}_NUM_PER_SIDE", group_name.to_uppercase());

    cu::info!("emitting metadata for '{group_name}': {meta_emit_name}");

    let ts_chunk_type = (0..group.chunks.len()).map(|i| i.to_string()).join("|");
    let metadata = json::stringify(&metadata)?;
    let metadata_ts = cconcat![
        "/** FileName => [ChunkId, Position]*/",
        format!("export type {meta_type_name} = Record<string,[{ts_chunk_type},number]>;",),
        format!("export const {meta_type_name}: {meta_type_name} = JSON.parse(`{metadata}`)",),
        format!("export const {meta_per_side_constant} = {sprites_per_side} as const;",),
    ];

    cu::fs::write(meta_emit_path, metadata_ts.to_string())?;

    Ok(())
}

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
