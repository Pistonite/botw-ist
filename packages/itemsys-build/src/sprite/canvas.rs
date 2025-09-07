use std::path::PathBuf;

use cu::pre::*;
use image::DynamicImage;
use image::imageops::{self, FilterType};
use webp::Encoder;

/// Canvas for drawing individual item images onto a bigger image
#[derive(Debug, Clone)]
pub struct Canvas {
    /// The output path of the sprite sheet
    output: PathBuf,
    /// Number of sprite on each side of the canvas
    ///
    /// Total number is this squared
    sprite_per_side: u32,
    /// Padding around each sprite
    padding: u32,
    /// Resolution to scale each sprite to
    scale_to: u32,

    /// The encoding quality
    quality: f32,

    image: DynamicImage,
}

impl Canvas {
    pub fn new(
        output: PathBuf,
        sprite_per_side: u32,
        outer_resolution: u32,
        inner_resolution: u32,
        quality: f32,
    ) -> Self {
        let padding2 = outer_resolution - inner_resolution;
        if padding2 % 2 != 0 {
            panic!("padding must be even");
        }
        let padding = padding2 / 2;

        let canvas_size = sprite_per_side * outer_resolution;
        let image = DynamicImage::new_rgba8(canvas_size, canvas_size);

        Self {
            output,
            sprite_per_side,
            padding,
            scale_to: inner_resolution,
            quality,
            image,
        }
    }

    pub fn load_image(
        &mut self,
        position: usize,
        image: &DynamicImage,
        use_padding: bool,
    ) -> cu::Result<()> {
        if position >= self.sprite_per_side as usize * self.sprite_per_side as usize {
            cu::bail!("position out of bounds");
        }
        let outer_res = self.scale_to + self.padding * 2;
        let (scale_to, padding) = if use_padding {
            (self.scale_to, self.padding)
        } else {
            (outer_res, 0)
        };
        let resized = DynamicImage::ImageRgba8(imageops::resize(
            image,
            scale_to,
            scale_to,
            FilterType::Lanczos3,
        ));
        let y = (position / self.sprite_per_side as usize) * (outer_res as usize);
        let y = y as u32 + padding;
        let x = (position % self.sprite_per_side as usize) * (outer_res as usize);
        let x = x as u32 + padding;
        imageops::overlay(&mut self.image, &resized, x as i64, y as i64);

        Ok(())
    }

    /// Encode a webp image and write it to a file
    ///
    /// Return the file size
    pub fn write(&self) -> cu::Result<usize> {
        let encoder = Encoder::from_image(&self.image) .map_err(|x| cu::fmterr!("{x}"));
        let encoder = cu::check!(encoder, "could not create encoder")?;

        let memory = encoder.encode_simple(false, self.quality).map_err(|x| cu::fmterr!("{x:?}"));
        let memory = cu::check!(memory, "failed to encode canvas")?;
        let len = memory.len();

        cu::fs::write(&self.output, &*memory)?;

        Ok(len)
    }
}
