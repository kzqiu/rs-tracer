use rand::Rng;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}

impl Vec3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self {
            e0: a,
            e1: b,
            e2: c,
        }
    }

    pub fn random() -> Self {
        Self {
            e0: rand::thread_rng().gen(),
            e1: rand::thread_rng().gen(),
            e2: rand::thread_rng().gen(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self {
            e0: rand::thread_rng().gen_range(min..max),
            e1: rand::thread_rng().gen_range(min..max),
            e2: rand::thread_rng().gen_range(min..max),
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

    pub fn random_unit() -> Self {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn x(&self) -> f64 {
        self.e0
    }

    pub fn y(&self) -> f64 {
        self.e1
    }

    pub fn z(&self) -> f64 {
        self.e2
    }

    pub fn len_2(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn len(&self) -> f64 {
        self.len_2().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s: f64 = 0.000000001;

        self.e0.abs() < s && self.e1.abs() < s && self.e2.abs() < s
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e0, self.e1, self.e2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            e0: -self.e0,
            e1: -self.e1,
            e2: -self.e2,
        }
    }
}

impl ops::Index<u32> for Vec3 {
    type Output = f64;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e2,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl ops::IndexMut<u32> for Vec3 {
    fn index_mut(&mut self, index: u32) -> &mut f64 {
        match index {
            0 => &mut self.e0,
            1 => &mut self.e1,
            2 => &mut self.e2,
            _ => panic!("Index out of bounds!"),
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.e0 = self.e0 * other;
        self.e1 = self.e1 * other;
        self.e2 = self.e2 * other;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.e0 = self.e0 / other;
        self.e1 = self.e1 / other;
        self.e2 = self.e2 / other;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e0: self * other.e0,
            e1: self * other.e1,
            e2: self * other.e2,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            e0: self.e0 / other,
            e1: self.e1 / other,
            e2: self.e2 / other,
        }
    }
}

// #[inline(always)]
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e0 * v.e0 + u.e1 * v.e1 + u.e2 * v.e2
}

#[inline(always)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e0: u.e1 * v.e2 - u.e2 * v.e1,
        e1: u.e2 * v.e0 - u.e0 * v.e2,
        e2: u.e0 * v.e1 - u.e1 * v.e0,
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
