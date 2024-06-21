use std::fmt::Debug;

use crate::{resources::Resources, vector::Color};

/// A texture that can be used by materials in a scene.
pub trait Texture: Debug + Send + Sync {
    /// Get the color of the texture at a given UV coordinate.
    fn color(&self, resources: &Resources, u: f64, v: f64) -> Color;
}
