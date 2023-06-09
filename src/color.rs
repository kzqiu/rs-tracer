use crate::vec3::Vec3;

pub fn write_color(c: Vec3) {
    println!(
        "{} {} {}",
        (255.999 * c.x()) as i32,
        (255.999 * c.y()) as i32,
        (255.999 * c.z()) as i32
    );
}
