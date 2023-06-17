#![allow(dead_code, unused_variables)]

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Vec3};
use std::rc::Rc;

const PI: f64 = 3.1415926535897932385;
const INF: f64 = f64::INFINITY;

fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.;
}

fn ray_color(r: &Ray, world: &impl Hittable) -> Vec3 {
    let mut rec = HitRecord::new();

    if world.hit(r, 0., INF, &mut rec) {
        return 0.5 * (rec.norm + Vec3::new(1., 1., 1.));
    }

    let unit_dir = unit_vector(r.dir);
    let t = 0.5 * (unit_dir.y() + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u64 = 400;
    const HEIGHT: u64 = (WIDTH as f64 / ASPECT_RATIO) as u64;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    // Camera
    let viewport_height: f64 = 2.;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_len: f64 = 1.;
    let origin = Vec3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_len);
    // eprintln!("{}", lower_left);

    // PPM Image Header
    println!("P3\n{WIDTH} {HEIGHT}\n255");

    // Rendering Image
    for j in (0..HEIGHT).rev() {
        if j % 25 == 0 {
            eprintln!("Scanlines remaining: {j}");
        }

        for i in 0..WIDTH {
            let u: f64 = (i as f64) / (WIDTH as f64 - 1.);
            let v: f64 = (j as f64) / (HEIGHT as f64 - 1.);
            let r = Ray {
                orig: origin,
                dir: lower_left + u * horizontal + v * vertical - origin,
            };

            // let c = Vec3 {
            //     e0: (i as f64) / (WIDTH as f64 - 1.),
            //     e1: (j as f64) / (HEIGHT as f64 - 1.),
            //     e2: 0.25,
            // };

            write_color(ray_color(&r, &world));
        }
    }

    eprintln!("Done!");
}
