use crate::vec3::Vec3;
use image::Rgb;

pub fn write_color(pixel: Rgb<u8>, c: Vec3) {
    println!(
        "{} {} {}",
        (255.999 * c.x()) as i32,
        (255.999 * c.y()) as i32,
        (255.999 * c.z()) as i32
    );
}
