#![allow(dead_code, unused_variables)]

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{unit_vector, Vec3};
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use std::ops::DerefMut;
use std::rc::Rc;

use image::{Rgb, RgbImage};

// Utility constants and functions
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

    let mut img = RgbImage::new(WIDTH, HEIGHT);

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new();

    let stride = WIDTH as usize * 3;

    img.deref_mut()
        .par_chunks_exact_mut(stride)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.chunks_exact_mut(3).enumerate() {
                let u: f64 = (x as f64) / (WIDTH as f64 - 1.);
                let v: f64 = ((HEIGHT - y as u32) as f64) / (HEIGHT as f64 - 1.);
                let r = camera.get_ray(u, v);

                if x >= WIDTH as usize || y >= HEIGHT as usize {
                    continue;
                }

                let color = ray_color(&r, &world);
                let Rgb(a) = Rgb([
                    (255.99 * color.x()) as u8,
                    (255.99 * color.y()) as u8,
                    (255.99 * color.z()) as u8,
                ]);

                pixel.copy_from_slice(&a);
            }
        });

    match img.save("image.png") {
        Err(e) => eprintln!("Error writing file: {}", e),
        Ok(_) => println!("Done."),
    };
}
