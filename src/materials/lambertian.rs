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
}

impl LambertianMaterial {
    /// Create a new Lambertian material with the given albedo texture.
    pub fn new(albedo: TextureId) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(
        &self,
        resources: &Resources,
        _ray: &Ray,
        hit: &Intersection,
    ) -> Option<(Ray, Color)> {
        let mut scatter_dir = hit.normal + Vec3::random_in_unit_sphere().unit();

        if scatter_dir.near_zero() {
            scatter_dir = hit.normal;
        }

        let albedo = resources[self.albedo].color(resources, hit.u, hit.v);

        let scattered_ray = Ray::new(hit.point, scatter_dir);

        Some((scattered_ray, albedo))
    }
}
