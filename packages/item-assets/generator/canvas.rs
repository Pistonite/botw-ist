use std::path::PathBuf;

use anyhow::{anyhow, bail};
use image::imageops::{self, FilterType};
use image::DynamicImage;
use webp::Encoder;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Canvas {
    /// The output path of the sprite sheet
    pub output: PathBuf,
    /// Number of sprite on each side of the canvas
    ///
    /// Total number is this squared
    pub sprite_per_side: u32,
    /// Padding around each sprite
    pub padding: u32,
    /// Resolution to scale each sprite to
    pub scale_to: u32,

    /// The encoding quality
    pub quality: f32,

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

    pub fn load_image(&mut self, position: usize, image: &DynamicImage) -> anyhow::Result<()> {
        if position >= self.sprite_per_side as usize * self.sprite_per_side as usize {
            bail!("position out of bounds");
        }
        let resized = DynamicImage::ImageRgba8(imageops::resize(
            image,
            self.scale_to,
            self.scale_to,
            FilterType::Lanczos3,
        ));
        let outer_res = self.scale_to + self.padding * 2;
        let y = (position / self.sprite_per_side as usize) * (outer_res as usize);
        let y = y as u32 + self.padding;
        let x = (position % self.sprite_per_side as usize) * (outer_res as usize);
        let x = x as u32 + self.padding;
        imageops::overlay(&mut self.image, &resized, x as i64, y as i64);

        Ok(())
    }

    /// Encode a webp image and write it to a file
    ///
    /// Return the file size
    pub fn write(&self) -> anyhow::Result<usize> {
        let encoder = match Encoder::from_image(&self.image) {
            Ok(e) => e,
            Err(e) => Err(anyhow!("Could not create encoder: {}", e))?,
        };

        let memory = encoder
            .encode_simple(false, self.quality)
            .map_err(Error::from)?;
        let len = memory.len();

        std::fs::write(&self.output, &*memory)?;

        Ok(len)
    }
}
