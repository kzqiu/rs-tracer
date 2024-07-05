use crate::material::MatType;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct HitRecord {
    pub p: Vec3,
    pub norm: Vec3,
    pub t: f64,
    pub front: bool,
    pub material: MatType,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            p: Vec3::new(0., 0., 0.),
            norm: Vec3::new(0., 0., 0.),
            t: 0.,
            front: false,
            material: MatType::Lambertian(crate::material::Lambertian {
                albedo: Vec3::new(1., 1., 1.),
            }),
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_norm: Vec3) {
        self.front = dot(r.dir, outward_norm) < 0.;
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
