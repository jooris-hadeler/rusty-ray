use std::ops::Index;

use crate::{interval::Interval, intr, ray::Ray, vector::Point3};

#[derive(Debug, Clone, Copy)]
/// An axis-aligned bounding box.
pub struct Aabb {
    /// The interval of the bounding box in the x direction.
    pub x: Interval,
    /// The interval of the bounding box in the y direction.
    pub y: Interval,
    /// The interval of the bounding box in the z direction.
    pub z: Interval,
}

impl Aabb {
    /// Empty bounding box.
    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    /// Creates a new axis-aligned bounding box from two points.
    pub fn new(min: Point3, max: Point3) -> Self {
        Self {
            x: intr!(min.x, max.x),
            y: intr!(min.y, max.y),
            z: intr!(min.z, max.z),
        }
    }

    /// Grows the bounding box to include another bounding box.
    pub fn grow(&mut self, other: &Aabb) {
        self.x.start = self.x.start.min(other.x.start);
        self.x.end = self.x.end.max(other.x.end);

        self.y.start = self.y.start.min(other.y.start);
        self.y.end = self.y.end.max(other.y.end);

        self.z.start = self.z.start.min(other.z.start);
        self.z.end = self.z.end.max(other.z.end);
    }

    /// Returns the axis with the largest extent.
    pub fn largest_axis(&self) -> usize {
        let x_extent = self.x.end - self.x.start;
        let y_extent = self.y.end - self.y.start;
        let z_extent = self.z.end - self.z.start;

        if x_extent > y_extent {
            if x_extent > z_extent {
                0
            } else {
                2
            }
        } else if y_extent > z_extent {
            1
        } else {
            2
        }
    }

    /// Returns the component of the bounding box for the given axis.
    pub fn component(&self, axis: usize) -> Interval {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid axis"),
        }
    }

    /// Checks if the bounding box intersects with a ray.
    pub fn hit(&self, ray: &Ray, mut time: Interval) -> bool {
        for idx in 0..3 {
            let axis = self[idx];
            let inv_d = 1.0 / ray.dir[idx];

            let t0 = (axis.start - ray.orig[idx]) * inv_d;
            let t1 = (axis.end - ray.orig[idx]) * inv_d;

            let (t0, t1) = if t1 < t0 { (t1, t0) } else { (t0, t1) };

            if t0 > time.start {
                time.start = t0;
            }

            if t1 < time.end {
                time.end = t1;
            }

            if time.end <= time.start {
                return false;
            }
        }

        true
    }
}

impl Index<usize> for Aabb {
    type Output = Interval;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid axis"),
        }
    }
}
