use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub e0: f64,
    pub e1: f64,
    pub e2: f64,
}

#[allow(dead_code)]
impl Vec3 {
    pub fn new() -> Self {
        Self {
            e0: 0.,
            e1: 0.,
            e2: 0.,
        }
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

#[inline(always)]
fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e0 * v.e0 + u.e1 * v.e1 + u.e2 * v.e2
}

#[inline(always)]
fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e0: u.e1 * v.e2 - u.e2 * v.e1,
        e1: u.e2 * v.e0 - u.e0 * v.e2,
        e2: u.e0 * v.e1 - u.e1 * v.e0,
    }
}

#[inline(always)]
fn unit_vector(v: Vec3) -> Vec3 {
    v / v.len()
}
