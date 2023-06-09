mod vec3;
use crate::vec3::Vec3;

fn main() {
    // const WIDTH: u64 = 256;
    // const HEIGHT: u64 = 256;

    // // PPM Image Header
    // println!("P3\n{WIDTH} {HEIGHT}\n255");

    // // Rendering Image

    // for j in (0..HEIGHT).rev() {
    //     eprintln!("Scanlines remaining: {j}");
    //     for i in 0..WIDTH {
    //         let r: f32 = i as f32 / (WIDTH as f32 - 1.);
    //         let g: f32 = j as f32 / (HEIGHT as f32 - 1.);
    //         let b: f32 = 0.25;

    //         let ir = (255.999 * r) as i32;
    //         let ig = (255.999 * g) as i32;
    //         let ib = (255.999 * b) as i32;

    //         println!("{ir} {ig} {ib}");
    //     }
    // }

    // eprintln!("Done!");

    let a = Vec3 {
        e0: 1.,
        e1: 2.,
        e2: 3.,
    };

    let b = Vec3 {
        e0: 4.,
        e1: 5.,
        e2: 6.,
    };

    println!("{}", (a ^ b));
    println!("{}", a.len_2());
}
