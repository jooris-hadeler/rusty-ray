use crate::{
    material::Material,
    ray::{Intersection, Ray},
    resources::Resources,
    vector::Color,
};

#[derive(Debug)]
/// A dielectric material, which refracts light through the object.
pub struct DielectricMaterial {
    /// The refractive index of the material.
    pub refraction_index: f64,
}

impl DielectricMaterial {
    /// Constructs a new dielectric material with the given refractive index.
    pub const fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for DielectricMaterial {
    fn scatter(
        &self,
        _resources: &Resources,
        ray: &Ray,
        hit: &Intersection,
    ) -> Option<(Ray, Color)> {
        let ri = if hit.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.dir.unit();
        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract {
            unit_direction.reflect(hit.normal)
        } else {
            unit_direction.refract(hit.normal, ri)
        };

        let scattered_ray = Ray::new(hit.point, direction);

        Some((scattered_ray, Color::WHITE))
    }
}
