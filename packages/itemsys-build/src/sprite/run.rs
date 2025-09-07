use std::path::{Path, PathBuf};

use cu::pre::*;
use codize::cconcat;

use super::sprite_sheet::{Metadata, SpriteSheet};
use super::canvas::Canvas;


pub fn run() -> cu::Result<()> {
    let home = find_root()?;
    let icons_dir = home.join("icons");
    cu::ensure!(icons_dir.exists(), "icons deps needs to be pulled to build sprites.");

    let itemsys_dir = home.parent_abs()?.join("itemsys");
    let src_dir = itemsys_dir.join("src").join("sprite");
    cu::fs::make_dir(&src_dir)?;
    let sprites_dir = home.join("public").join("sprites");
    cu::fs::make_dir(&sprites_dir)?;

    generate_actors(&icons_dir, &sprites_dir, &src_dir)?;
    generate_modifiers(&icons_dir, &sprites_dir, &src_dir)?;

    cu::info!("done!");
    Ok(())
}

fn generate_actors(
    icons_dir: &Path,
    sprites_dir: &Path,
    src_dir: &Path,
) -> cu::Result<()> {
    cu::info!("configuring actor chunks...");
    let mut chunks = vec![
        // chunk 0
        find_images(icons_dir, &["CapturedActor", "Item", "PlayerItem"])?,
        // chunk 1
        find_images(
            icons_dir,
            &[
                "Bullet",
                "WeaponBow",
                "WeaponLargeSword",
                "WeaponShield",
                "WeaponSmallSword",
                "WeaponSpear",
            ],
        )?,
        // chunk 2
        find_images(
            icons_dir,
            &[
                "ArmorHead",
                "ArmorLower",
                "ArmorUpper",
                "HorseReins",
                "HorseSaddle",
                "CookResult",
            ],
        )?,
    ];

    let special_dir = icons_dir.join("SP");

    // add the fallback "dummy" image
    let dummy_path = special_dir.join("Dummy.png");
    cu::ensure!(dummy_path.exists(), "cannot find dummy image: '{}'", dummy_path.display());
    chunks.last_mut().unwrap().push(dummy_path);

    const SPRITES_PER_SIDE: u32 = 16;

    // print stat
    for (i, chunk) in chunks.iter().enumerate() {
        let len = chunk.len();
        let max = (SPRITES_PER_SIDE * SPRITES_PER_SIDE) as usize;
        cu::info!("actor chunk {i}: {len}/{max} images");
        cu::ensure!(len <= max, "actor chunk {i} is too big");
    }

    // load the individual icons into sprite sheets
    let mut sprite_sheets = (0..chunks.len())
        .map(|i| {
            let mut sprite_sheet = SpriteSheet::new(i as u16);
            let lo_res_path = sprites_dir.join(format!("chunk{i}x32.webp"));
            let lo_res = Canvas::new(lo_res_path, SPRITES_PER_SIDE, 32, 28, 75f32);
            let hi_res_path = sprites_dir.join(format!("chunk{i}x64.webp"));
            let hi_res = Canvas::new(hi_res_path, SPRITES_PER_SIDE, 64, 56, 90f32);
            sprite_sheet.add_canvas(lo_res);
            sprite_sheet.add_canvas(hi_res);
            sprite_sheet
        })
        .collect::<Vec<_>>();

    for (sheet, chunk) in sprite_sheets.iter_mut().zip(chunks) {
        for file in chunk {
            let name = file.file_stem().unwrap().to_string_lossy().into_owned();
            sheet.add_sprite(&name, file)?;
        }
    }

    cu::debug!("encoding actor sprite sheets...");
    for (i, sheet) in sprite_sheets.iter().enumerate() {
        cu::debug!("-- chunk {i}");
        let sizes = sheet.write()?;
        cu::debug!("     low resolution: {} bytes", sizes[0]);
        cu::debug!("     high resolution: {} bytes", sizes[1]);
    }

    let mut metadata = Metadata::default();
    for sheet in &sprite_sheets {
        sheet.add_metadata(&mut metadata)?;
    }
    let ts_chunk_type = (0..sprite_sheets.len())
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let metadata = json::stringify(&metadata)?;
    let metadata_ts = cconcat![
        // metadata for finding where an actor is
        "/** Actor => [Chunk, Position]*/",
        format!(
            "export type ActorMetadata = Record<string,[{},number]>;",
            ts_chunk_type
        ),
        format!(
            "export const ActorMetadata: ActorMetadata = JSON.parse(`{}`)",
            metadata
        ),
    ];

    cu::fs::write(
        src_dir.join("actor_meta.ts"),
        metadata_ts.to_string(),
    )?;

    Ok(())
}

fn generate_modifiers(
    icons_dir: &Path,
    sprites_dir: &Path,
    src_dir: &Path,
) -> cu::Result<()> {
    cu::info!("configuring modifier chunks...");
    const SPRITES_PER_SIDE: u32 = 8;
    let modifier_chunk = find_images(icons_dir, &["SpecialStatus"])?;
    let len = modifier_chunk.len();
    let max = (SPRITES_PER_SIDE * SPRITES_PER_SIDE) as usize;
    cu::info!("modifier chunk: {len}/{max} images");
    cu::ensure!(len <= max, "too many modifiers");

    let mut modifier_sheet = SpriteSheet::new(0);
    let modifier_path = sprites_dir.join("modifiers.webp");
    let modifier_canvas = Canvas::new(modifier_path, SPRITES_PER_SIDE, 48, 48, 90f32);
    modifier_sheet.add_canvas(modifier_canvas);

    for file in modifier_chunk {
        let name = file.file_stem().unwrap().to_string_lossy().into_owned();
        modifier_sheet.add_sprite(&name, file)?;
    }

    cu::debug!("encoding modifier sprite sheet...");
    let sizes = modifier_sheet.write()?;
    cu::debug!("     modifiers: {} bytes", sizes[0]);

    cu::debug!("writing modifier metadata...");
    let mut metadata = Metadata::default();
    modifier_sheet.add_metadata(&mut metadata)?;
    let metadata = json::stringify(&metadata)?;
    let metadata_ts = cconcat![
        "/** Modifier => [Chunk, Position]*/",
        "export type ModifierMetadata = Record<string,[0,number]>;",
        format!(
            "export const ModifierMetadata: ModifierMetadata = JSON.parse(`{}`)",
            metadata
        ),
    ];

    std::fs::write(
        src_dir.join("modifier_meta.ts"),
        metadata_ts.to_string(),
    )?;
    Ok(())
}

fn find_images(data_dir: &Path, profiles: &[&str]) -> cu::Result<Vec<PathBuf>> {
    // we need to synchronously list all images to guarantee
    // consistent ordering in the output
    let mut out = Vec::new();
    for profile in profiles {
        let profile_dir = data_dir.join(profile);
        cu::ensure!(profile_dir.exists(), "profile directory does not exist: '{}'", profile_dir.display());

        let mut images = Vec::new();
        for entry in profile_dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            cu::ensure!(path.is_file(), 
                "not a file: '{}'", path.display()
            );
            images.push(path);
        }
        cu::debug!("profile: {} ({} actors)", profile, images.len());
        images.sort();
        out.extend(images);
    }
    Ok(out)
}

/// Find the package root directory, only works when running from cargo
fn find_root() -> cu::Result<PathBuf> {
    let e = cu::fs::current_exe()?;
    let root_path = e
        .parent() // /target/release
        .and_then(|x| x.parent()) // /target
        .and_then(|x| x.parent()) // /
        .ok_or_else(|| cu::fmterr!("Could not find parent of exe"))?;
    let mut path = root_path.to_path_buf();
    // check
    path.push("package.json");
    cu::ensure!(path.exists(), "could not find package.json, make sure you are running through the taskfile or cargo.");
    match cu::fs::read_string(&path) {
        Ok(x) if x.contains(r#""name": "skybook-itemsys-build","#) => {
            // found the package
        }
        _ => {
            cu::bail!(
                "could not verify the root directory is correct. make sure you are running through the taskfile or cargo."
            );
        }
    };
    path.pop();
    path.normalize()
}
