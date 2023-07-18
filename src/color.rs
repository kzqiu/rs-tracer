use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

pub fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0., crate::INF, &mut rec) {
        return 0.5 * (rec.norm + Vec3::new(1., 1., 1.));
    }

    let unit_dir = unit_vector(r.dir);
    let t = 0.5 * (unit_dir.y() + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    x
}
