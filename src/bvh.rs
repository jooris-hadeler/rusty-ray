use crate::{
    interval::Interval,
    ray::Ray,
    scene::{ObjectId, Scene},
};

#[derive(Debug)]
/// A bounding volume hierarchy for a scene.
pub struct Bvh;

impl Bvh {
    /// Checks for intersections between the ray and the objects in the scene.
    /// Returns a list of object IDs that were hit by the ray.
    pub fn hit(&self, _scene: &Scene, _ray: &Ray, _time: Interval) -> Option<Vec<ObjectId>> {
        todo!()
    }
}
