use std::fmt::{Display, Formatter};
use std::io::{BufWriter, Stdout, Write};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::{fmt, io};

/// The `vec3` struct represents a vector of three dimensions.
#[derive(Debug, Copy, Clone)]
pub struct vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl vec3 {
    pub const ORIGIN: vec3 = vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    /// Constructs a vector composed of the given values.
    pub fn newi(x: isize, y: isize, z: isize) -> Self {
        vec3 {
            x: x as f64,
            y: y as f64,
            z: z as f64,
        }
    }

    /// Constructs a vector composed of the given values.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        vec3 { x, y, z }
    }

    /// Calculates the length of the vector using the Euclidean norm.
    pub fn len(&self) -> f64 {
        self.dot(self).sqrt()
    }

    /// Calculates the dot-product of this vector with another vector.
    pub fn dot(&self, rhs: &vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Calculates the cross-product of this vector with another vector.
    pub fn cross(&self, rhs: &vec3) -> Self {
        vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    /// Returns this vector scaled to a unit vector.
    pub fn to_unit(&self) -> vec3 {
        *self / self.len()
    }
}

impl Default for vec3 {
    fn default() -> Self {
        vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Neg for vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<isize> for vec3 {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        vec3 {
            x: self.x + rhs as f64,
            y: self.y + rhs as f64,
            z: self.z + rhs as f64,
        }
    }
}

impl Add<f64> for vec3 {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl AddAssign for vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<f64> for vec3 {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Sub for vec3 {
    type Output = vec3;

    fn sub(self, rhs: vec3) -> Self::Output {
        vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<vec3> for f64 {
    type Output = vec3;

    fn mul(self, rhs: vec3) -> Self::Output {
        vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

pub trait WriteColor<W>
where
    W: Write,
{
    fn write(&self, w: &mut W) -> io::Result<()>;
}

impl WriteColor<Stdout> for vec3 {
    fn write(&self, w: &mut Stdout) -> io::Result<()> {
        let colorvec = *self * 255.99;
        writeln!(
            w,
            "{} {} {}",
            colorvec.x as u64, colorvec.y as u64, colorvec.z as u64
        )
    }
}
