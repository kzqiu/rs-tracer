#![allow(dead_code)]

mod color;
mod ray;
mod vec3;

use crate::vec3::Vec3;

fn main() {
    const WIDTH: u64 = 256;
    const HEIGHT: u64 = 256;

    // PPM Image Header
    println!("P3\n{WIDTH} {HEIGHT}\n255");

    // Rendering Image

    for j in (0..HEIGHT).rev() {
        if j % 25 == 0 {
            eprintln!("Scanlines remaining: {j}");
        }

        for i in 0..WIDTH {
            let c = Vec3 {
                e0: (i as f64) / (WIDTH as f64 - 1.),
                e1: (j as f64) / (HEIGHT as f64 - 1.),
                e2: 0.25,
            };

            crate::color::write_color(c);
        }
    }

    eprintln!("Done!");
}
