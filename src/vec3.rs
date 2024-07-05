use rand::Rng;
use std::ops;

#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { x: a, y: b, z: c }
    }

    pub fn random() -> Self {
        Self {
            x: rand::thread_rng().gen(),
            y: rand::thread_rng().gen(),
            z: rand::thread_rng().gen(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
        }
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_range(-1., 1.);
            if dot(p, p) >= 1. {
                continue;
            }

            return p;
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(
                rand::thread_rng().gen_range(-1. ..1.),
                rand::thread_rng().gen_range(-1. ..1.),
                0.,
            );
            if p.len_2() >= 1. {
                continue;
            }
            return p;
        }
    }

    pub fn random_unit() -> Self {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn len_2(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn len(&self) -> f64 {
        self.len_2().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 0.000000001;

        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Index<u32> for Vec3 {
    type Output = f64;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl ops::IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, index: u32) -> &mut f64 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

// #[inline(always)]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

#[inline(always)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    }
}

#[inline(always)]
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * dot(v, n) * n
}

pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.);
    let out_perp = etai_over_etat * (uv + cos_theta * n);
    let out_parallel = -((1.0 - out_perp.len_2()).abs().sqrt()) * n;
    out_perp + out_parallel
}
