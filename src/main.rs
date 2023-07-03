#![allow(dead_code, unused_variables)]

mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

// use crate::color::write_color;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Vec3};
use std::rc::Rc;

use image::{ImageBuffer, Rgb, RgbImage};

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
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = (WIDTH as f64 / ASPECT_RATIO) as u32;

    let mut buffer: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

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

    // Rendering Image
    for (i, j, pixel) in buffer.enumerate_pixels_mut() {
        let u: f64 = (i as f64) / (WIDTH as f64 - 1.);
        let v: f64 = ((HEIGHT - j) as f64) / (HEIGHT as f64 - 1.);
        let r = Ray {
            orig: origin,
            dir: lower_left + u * horizontal + v * vertical - origin,
        };

        let color = ray_color(&r, &world);
        *pixel = Rgb([
            (255.99 * color.x()) as u8,
            (255.99 * color.y()) as u8,
            (255.99 * color.z()) as u8,
        ]);
    }

    match buffer.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done."),
    };
}
