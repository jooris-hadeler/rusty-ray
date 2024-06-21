use crate::{
    material::Material,
    ray::{Intersection, Ray},
    resources::Resources,
    vector::{Color, Vec3},
};

#[derive(Debug)]
pub struct MetalMaterial {
    /// The albedo of the material.
    albedo: Color,
    /// The fuzziness of the material.
    fuzz: f64,
}

impl MetalMaterial {
    /// Creates a new metal material with the given albedo and fuzziness.
    pub const fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(
        &self,
        _resources: &Resources,
        ray: &Ray,
        hit: &Intersection,
    ) -> Option<(Ray, Color)> {
        let mut reflected = ray.dir.reflect(hit.normal).unit();

        reflected += Vec3::random_in_unit_sphere() * self.fuzz;

        let ray = Ray::new(hit.point, reflected);

        Some((ray, self.albedo))
    }
}
