use crate::{
    material::Material,
    ray::{Intersection, Ray},
    resources::{Resources, TextureId},
    vector::{Color, Vec3},
};

#[derive(Debug)]
/// A Lambertian material, which scatters rays in random directions.
pub struct LambertianMaterial {
    /// The texture of the material's albedo.
    albedo: TextureId,
    /// The normal map of the material.
    normal_map: Option<TextureId>,
}

impl LambertianMaterial {
    /// Create a new Lambertian material with the given albedo texture.
    pub fn new(albedo: TextureId) -> Self {
        Self {
            albedo,
            normal_map: None,
        }
    }

    /// Create a new Lambertian material with the given albedo and normal map textures.
    pub fn with_normal_map(albedo: TextureId, normal_map: TextureId) -> Self {
        Self {
            albedo,
            normal_map: Some(normal_map),
        }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        resources: &Resources,
        _ray: &Ray,
        hit: &Intersection,
    ) -> Option<(Ray, Color)> {
        let mut normal = hit.normal;

        // Apply the normal map if one is present.
        if let Some(normal_map) = self.normal_map {
            // Convert from range [0, 1] to range [-1, 1].
            let result = resources[normal_map].color(resources, hit.u, hit.v) * 2.0 - Vec3::ONE;

            // Convert the normal from normal space to world space.
            normal = Vec3::convert_to_world_space(hit.normal, result);
        }

        let mut scatter_dir = normal + Vec3::random_in_unit_sphere().unit();

        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }

        let albedo = resources[self.albedo].color(resources, hit.u, hit.v);

        let scattered_ray = Ray::new(hit.point, scatter_dir);

        Some((scattered_ray, albedo))
    }
}
