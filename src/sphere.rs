use crate::hittable::{HitRecord, Hittable};
use crate::material::MatType;
use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub rad: f64,
    pub material: MatType,
    pub velocity: Vec3,
    pub moving: bool,
}

impl Sphere {
    pub fn new(center: Vec3, rad: f64, material: MatType) -> Self {
        Sphere {
            center,
            rad,
            material,
            velocity: Vec3::default(),
            moving: false,
        }
    }

    pub fn new_moving(center: Vec3, target: Vec3, rad: f64, material: MatType) -> Self {
        Sphere {
            center,
            rad,
            material,
            velocity: target - center,
            moving: true,
        }
    }

    fn sphere_center(&self, time: f64) -> Vec3 {
        self.center + time * self.velocity
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let center = if self.moving {
            self.sphere_center(r.time)
        } else {
            self.center
        };
        let oc: Vec3 = r.orig - center;
        let a = r.dir.len_2();
        let half_b = dot(oc, r.dir);
        let c = oc.len_2() - self.rad * self.rad;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return false;
        }

        // Find the nearest root that lies in the acceptable range.
        let sqrt_disc = discriminant.sqrt();

        let mut root = (-half_b - sqrt_disc) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrt_disc) / a;

            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        rec.material = self.material;
        let outward_norm: Vec3 = (rec.p - self.center) / self.rad;
        rec.set_face_normal(r, outward_norm);

        true
    }
}
