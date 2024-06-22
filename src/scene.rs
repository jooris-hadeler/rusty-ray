use std::ops::Index;

use crate::{
    bvh::Bvh,
    hittable::Hittable,
    interval::Interval,
    ray::{Intersection, Ray},
    vector::{Color, Vec3},
};

#[derive(Debug, Clone, Copy)]
/// An ID for an object in a scene.
pub struct ObjectId(usize);

/// A scene containing objects to be rendered.
pub struct Scene {
    /// The objects in the scene.
    objects: Vec<Box<dyn Hittable>>,
    /// The function to calculate the background color of the scene.
    background_func: Box<dyn Fn(Vec3) -> Color>,
    // /// The hierarchy of bounding volumes for the scene.
    bvh: Option<Bvh>,
}

impl Scene {
    /// Creates a new scene with the given background color.
    pub fn new<F: Fn(Vec3) -> Color + 'static>(background: F) -> Self {
        Self {
            objects: Vec::new(),
            background_func: Box::new(background),
            bvh: None,
        }
    }

    /// Builds the bounding volume hierarchy for the scene.
    pub fn build_bvh(&mut self) {
        // Collect bounding boxes for all objects
        let objects_with_bbs = self
            .objects
            .iter()
            .enumerate()
            .map(|(id, object)| (ObjectId(id), object.bounding_box()))
            .collect();

        // Construct the BVH from the bounding boxes
        self.bvh = Some(Bvh::new(objects_with_bbs));
    }

    /// Adds an object to the scene.
    pub fn add<H: Hittable + 'static>(&mut self, object: H) -> ObjectId {
        let id = ObjectId(self.objects.len());
        self.objects.push(Box::new(object));
        id
    }

    #[inline]
    /// Get the background color of the scene.
    pub fn background(&self, dir: Vec3) -> Color {
        (self.background_func)(dir)
    }

    /// Checks for intersections between the ray and the objects in the scene.
    pub fn hit(&self, ray: &Ray, time: Interval) -> Option<Intersection> {
        if self.bvh.is_some() {
            self.hit_fast(ray, time)
        } else {
            self.hit_slow(ray, time)
        }
    }

    /// Checks for intersections between the ray and the objects in the scene using the BVH.
    fn hit_fast(&self, ray: &Ray, mut time: Interval) -> Option<Intersection> {
        // Get the objects that could be hit by the ray.
        let objects_to_check = self.bvh.as_ref().unwrap().hit(ray, time)?;

        let mut closest = None;

        // Check each possible object for intersections.
        for object_id in objects_to_check {
            if let Some(intersection) = self[object_id].hit(ray, time) {
                // Update the closest intersection.
                time.end = intersection.t;
                closest = Some(intersection);
            }
        }

        closest
    }

    /// Checks for every object in the scene if the ray intersects with it.
    fn hit_slow(&self, ray: &Ray, mut time: Interval) -> Option<Intersection> {
        let mut closest = None;

        // Check each object in the scene for intersections.
        for object in self.objects.iter() {
            if let Some(intersection) = object.hit(ray, time) {
                // Update the closest intersection.
                time.end = intersection.t;
                closest = Some(intersection);
            }
        }

        closest
    }
}

impl Index<ObjectId> for Scene {
    type Output = Box<dyn Hittable>;

    fn index(&self, id: ObjectId) -> &Self::Output {
        &self.objects[id.0]
    }
}
