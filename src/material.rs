use crate::vec3::{dot, reflect, unit_vector, Vec3};
use crate::{hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy)]
pub enum MatType {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl MatType {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        self.as_trait().scatter(r_in, rec, attenuation, scattered)
    }

    fn as_trait(&self) -> &dyn Material {
        match self {
            MatType::Metal(m) => m,
            MatType::Lambertian(l) => l,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_dir = rec.norm + Vec3::random_unit();
        if scatter_dir.near_zero() {
            scatter_dir = rec.norm;
        }
        *scattered = Ray {
            orig: rec.p,
            dir: scatter_dir,
        };
        *attenuation = self.albedo;

        true
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected: Vec3 = reflect(unit_vector(r_in.direction()), rec.norm);
        *scattered = Ray {
            orig: rec.p,
            dir: reflected + self.fuzz * Vec3::random_unit(),
        };
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.norm) > 0.
    }
}
