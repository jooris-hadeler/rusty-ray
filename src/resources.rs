use std::ops::Index;

use crate::{material::Material, texture::Texture};

#[derive(Debug, Default)]
/// Resources that can be used a scene.
pub struct Resources {
    /// A list of materials that can be assigned to objects in the scene.
    pub materials: Vec<Box<dyn Material>>,
    /// A list of textures that can be used by materials in the scene.
    pub textures: Vec<Box<dyn Texture>>,
}

impl Resources {
    /// Adds a material to the resources and returns its identifier.
    pub fn add_material<M: Material + 'static>(&mut self, material: M) -> MaterialId {
        let id = MaterialId(self.materials.len());
        self.materials.push(Box::new(material));
        id
    }

    /// Adds a texture to the resources and returns its identifier.
    pub fn add_texture<T: Texture + 'static>(&mut self, texture: T) -> TextureId {
        let id = TextureId(self.textures.len());
        self.textures.push(Box::new(texture));
        id
    }
}

#[derive(Debug, Clone, Copy)]
/// An identifier for a material.
pub struct MaterialId(usize);

impl Index<MaterialId> for Resources {
    type Output = dyn Material;

    fn index(&self, index: MaterialId) -> &Self::Output {
        &*self.materials[index.0]
    }
}

#[derive(Debug, Clone, Copy)]
/// An identifier for a texture.
pub struct TextureId(usize);

impl Index<TextureId> for Resources {
    type Output = dyn Texture;

    fn index(&self, index: TextureId) -> &Self::Output {
        &*self.textures[index.0]
    }
}
