use std::fmt::Debug;

use crate::{
    aabb::Aabb,
    interval::Interval,
    ray::{Intersection, Ray},
};

/// A trait for objects that can be hit by a ray.
pub trait Hittable: Debug + Send + Sync {
    fn hit(&self, r: &Ray, time: Interval) -> Option<Intersection>;

    fn bounding_box(&self) -> Aabb;
}
