use crate::{
    material::Material,
    ray::Intersection,
    resources::{Resources, TextureId},
    vector::Color,
};

#[derive(Debug)]
/// A material that emits light.
pub struct DiffuseLightMaterial {
    /// The texture of the material.
    texture: TextureId,
}

impl DiffuseLightMaterial {
    /// Create a new diffuse light material with the given texture.
    pub fn new(texture: TextureId) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLightMaterial {
    fn emit(&self, resources: &Resources, hit: &Intersection) -> Color {
        resources[self.texture].color(resources, hit.u, hit.v)
    }
}
