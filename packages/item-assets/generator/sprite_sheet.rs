use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{bail, Context};
use image::GenericImageView;
use serde::{Deserialize, Serialize};

use crate::canvas::Canvas;
use crate::Error;

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    /// The chunk number
    pub chunk: u16,
    /// The name of the sprites
    pub sprites: Vec<String>,
    /// The canvases for the sprite sheet, each canvas is a different resolution/ config
    pub canvases: Vec<Canvas>,
}

impl SpriteSheet {
    pub fn new(chunk: u16) -> Self {
        Self {
            chunk,
            sprites: Vec::new(),
            canvases: Vec::new(),
        }
    }

    /// Add a canvas configuration for output
    pub fn add_canvas(&mut self, canvas: Canvas) {
        self.canvases.push(canvas);
    }


    /// Add the metadata of the sprite sheet to the metadata object
    ///
    /// Returns how many sprites were added
    pub fn add_metadata(&self, metadata: &mut Metadata) -> anyhow::Result<usize> {
        for (pos, name) in self.sprites.iter().enumerate() {
            metadata.register(name, self.chunk, pos)?;
        }
        Ok(self.sprites.len())
    }

    /// Load an image from a file and encode it into the sprite sheet
    pub fn add_sprite(&mut self, name: &str, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let position = {
            let position = self.sprites.len();
            self.sprites.push(name.to_string());
            position
        };
        self.load_image(path, position)?;
        Ok(())
    }
    /// Load an image file into sprite sheet canvas
    fn load_image(
        &mut self,
        path: impl AsRef<Path>,
        position: usize,
    ) -> anyhow::Result<()> {
        let path = path.as_ref();
        let image = image::open(&path).context(format!("Could not open image: {}", path.display()))?;
        let (w, h) = image.dimensions();
        if w != h {
            Err(Error::NotSquare(path.display().to_string(), w, h))?;
        }
        for canvas in &mut self.canvases {
            canvas.load_image(position, &image)?;
        }

        Ok(())
    }

    /// Write the sprite sheets to output files
    ///
    /// Returns the sizes of the canvases
    pub fn write(&self) -> anyhow::Result<Vec<usize>> {
        let mut sizes = Vec::with_capacity(self.canvases.len());
        for canvas in &self.canvases {
            sizes.push(canvas.write()?);
        }
        Ok(sizes)
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
