use crate::vec3::vec3;
use std::ops::Mul;
use crate::hittable::Hittable;

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

    pub fn color<T: Hittable>(&self, world: &Vec<T>) -> vec3 {
        if let Some(hit) = world.hit(self, 0.0, std::f64::INFINITY) {
            return 0.5 * (hit.normal + 1);
        }
        let unit = &self.orient.to_unit();
        let t = 0.5 * (unit.y + 1.0);
        (1.0 - t) * vec3::newi(1, 1, 1) + t * vec3::new(0.5, 0.7, 1.0)
    }
}
