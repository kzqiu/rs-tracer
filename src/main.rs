#![allow(dead_code, unused_variables)]

mod color;
mod ray;
mod vec3;

use crate::color::write_color;
use crate::ray::Ray;
use crate::vec3::Vec3;

fn ray_color(r: &Ray) -> Vec3 {
    let unit_dir = vec3::unit_vector(r.dir);

    let t = 0.5 * (unit_dir.y() + 1.);
    (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.)
}

fn main() {
    // Image constant vals
    const ASPECT_RATIO: f64 = 16. / 9.;
    const WIDTH: u64 = 400;
    const HEIGHT: u64 = (WIDTH as f64 / ASPECT_RATIO) as u64;

    let viewport_height: f64 = 2.;
    let viewport_width: f64 = ASPECT_RATIO * viewport_height;
    let focal_len: f64 = 1.;

    let origin = Vec3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_width, 0.);
    let lower_left = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_len);

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

            write_color(ray_color(&r));
        }
    }

    eprintln!("Done!");
}
