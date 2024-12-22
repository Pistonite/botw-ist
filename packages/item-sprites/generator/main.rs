// use clap::Parser;

use std::path::{Path, PathBuf};

use anyhow::bail;
use codize::{cblock, cconcat, clist};
use item_sprites_generator::{Metadata, SpriteSheet};
use threadpool::ThreadPool;

fn main() {
    if let Err(e) = generate() {
        eprintln!("Error: {:?}", e);
    }
}

// #[derive(Debug, Clone, Parser)]
// struct Cli {
//     /// The secret URL used to download the sprites
//     #[clap(short, long)]
//     pub secret: String,
//
//     /// If set, generate the sprites instead of downloading them
//     /// from secrets. Requires the input image files at /packages/item-sprites/data
//     #[clap(long, conflicts_with("secret"))]
//     pub generate: bool,
// }

fn generate() -> anyhow::Result<()> {
    let home = item_sprites_generator::find_home()?;
    let data = home.join("data");
    if !data.exists() {
        bail!("Data directory does not exist: {}", data.display());
    }

    let sprites_dir = home.join("src").join("sprites");
    if !sprites_dir.exists() {
        std::fs::create_dir_all(&sprites_dir)?;
    }
    println!("configuring chunks...");

    let mut chunks = vec![
        // chunk 0
        find_images(&data, &["CapturedActor", "Item", "PlayerItem"])?,
        // chunk 1
        find_images(
            &data,
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
            &data,
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
    let dummy_path = data.join("Dummy.png");
    if !dummy_path.exists() {
        bail!("Dummy image does not exist: {}", dummy_path.display());
    }
    println!("adding dummy image to last chunk");
    chunks.last_mut().unwrap().push(dummy_path);

    for (i, chunk) in chunks.iter().enumerate() {
        println!("chunk {}: {} images", i, chunk.len());
        if chunk.len() > item_sprites_generator::MAX_SPRITES as usize {
            bail!("Too many sprites in chunk {}: {}", i, chunk.len());
        }
    }

    println!("loading sprites...");
    let pool = ThreadPool::new(num_cpus::get().saturating_sub(1).max(1));
    let sprite_sheets = (0..chunks.len())
        .map(|i| SpriteSheet::new(i as u16))
        .collect::<Vec<_>>();
    let mut handles = Vec::new();

    for (sheet, chunk) in sprite_sheets.iter().zip(chunks) {
        let sheet = sheet.clone();
        for file in chunk {
            let name = file.file_stem().unwrap().to_string_lossy().into_owned();
            let sheet = sheet.clone();
            let (tx, rx) = oneshot::channel();
            pool.execute(move || {
                let _ = tx.send(sheet.add_sprite(&name, file));
            });
            handles.push(rx);
        }
    }

    for handle in handles {
        handle.recv()??;
    }

    pool.join();

    println!("encoding sprite sheets...");
    for (i, sheet) in sprite_sheets.iter().enumerate() {
        let (lo_size, hi_size) = sheet.write_to_directory(&sprites_dir)?;
        println!("-- chunk {}", i);
        println!("     low resolution: {} bytes", lo_size);
        println!("     high resolution: {} bytes", hi_size);
    }

    println!("writing metadata...");
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
        cconcat!((0..sprite_sheets.len())
            .map(|i| { format!("import chunk{i}x32 from \"./chunk{i}x32.webp?url\";") })),
        cconcat!((0..sprite_sheets.len())
            .map(|i| { format!("import chunk{i}x64 from \"./chunk{i}x64.webp?url\";") })),
        cblock! {
            "export const ChunkMap = {",
            [
                clist!("" => (0..sprite_sheets.len()).map(|i| {
                    format!("\".sprite-chunk{i}x32\": {{ backgroundImage: `url(${{chunk{i}x32}})` }},")
                })),
                clist!("" => (0..sprite_sheets.len()).map(|i| {
                    format!("\".sprite-chunk{i}x64\": {{ backgroundImage: `url(${{chunk{i}x64}})` }},")
                })),
            ],
            "} as const;"
        },
        "/** Sprite metadata, Actor => [Chunk, Position]*/",
        format!(
            "export type Metadata = Record<string,[{},number]>;",
            ts_chunk_type
        ),
        format!(
            "export const Metadata: Metadata = JSON.parse(`{}`)",
            metadata
        ),
    ];

    std::fs::write(sprites_dir.join("metadata.ts"), metadata_ts.to_string())?;

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
