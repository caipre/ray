use crate::vec3::vec3;
use std::ops::Mul;

pub struct Ray {
    pub origin: vec3,
    pub orient: vec3,
}

impl Ray {
    pub fn new(origin: vec3, orient: vec3) -> Ray {
        Ray { origin, orient }
    }

    pub fn at(&self, t: f64) -> vec3 {
        self.origin + t * self.orient
    }
}
