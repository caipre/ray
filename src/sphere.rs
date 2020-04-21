use crate::hittable::{Hit, Hittable};
use crate::vec3::vec3;
use core::fmt::Alignment::Center;
use crate::ray::Ray;

pub struct Sphere {
    pub center: vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = r.orient.dot(&r.orient);
        let half_b = oc.dot(&r.orient);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let root_neg = (-half_b - root) / a;
            if root_neg < t_max && root_neg > t_min {
                let outward = (r.at(root_neg) - self.center) / self.radius;
                let hit = Hit::new(r, root_neg, &outward);
                return Some(hit);
            }
            let root_pos = (-half_b + root) / a;
            if root_pos < t_max && root_pos > t_min {
                let outward = (r.at(root_pos) - self.center) / self.radius;
                let hit = Hit::new(r, root_pos, &outward);
                return Some(hit);
            }
        }
        None
    }
}
