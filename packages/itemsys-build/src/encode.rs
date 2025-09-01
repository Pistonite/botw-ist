//! Step 3: encode frames into image
//!     cargo run --release <object>
//! This will find target/decode/<object>/ and encode the image to target/encode/<object>.webp

use cu::pre::*;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use image::imageops::{self, FilterType};
use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use webp_animation::{
    AnimParams, ColorMode, Encoder, EncoderOptions, EncodingConfig, EncodingType,
    LossyEncodingConfig,
};

#[derive(Debug, Deserialize)]
struct Config {
    objects: BTreeMap<String, yaml::Value>, // the value is unused here
    encoder: EncoderProfile,
}
#[derive(Debug, Deserialize)]
struct EncoderProfile {
    base_dimension: u32,
    target_dimension: u32,
    filter_strength: usize,
    filter_sharpness: usize,
    alpha_filtering: usize,
    alpha_quality: usize,
    pass: usize,
    quality: f32,
    method: usize,
    segments: usize,
}

#[derive(Debug, Clone, AsRef, clap::Parser)]
pub struct Cmd {
    objects: Vec<String>,
    #[clap(flatten)]
    #[as_ref]
    common: cu::cli::Flags,
}

pub async fn run(mut args: Cmd) -> cu::Result<()> {
    // load config
    let config_file = include_str!("../Animate.yaml");
    let config= yaml::parse::<Config>(&config_file)?;
    let mut expanded_objects = BTreeSet::new();

    // parse inputs
    if args.objects.is_empty() {
        args.objects.push("*".to_string());
    }
    for obj in args.objects {
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

    // encode each object
    let profile = Arc::new(config.encoder);
    let mut handles = Vec::with_capacity(expanded_objects.len());
    let pool = cu::co::pool(0);
    let bar = cu::progress_bar_lowp(expanded_objects.len(), "encoding objects");
    for object in expanded_objects {
        let profile = Arc::clone(&profile);
        let handle = pool.spawn(async move {
            encode(&object, &profile)?;
            cu::Ok(object)
        });
        handles.push(handle);
    }

    let mut set = cu::co::set(handles);
    let mut count = 0;
    while let Some(result) = set.next().await {
        let object = result??;
        count += 1;
        cu::progress!(&bar, count, "{object}");
    }

    Ok(())
}

fn encode(object: &str, profile: &EncoderProfile) -> cu::Result<()> {
    let frames_dir = Path::new("target").join("decode").join(object);
    let output_dir = Path::new("target").join("encode");
    cu::fs::make_dir(&output_dir)?;
    let mut output_name = PathBuf::from(frames_dir.file_name().unwrap());
    output_name.set_extension("webp");
    let input_path = frames_dir.join("frame_0.png");
    cu::ensure!(input_path.exists(), "cannot find first frame for {object}");
    let image = image::open(input_path)?;
    let (w, h) = image.dimensions();
    // save the first frame as png in original resolution
    let first_frame = process_image(&image, w, h);
    let mut output_png = output_name.clone();
    output_png.set_extension("png");
    first_frame.save(output_dir.join(output_png))?;
    cu::debug!("[{object}] saved first frame");

    // try to avoid precision loss when converting to target dimension
    let ratio = profile.target_dimension as f64 / profile.base_dimension as f64;
    let target_w = if w == profile.base_dimension {
        profile.target_dimension
    } else {
        (w as f64 * ratio) as u32
    };
    let target_h = if w == h {
        target_w
    } else {
        (h as f64 * ratio) as u32
    };

    cu::debug!("[{object}] webp dimension will be {target_w}x{target_h}");
    cu::debug!("[{object}] loading and transforming frames...");

    let mut frame_images = Vec::new();
    for i in 0.. {
        let input_path = frames_dir.join(format!("frame_{i}.png"));
        if !input_path.exists() {
            break;
        }
        let image = image::open(input_path)?;
        let image = process_image(&image, target_w, target_h);
        frame_images.push(image);
    }

    let bar = cu::progress_bar(frame_images.len(), format!("encoding {object}"));

    let lossy_config = LossyEncodingConfig {
        target_size: 0,    // off
        target_psnr: 0f32, // off
        segments: profile.segments,
        sns_strength: 100,
        filter_strength: profile.filter_strength,
        filter_sharpness: profile.filter_sharpness,
        filter_type: 1,
        autofilter: true,
        alpha_compression: true,
        alpha_filtering: profile.alpha_filtering,
        alpha_quality: profile.alpha_quality,
        pass: profile.pass,
        show_compressed: false,
        preprocessing: true,
        partitions: 0,
        partition_limit: 0,
        use_sharp_yuv: true,
    };
    let encoding_config = EncodingConfig {
        encoding_type: EncodingType::Lossy(lossy_config),
        quality: profile.quality,
        method: profile.method,
    };
    let anim_params = AnimParams {
        loop_count: 0, // inf
    };
    let encoder_options = EncoderOptions {
        anim_params,
        minimize_size: true,
        verbose: false,
        color_mode: ColorMode::Rgba,
        encoding_config: Some(encoding_config),
        ..Default::default()
    };
    let mut encoder = Encoder::new_with_options((target_w, target_h), encoder_options)?;
    for (i, image) in frame_images.iter().enumerate() {
        cu::progress!(&bar, i);
        encoder.add_frame(image.as_bytes(), timestamp(i as u32))?;
    }

    let encoded = encoder.finalize(timestamp(frame_images.len() as u32))?;
    let size = encoded.len();
    let output_path = output_dir.join(output_name);
    cu::fs::write(output_path, &*encoded)?;
    cu::progress_done!(&bar, "finished {object}: {size} bytes");

    Ok(())
}

fn timestamp(i: u32) -> i32 {
    // 30 fps
    let whole = i as i32 / 3 * 100;
    match i % 3 {
        0 => whole,
        1 => whole + 33,
        2 => whole + 67,
        _ => unreachable!(),
    }
}

fn process_image(input: &DynamicImage, target_w: u32, target_h: u32) -> DynamicImage {
    let (w, h) = input.dimensions();
    let mut output = DynamicImage::new_rgba8(w, h);
    for x in 0..w {
        for y in 0..h {
            let [r, g, b, _] = input.get_pixel(x, y).0;
            let (r, g, b, a) = add_alpha(r, g, b);
            output.put_pixel(x, y, Rgba([r, g, b, a]))
        }
    }
    let resized = imageops::resize(&output, target_w, target_h, FilterType::Lanczos3);
    resized.into()
}

fn add_alpha(r: u8, g: u8, b: u8) -> (u8, u8, u8, u8) {
    let rf = r as f64;
    let gf = g as f64;
    let bf = b as f64;
    let alpha_weight = (rf * rf + gf * gf + bf * bf).sqrt();

    // let lumi = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
    let alpha_weight = (alpha_weight / 255.0).min(1.0);
    // sqrt to curve it?
    let alpha = alpha_weight; //.sqrt();

    #[inline]
    fn fix(x: u8, a: f64) -> u8 {
        // since alpha max is 1.0, this should not overflow
        // check just in case
        let xa = (x as f64 / a).floor();
        if xa > 255.0 {
            255
        } else {
            xa as u8
        }
    }

    let a = (alpha * 255.0).floor() as u8;
    (fix(r, alpha), fix(g, alpha), fix(b, alpha), a)
}
