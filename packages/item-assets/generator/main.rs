// use clap::Parser;

use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail};
use canvas::Canvas;
use codize::{cblock, cconcat, clist};

mod error;
use error::Error;

mod sprite_sheet;
use sprite_sheet::{Metadata, SpriteSheet};

mod canvas;

fn main() {
    if let Err(e) = generate() {
        eprintln!("error: {:?}", e);
    }
}

fn generate() -> anyhow::Result<()> {
    let home = find_root()?;
    let icons_dir = home.join("icons");
    if !icons_dir.exists() {
        bail!("icons directory does not exist: {}", icons_dir.display());
    }

    let generated_dir = home.join("src").join("generated").join("sprites");
    if !generated_dir.exists() {
        std::fs::create_dir_all(&generated_dir)?;
    }
    println!("configuring actor chunks...");

    let mut chunks = vec![
        // chunk 0
        find_images(&icons_dir, &["CapturedActor", "Item", "PlayerItem"])?,
        // chunk 1
        find_images(
            &icons_dir,
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
            &icons_dir,
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
    if !dummy_path.exists() {
        bail!("Dummy image does not exist: {}", dummy_path.display());
    }
    println!("adding dummy image to last chunk");
    chunks.last_mut().unwrap().push(dummy_path);

    // print stat
    for (i, chunk) in chunks.iter().enumerate() {
        println!("chunk {}: {} images", i, chunk.len());
    }
    println!("loading actor sprites...");

    // load the individual icons into sprite sheets
    let mut sprite_sheets = (0..chunks.len())
        .map(|i| {
            let mut sprite_sheet = SpriteSheet::new(i as u16);
            let lo_res_path = generated_dir.join(format!("chunk{}x32.webp", i));
            let lo_res = Canvas::new(lo_res_path, 16, 32, 28, 75f32);
            let hi_res_path = generated_dir.join(format!("chunk{}x64.webp", i));
            let hi_res = Canvas::new(hi_res_path, 16, 64, 56, 90f32);
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

    println!("encoding actor sprite sheets...");
    for (i, sheet) in sprite_sheets.iter().enumerate() {
        println!("-- chunk {}", i);
        let sizes = sheet.write()?;
        println!("     low resolution: {} bytes", sizes[0]);
        println!("     high resolution: {} bytes", sizes[1]);
    }

    println!("writing actor metadata...");
    let mut metadata = Metadata::default();
    for sheet in &sprite_sheets {
        sheet.add_metadata(&mut metadata)?;
    }
    let ts_chunk_type = (0..sprite_sheets.len())
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("|");
    let metadata = serde_json::to_string(&metadata)?;
    let metadata_ts = cconcat![
        // imports
        cconcat!((0..sprite_sheets.len())
            .map(|i| { format!("import chunk{i}x32 from \"./sprites/chunk{i}x32.webp?url\";") })),
        cconcat!((0..sprite_sheets.len())
            .map(|i| { format!("import chunk{i}x64 from \"./sprites/chunk{i}x64.webp?url\";") })),
        // chunkmap classnames
        cblock! {
            "export const ActorChunkClasses = {",
            [
                clist!("" => (0..sprite_sheets.len()).map(|i| {
                    format!("\".sprite-chunk{i}x32\": {{ backgroundImage: `url(${{chunk{i}x32}})` }},")
                })),
                clist!("" => (0..sprite_sheets.len()).map(|i| {
                    format!("\".sprite-chunk{i}x64\": {{ backgroundImage: `url(${{chunk{i}x64}})` }},")
                })),
                clist!("" => (0..sprite_sheets.len()).map(|i| {
                    format!("\".sprite-mask-chunk{i}x32\": {{ maskImage: `url(${{chunk{i}x32}})` }},")
                })),
                clist!("" => (0..sprite_sheets.len()).map(|i| {
                    format!("\".sprite-mask-chunk{i}x64\": {{ maskImage: `url(${{chunk{i}x64}})` }},")
                })),
            ],
            "} as const;"
        },
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

    let metadata_dir = generated_dir.parent().unwrap();

    std::fs::write(
        metadata_dir.join("ActorMetadata.ts"),
        metadata_ts.to_string(),
    )?;

    println!("configuring modifier chunks...");
    let modifier_chunk = find_images(&icons_dir, &["SpecialStatus"])?;
    let mut modifier_sheet = SpriteSheet::new(0);
    let modifier_path = generated_dir.join("modifiers.webp");
    let modifier_canvas = Canvas::new(modifier_path, 8, 48, 48, 90f32);
    modifier_sheet.add_canvas(modifier_canvas);

    println!("loading modifier sprites...");
    for file in modifier_chunk {
        let name = file.file_stem().unwrap().to_string_lossy().into_owned();
        modifier_sheet.add_sprite(&name, file)?;
    }

    println!("encoding modifier sprite sheet...");
    let sizes = modifier_sheet.write()?;
    println!("     modifiers: {} bytes", sizes[0]);

    println!("writing modifier metadata...");
    let mut metadata = Metadata::default();
    modifier_sheet.add_metadata(&mut metadata)?;
    let metadata = serde_json::to_string(&metadata)?;
    let metadata_ts = cconcat![
        // imports
        "import modifiers from \"./sprites/modifiers.webp?url\";",
        // chunkmap classnames
        cblock! {
            "export const ModifierChunkClasses = {",
            [
                "\".sprite-modifiers\": { backgroundImage: `url(${modifiers})` },",
            ],
            "} as const;"
        },
        "/** Modifier => [Chunk, Position]*/",
        "export type ModifierMetadata = Record<string,[0,number]>;",
        format!(
            "export const ModifierMetadata: ModifierMetadata = JSON.parse(`{}`)",
            metadata
        ),
    ];

    std::fs::write(
        metadata_dir.join("ModifierMetadata.ts"),
        metadata_ts.to_string(),
    )?;

    println!("done!");

    Ok(())
}

fn find_images(data_dir: &Path, profiles: &[&str]) -> anyhow::Result<Vec<PathBuf>> {
    // we need to synchronously list all images to guarantee
    // consistent ordering in the output
    let mut out = Vec::new();
    for profile in profiles {
        let profile_dir = data_dir.join(profile);
        if !profile_dir.exists() {
            bail!(
                "Profile directory does not exist: {}",
                profile_dir.display()
            );
        }

        let mut images = Vec::new();
        for entry in profile_dir.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                bail!("Not a file: {}", path.display());
            }
            images.push(path);
        }
        println!("profile: {} ({} actors)", profile, images.len());
        images.sort();
        out.extend(images);
    }
    Ok(out)
}

/// Find the package root directory, only works when running from cargo
fn find_root() -> anyhow::Result<PathBuf> {
    let e = std::env::current_exe()?;
    let root_path = e
        .parent() // /target/release
        .and_then(|x| x.parent()) // /target
        .and_then(|x| x.parent()) // /
        .ok_or_else(|| anyhow!("Could not find parent of exe"))?;
    let mut path = root_path.to_path_buf();
    // check
    path.push("package.json");
    if !path.exists() {
        bail!("could not find package.json. make sure you are running the generator with cargo");
    }
    match std::fs::read_to_string(&path) {
        Ok(x) if x.contains("skybook-item-assets") => {
            // found the package
        }
        _ => {
            bail!("could not verify the root directory is correct. make sure you are running the generator with cargo");
        }
    };
    path.pop();
    Ok(path)
}
