use crate::{
    resources::MaterialId,
    vector::{Point3, Vec3},
};

#[derive(Debug, Clone)]
/// A ray in 3D space, with an origin and a direction.
pub struct Ray {
    /// The origin of the ray.
    pub orig: Point3,
    /// The direction of the ray.
    pub dir: Vec3,
}

impl Ray {
    /// Create a new ray with the given origin and direction.
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// Get the point along the ray at a given distance.
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

#[derive(Debug, Clone)]
/// A record of a hit by a ray.
pub struct Intersection {
    /// The point at which the ray hit the object.
    pub point: Point3,
    /// The normal vector of the object at the point of intersection.
    pub normal: Vec3,
    /// Whether the ray hit the object from the inside.
    pub front_face: bool,
    /// The material of the object that was hit.
    pub material: MaterialId,
    /// The distance from the ray's origin to the point of intersection.
    pub t: f64,
    /// The u texture coordinate of the hit.
    pub u: f64,
    /// The v texture coordinate of the hit.
    pub v: f64,
}

impl Intersection {
    /// Calculate the normal vector of the object at the point of intersection,
    /// and whether the ray hit the object from the inside.
    pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.dir.dot(outward_normal) < 0.0;

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        (front_face, normal)
    }
}
