use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::random::THREAD_RNG;

#[macro_export]
/// Create a new Vec3 with the given x, y, and z components.
macro_rules! vec3 {
    ($i:expr) => {{
        let i: f64 = $i.into();
        $crate::vector::Vec3 { x: i, y: i, z: i }
    }};
    ($x:expr, $y:expr, $z:expr) => {
        $crate::vector::Vec3 {
            x: $x.into(),
            y: $y.into(),
            z: $z.into(),
        }
    };
}

/// A color containing red, green, and blue components.
/// This is an alias for Vec3.
/// x is red, y is green, and z is blue.
pub type Color = Vec3;

impl Color {
    /// Constant color with all components set to 0.
    pub const WHITE: Color = Color {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
}

/// A point in 3D space, with x, y, and z components.
/// This is an alias for Vec3.
pub type Point3 = Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
/// A vector in 3D space, with x, y, and z components.
pub struct Vec3 {
    /// The x component of the vector.
    pub x: f64,
    /// The y component of the vector.
    pub y: f64,
    /// The z component of the vector.
    pub z: f64,
}

impl Vec3 {
    /// Constant vector with all components set to 0.
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Create a new Vec3 that lies inside the unit sphere.
    pub fn random_in_unit_sphere() -> Vec3 {
        THREAD_RNG.with(|rng| {
            let mut rng = rng.borrow_mut();

            loop {
                let p = vec3!(
                    rng.random_f64() * 2.0 - 1.0,
                    rng.random_f64() * 2.0 - 1.0,
                    rng.random_f64() * 2.0 - 1.0
                );

                if p.len_sq() < 1.0 {
                    return p;
                }
            }
        })
    }

    #[inline]
    /// Checks if the vector is near zero.
    pub fn near_zero(&self) -> bool {
        const DELTA: f64 = 1e-8;
        self.x.abs() < DELTA && self.y.abs() < DELTA && self.z.abs() < DELTA
    }

    #[inline]
    /// Calculates the length of the vector.
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }

    #[inline]
    /// Calculates the squared length of the vector.
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    /// Calculates the dot product of two vectors.
    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[inline]
    /// Calculates the cross product of two vectors.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    /// Returns a new vector with the same direction, but a length of 1.
    pub fn unit(&self) -> Vec3 {
        *self / self.len()
    }

    #[inline]
    /// Reflects the vector across a normal.
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - normal * self.dot(normal) * 2.0
    }

    #[inline]
    /// Refracts the vector through a surface with the given normal and refractive index.
    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.len_sq()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    #[inline]
    /// Adds two vectors together.
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    #[inline]
    /// Adds another vector to this one.
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    /// Negates the vector.
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    #[inline]
    /// Subtracts one vector from another.
    fn sub(self, other: Vec3) -> Vec3 {
        self + -other
    }
}

impl SubAssign for Vec3 {
    #[inline]
    /// Subtracts another vector from this one.
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other;
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    /// Multiplies a vector by a scalar.
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline]
    /// Multiplies a scalar by a vector.
    fn mul(self, vec: Vec3) -> Vec3 {
        vec * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    /// Multiplies two vectors together.
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    /// Multiplies this vector by a scalar.
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar;
    }
}

impl MulAssign<Vec3> for Vec3 {
    #[inline]
    /// Multiplies this vector by another vector.
    fn mul_assign(&mut self, other: Vec3) {
        *self = *self * other;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline]
    /// Divides a vector by a scalar.
    fn div(self, scalar: f64) -> Vec3 {
        let scalar = 1.0 / scalar;
        self * scalar
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    /// Divides this vector by a scalar.
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    #[inline]
    /// Indexes into the vector.
    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}
