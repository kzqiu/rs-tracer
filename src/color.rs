use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{unit_vector, Vec3};

pub fn ray_color(r: &Ray, world: &impl Hittable, depth: u32) -> Vec3 {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    if world.hit(r, 0.001, crate::INF, &mut rec) {
        let mut scattered = Ray {
            orig: Vec3::new(0., 0., 0.),
            dir: Vec3::new(0., 0., 0.),
        };
        let mut attenuation = Vec3::new(0., 0., 0.);

        if rec
            .material
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Vec3::new(0., 0., 0.);
    }

    let unit_dir = unit_vector(r.dir);
    let t = 0.5 * (unit_dir.y + 1.);
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
