use std::ops::Index;

use crate::{
    hittable::Hittable,
    interval::Interval,
    ray::{Intersection, Ray},
    vec3,
    vector::{Color, Vec3},
};

#[derive(Debug, Clone)]
/// An ID for an object in a scene.
pub struct ObjectId(usize);

#[derive(Debug)]
/// A scene containing objects to be rendered.
pub struct Scene {
    /// The objects in the scene.
    objects: Vec<Box<dyn Hittable>>,
    /// The background color of the scene.
    background: Color,
    // /// The hierarchy of bounding volumes for the scene.
    // bvh: Option<Bvh>,
}

impl Scene {
    /// Creates a new scene with the given background color.
    pub fn new(background: Color) -> Self {
        Self {
            objects: Vec::new(),
            background,
            // bvh: None,
        }
    }

    /// Builds the bounding volume hierarchy for the scene.
    pub fn build_bvh(&mut self) {
        todo!()
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
        let unit_dir = dir.unit();
        let a = 0.5 * (unit_dir.y + 1.0);

        (1.0 - a) * vec3!(1, 1, 1) + a * self.background
    }
}

impl Hittable for Scene {
    /// Checks for intersections between the ray and the objects in the scene.
    fn hit(&self, ray: &Ray, mut time: Interval) -> Option<Intersection> {
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
