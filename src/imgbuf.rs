use std::{
    fs::File,
    ops::{Index, IndexMut},
};

use png::{BitDepth, ColorType, Encoder, ScaledFloat, SourceChromaticities};

#[derive(Debug)]
/// A image buffer that can be used to store the result of rendering.
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub data: Box<[u8]>,
}

impl ImageBuffer {
    /// Creates a new image buffer with the given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0; (width * height * 3) as usize].into_boxed_slice(),
        }
    }

    /// Creates a new image buffer with the given dimensions and data.
    pub fn with_data<D: Into<Box<[u8]>>>(width: u32, height: u32, data: D) -> Self {
        let data = data.into();

        assert!(
            data.len() == (width * height * 3) as usize,
            "Data length does not match dimensions"
        );

        Self {
            width,
            height,
            data,
        }
    }

    /// Saves the image buffer to a file at the given path.
    pub fn save<T: ToString>(self, path: T) -> Result<(), &'static str> {
        let file = File::create(path.to_string()).map_err(|_| "failed to create file")?;

        let mut encoder = Encoder::new(file, self.width, self.height);

        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_source_gamma(ScaledFloat::new(1.0 / 2.2));

        let source_chromaticities = SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);

        let mut writer = encoder
            .write_header()
            .map_err(|_| "failed to write image header")?;

        writer
            .write_image_data(&self.data)
            .map_err(|_| "failed to write image data")?;

        Ok(())
    }
}

impl Index<(u32, u32)> for ImageBuffer {
    type Output = [u8];

    fn index(&self, (x, y): (u32, u32)) -> &Self::Output {
        let idx = ((y * self.width + x) * 3) as usize;

        &self.data[idx..idx + 3]
    }
}

impl IndexMut<(u32, u32)> for ImageBuffer {
    fn index_mut(&mut self, (x, y): (u32, u32)) -> &mut Self::Output {
        let idx = ((y * self.width + x) * 3) as usize;

        &mut self.data[idx..idx + 3]
    }
}
