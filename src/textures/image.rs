use crate::{imgbuf::ImageBuffer, resources::Resources, texture::Texture, vec3, vector::Color};

#[derive(Debug)]
/// A texture that uses an image as its source.
pub struct ImageTexture {
    /// The image buffer of the texture.
    image: ImageBuffer,
}

impl ImageTexture {
    /// Create a new image texture with the given image buffer.
    pub fn new(image: ImageBuffer) -> Self {
        Self { image }
    }
}

impl Texture for ImageTexture {
    fn color(&self, _resources: &Resources, u: f64, v: f64) -> Color {
        let x = self.image.width as f64 * u;
        let y = self.image.height as f64 * v;

        let pixel = &self.image[(x as u32, y as u32)];
        let r = pixel[0] as f64 / 255.0;
        let g = pixel[1] as f64 / 255.0;
        let b = pixel[2] as f64 / 255.0;

        vec3!(r, g, b)
    }
}
