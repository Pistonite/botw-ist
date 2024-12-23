//! Step 3: encode frames into image
//!     cargo run --release <object>
//! This will find target/decode/<object>/ and encode the image to target/encode/<object>.webp
//! 
//! --fast is fast but lower quality for testing parameters

use std::{collections::{BTreeMap, BTreeSet}, path::{Path, PathBuf}, sync::Arc};

use clap::Parser;
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use anyhow::{anyhow, bail};
use serde::Deserialize;
use serde_yaml_ng::Value;
use webp_animation::{AnimParams, ColorMode, Encoder, EncoderOptions, EncodingConfig, EncodingType, LossyEncodingConfig};

#[derive(Debug, Deserialize)]
struct Config {
    objects: BTreeMap<String, Value>, // the value is unused here
    encoder: EncoderProfiles,
}
#[derive(Debug, Deserialize)]
struct EncoderProfiles {
    fast: EncoderProfile,
    best: EncoderProfile
}
#[derive(Debug, Deserialize)]
struct EncoderProfile {
    filter_strength: usize,
    filter_sharpness: usize,
    alpha_quality: usize,
    pass: usize,
    quality: f32,
    method: usize
}

#[derive(Debug, Clone, Parser)]
struct Cli {
    objects: Vec<String>,

    #[clap(long)]
    fast: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    if cli.fast {
        println!("using fast config");
    }
    let config_file = std::fs::read("config.yaml")?;
    let config: Config = serde_yaml_ng::from_slice(&config_file)?;
    let mut expanded_objects = BTreeSet::new();
    for obj in cli.objects {
        if let Some(object) = obj.strip_suffix('*') {
            if object.is_empty() {
                for key in config.objects.keys() {
                    let name = key.splitn(2, ":").skip(1).next().unwrap();
                    expanded_objects.insert(name.to_string());
                }
            } else {
                for key in config.objects.keys() {
                    let name = key.splitn(2, ":").skip(1).next().unwrap();
                    if name.starts_with(object) {
                        expanded_objects.insert(name.to_string());
                    }
                }
            }
        } else {
            for key in config.objects.keys() {
                let mut parts = key.splitn(2, ":");
                let profile = parts.next().unwrap();
                let name = parts.next().unwrap();
                if name == obj || profile == obj {
                    expanded_objects.insert(name.to_string());
                }
            }
        }
    }
    let profile = if cli.fast { config.encoder.fast } else { config.encoder.best};
    let profile = Arc::new(profile);
    let mut handles = Vec::new();
    for object in expanded_objects {
        let profile = Arc::clone(&profile);
        let handle = std::thread::spawn(move || {
            encode(&object, &profile).map_err(|x| format!("{x:?}"))
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Err(_) => {},
            Ok(x) => x.map_err(|x| anyhow!(x))?
        }
    }

    Ok(())
}

fn encode(object: &str, profile: &EncoderProfile) -> anyhow::Result<()> {
    println!("input: {object}");
    let frames_dir = Path::new("target").join("decode").join(object);
    let output_dir = Path::new("target").join("encode");
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir)?;
    }
    let mut output_name = PathBuf::from(frames_dir.file_name().unwrap());
    output_name.set_extension("webp");
    let input_path = frames_dir.join("frame_0.png");
    if !input_path.exists() {
        bail!("[{object}] cannot find first frame");
    }
    let image = image::open(input_path)?;
    let (w,h) = image.dimensions();

    println!("[{object}] loading and transforming frames...");
    
    let mut frame_images = Vec::new();
    for i in 0.. {
        let input_path = frames_dir.join(format!("frame_{i}.png"));
        if !input_path.exists() {
            break;
        }
        let image = add_alpha_to_image(image::open(input_path)?);
        frame_images.push(image);
    }
    println!("[{object}] loaded {} frames, encoding...", frame_images.len());

    let lossy_config = LossyEncodingConfig {
        target_size: 0, // off
        target_psnr: 0f32, // off
        segments: 1,
        sns_strength: 100,
        filter_strength: profile.filter_strength,
        filter_sharpness: profile.filter_sharpness,
        filter_type: 1,
        autofilter: true,
        alpha_compression: true,
        alpha_filtering: 2,
        alpha_quality: profile.alpha_quality,
        pass: profile.pass,
        show_compressed: false,
        preprocessing: true,
        partitions: 0,
        partition_limit: 0,
        use_sharp_yuv: true
    };
    let encoding_config = EncodingConfig {
        encoding_type: EncodingType::Lossy(lossy_config),
        quality: profile.quality,
        method: profile.method,
    };
    let anim_params = AnimParams {
        loop_count: 0 // inf
    };
    let encoder_options = EncoderOptions {
        anim_params,
        minimize_size: true,
        verbose: false,
        color_mode: ColorMode::Rgba,
        encoding_config: Some(encoding_config),
        ..Default::default()
    };
    let mut encoder = Encoder::new_with_options((w,h), encoder_options)?;
    for (i, image) in frame_images.iter().enumerate() {
        if i % 30 == 0 {
            println!("[{object}] encoding frame {i}");
        }
        encoder.add_frame(image.as_bytes(), timestamp(i as u32))?;
    }

    let encoded = encoder.finalize(timestamp(frame_images.len() as u32))?;
    let size = encoded.len();
    let output_path = output_dir.join(output_name);
    std::fs::write(output_path, &*encoded)?;
    println!("[{object}] done ({size} bytes)");

    Ok(())
 
}

fn timestamp(i: u32) -> i32 {
    // 30 fps
    let whole = i as i32 / 3 * 100;
    match i % 3 {
        0 => whole,
        1 => whole+ 33,
        2 => whole+ 67,
        _ => unreachable!()
    }
}

fn add_alpha_to_image(input: DynamicImage) -> DynamicImage{
    let (w, h) = input.dimensions();
    let mut output = DynamicImage::new_rgba8(w, h);
    for x in 0..w {
        for y in 0..h {
            let [r,g,b,_] = input.get_pixel(x, y).0;
            let (r,g,b,a) = add_alpha(r,g,b);
            output.put_pixel(x, y, Rgba([r,g,b,a]))
        }
    }
    output
}

fn add_alpha(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {

    let rf = r as f64;
    let gf = g as f64;
    let bf = b as f64;
   let alpha_weight = (rf * rf + gf * gf + bf * bf).sqrt();


   // let lumi = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
    let alpha_weight = (alpha_weight / 255.0).min(1.0);
    // sqrt to curve it?
    let alpha = alpha_weight;//.sqrt();

    #[inline]
    fn fix(x: u8, a: f64) -> u8 {
        // since alpha max is 1.0, this should not overflow
        // check just in case
        let xa = ((x as f64 / a)).floor();
        if xa > 255.0 {
            255
        } else {
            xa as u8
        }
    }

    let a = (alpha * 255.0).floor() as u8;
    (fix(r, alpha), fix(g, alpha), fix(b, alpha), a)
}