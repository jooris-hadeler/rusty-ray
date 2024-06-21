use crate::{resources::Resources, texture::Texture, vector::Color};

#[derive(Debug)]
/// A solid color texture.
pub struct SolidTexture {
    /// The color of the texture.
    color: Color,
}

impl SolidTexture {
    /// Create a new solid color texture.
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidTexture {
    fn color(&self, _resources: &Resources, _u: f64, _v: f64) -> Color {
        self.color
    }
}
