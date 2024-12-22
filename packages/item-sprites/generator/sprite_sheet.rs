use std::collections::BTreeMap;
use std::path::Path;
use std::sync::{Arc, RwLock};

use anyhow::{anyhow, bail, Context};
use image::imageops::{self, FilterType};
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use webp::Encoder;

use crate::Error;

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    /// The chunk number
    pub chunk: u16,
    /// The name of the sprites
    pub sprites: Arc<RwLock<Vec<String>>>,
    /// The low resolution image
    pub image_lo_res: Arc<RwLock<DynamicImage>>,
    /// The high resolution image
    pub image_hi_res: Arc<RwLock<DynamicImage>>,
}

impl SpriteSheet {
    pub fn new(chunk: u16) -> Self {
        let lo_res = DynamicImage::new_rgba8(SIZE * LO_RES, SIZE * LO_RES);
        let hi_res = DynamicImage::new_rgba8(SIZE * HI_RES, SIZE * HI_RES);
        Self {
            chunk,
            sprites: Arc::new(RwLock::new(Vec::new())),
            image_lo_res: Arc::new(RwLock::new(lo_res)),
            image_hi_res: Arc::new(RwLock::new(hi_res)),
        }
    }
    /// Add the metadata of the sprite sheet to the metadata object
    ///
    /// Returns how many sprites were added
    pub fn add_metadata(&self, metadata: &mut Metadata) -> anyhow::Result<usize> {
        let sprites = self.sprites.read().map_err(Error::from)?;
        for (pos, name) in sprites.iter().enumerate() {
            metadata.register(name, self.chunk, pos)?;
        }
        Ok(sprites.len())
    }

    /// Load an image from a file and encode it into the sprite sheet
    pub fn add_sprite(&self, name: &str, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let position = {
            let mut sprites = self.sprites.write().map_err(Error::from)?;
            if sprites.len() >= MAX_SPRITES as usize {
                Err(Error::TooManySprites(self.chunk))?;
            }
            let position = sprites.len();
            sprites.push(name.to_string());
            position
        };
        load_image_into_sprite_sheet(path, position, &self.image_lo_res, &self.image_hi_res)?;
        Ok(())
    }

    /// Write the sprite sheet to a directory
    /// The file format is `chunk{id}x{res}.webp`. `{res}` is either `32` or `64`
    ///
    /// Returns the sizes of the lo res and hi res images
    pub fn write_to_directory(&self, dir: impl AsRef<Path>) -> anyhow::Result<(usize, usize)> {
        let dir = dir.as_ref();
        let lo_res_path = dir.join(format!("chunk{}x32.webp", self.chunk));
        let hi_res_path = dir.join(format!("chunk{}x64.webp", self.chunk));
        let lo_size = {
            let lo_res = self.image_lo_res.read().map_err(Error::from)?;
            encode_and_write_image(&lo_res_path, &lo_res, false)?
        };
        let hi_size = {
            let hi_res = self.image_hi_res.read().map_err(Error::from)?;
            encode_and_write_image(&hi_res_path, &hi_res, true)?
        };
        Ok((lo_size, hi_size))
    }
}

/// The data of a sprite sheet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[repr(transparent)]
#[serde(transparent)]
pub struct Metadata(BTreeMap<String, (u16, u16)>);
impl Metadata {
    pub fn register(&mut self, name: &str, chunk: u16, position: usize) -> anyhow::Result<()> {
        if self
            .0
            .insert(name.to_string(), (chunk, position.try_into()?))
            .is_some()
        {
            bail!("Sprite {name} registered more than once");
        }
        Ok(())
    }
}

/// Number of sprites in each row and column of the sprite sheet
pub const SIZE: u32 = 16;
pub const MAX_SPRITES: u32 = SIZE * SIZE;
pub const LO_RES: u32 = 32;
pub const HI_RES: u32 = 64;

/// Load an image file into sprite sheet canvas
fn load_image_into_sprite_sheet(
    path: impl AsRef<Path>,
    position: usize,
    lo_res: &RwLock<DynamicImage>,
    hi_res: &RwLock<DynamicImage>,
) -> anyhow::Result<()> {
    let path = path.as_ref();
    let image = image::open(&path).context(format!("Could not open image: {}", path.display()))?;
    let (w, h) = image.dimensions();
    if w != h {
        Err(Error::NotSquare(path.display().to_string(), w, h))?;
    }
    load_image_into_sprite_sheet_with_res(&image, position, 28, LO_RES, lo_res)?;
    load_image_into_sprite_sheet_with_res(&image, position, 56, HI_RES, hi_res)?;

    Ok(())
}

/// Load an image file into sprite sheet canvas with a specific resolution
///
/// Inner resolution is what the image will be scaled to, and outer resolution is the size of the
/// sprite. The padding needs to be even on both sides.
fn load_image_into_sprite_sheet_with_res(
    image: &DynamicImage,
    position: usize,
    inner_res: u32,
    outer_res: u32,
    canvas: &RwLock<DynamicImage>,
) -> anyhow::Result<()> {
    let padding2 = outer_res - inner_res;
    if padding2 % 2 != 0 {
        Err(Error::InvalidPadding(inner_res, outer_res))?;
    }
    let padding = padding2 / 2;

    let resized =
        DynamicImage::ImageRgba8(imageops::resize(image, inner_res, inner_res, FilterType::Lanczos3));

    let y = (position / SIZE as usize) * (outer_res as usize);
    let y = y as u32 + padding;
    let x = (position % SIZE as usize) * (outer_res as usize);
    let x = x as u32 + padding;

    {
        let mut canvas = canvas.write().map_err(Error::from)?;
        imageops::overlay(&mut *canvas, &resized, x as i64, y as i64);
    }

    Ok(())
}

/// Encode a webp image and write it to a file
///
/// Return the file size
fn encode_and_write_image(path: impl AsRef<Path>, image: &DynamicImage, high_quality: bool) -> anyhow::Result<usize> {
    let encoder = match Encoder::from_image(image) {
        Ok(e) => e,
        Err(e) => Err(anyhow!("Could not create encoder: {}", e))?,
    };

    let memory = if high_quality {
        encoder.encode_simple(false, 90.0).map_err(Error::from)?
    } else {
        encoder.encode_simple(false, 75.0).map_err(Error::from)?
    };
    let len = memory.len();

    std::fs::write(path, &*memory)?;

    Ok(len)
}
