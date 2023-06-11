use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vec3) {
        self.front = dot(r.direction(), outward_norm) < 0.;
        self.norm = if self.front {
            outward_norm
        } else {
            -outward_norm
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
