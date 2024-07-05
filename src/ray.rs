use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
