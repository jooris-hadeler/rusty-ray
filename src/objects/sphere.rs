use std::f64::consts::PI;

use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::{Intersection, Ray},
    resources::MaterialId,
    vector::Point3,
};

#[derive(Debug)]
/// A sphere object in 3d space.
pub struct SphereObject {
    /// The center of the sphere.
    center: Point3,
    /// The radius of the sphere.
    radius: f64,
    /// The material of the sphere.
    material: MaterialId,
}

impl SphereObject {
    /// Create a new sphere object with the given center, radius, and material.
    pub fn new(center: Point3, radius: f64, material: MaterialId) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    /// Get the UV coordinates of a point on the sphere.
    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for SphereObject {
    fn hit(&self, r: &Ray, time: Interval) -> Option<Intersection> {
        let oc = self.center - r.orig;
        let a = r.dir.len_sq();
        let h = oc.dot(r.dir);
        let c = oc.len_sq() - self.radius * self.radius;

        // calculate the discriminant
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // find the nearest root that lies in the acceptable range
        let sqrt_d = discriminant.sqrt();

        let mut t = (h - sqrt_d) / a;
        if t <= time.start || time.end <= t {
            t = (h + sqrt_d) / a;
            if t <= time.start || time.end <= t {
                return None;
            }
        }

        // record the intersection
        let point = r.at(t);
        let outward_normal = (point - self.center) / self.radius;

        let material = self.material;
        let (u, v) = SphereObject::get_sphere_uv(outward_normal);
        let (front_face, normal) = Intersection::face_normal(r, outward_normal);

        Some(Intersection {
            point,
            normal,
            front_face,
            material,
            t,
            u,
            v,
        })
    }
}
