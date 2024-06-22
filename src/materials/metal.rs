use crate::{
    material::Material,
    ray::{Intersection, Ray},
    resources::{Resources, TextureId},
    vector::{Color, Vec3},
};

#[derive(Debug)]
pub struct MetalMaterial {
    /// The albedo of the material.
    albedo: Color,
    /// The fuzziness of the material.
    fuzz: f64,
    /// The normal map of the material.
    normal_map: Option<TextureId>,
}

impl MetalMaterial {
    /// Creates a new metal material with the given albedo and fuzziness.
    pub const fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz,
            normal_map: None,
        }
    }

    /// Creates a new metal material with the given albedo, fuzziness, and normal map.
    pub const fn with_normal_map(albedo: Color, fuzz: f64, normal_map: TextureId) -> Self {
        Self {
            albedo,
            fuzz,
            normal_map: Some(normal_map),
        }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        resources: &Resources,
        ray: &Ray,
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

        let mut reflected = ray.dir.reflect(normal).unit();

        reflected += Vec3::random_in_unit_sphere() * self.fuzz;

        let ray = Ray::new(hit.point, reflected);

        Some((ray, self.albedo))
    }
}
