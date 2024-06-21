use std::fmt::Debug;

use crate::{
    ray::{Intersection, Ray},
    resources::Resources,
    vector::Color,
};

/// A material that can be assigned to an object in a scene.
pub trait Material: Debug + Send + Sync {
    /// Scatter a ray off the material at a given intersection point.
    fn scatter(&self, resources: &Resources, ray: &Ray, hit: &Intersection)
        -> Option<(Ray, Color)>;

    /// Emit light from the material at a given intersection point.
    fn emit(&self, _resources: &Resources, _hit: &Intersection) -> Color {
        Color::ZERO
    }
}
