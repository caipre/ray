use crate::ray::Ray;
use crate::vec3::vec3;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub t: f64,
    pub front: bool,
    pub point: vec3,
    pub normal: vec3,
}

impl Hit {
    pub fn new(r: &Ray, t: f64, outward: &vec3) -> Hit {
        let mut hit = Hit { t, front: false, point: r.at(t), normal: *outward };
        hit.set_normal(r, outward);
        hit
    }

    fn set_normal(&mut self, r: &Ray, outward: &vec3) -> &mut Self {
        self.front = r.orient.dot(outward) < 0.0;
        self.normal = if self.front { *outward } else { -*outward };
        self
    }
}

impl<T> Hittable for Vec<T> where T: Hittable + Sized {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut closest_t = t_max;
        for ob in self.iter() {
            hit = ob.hit(r, t_min, closest_t);
            if let Some(h) = hit {
                closest_t = h.t;
            }
        }
        hit
    }
}
