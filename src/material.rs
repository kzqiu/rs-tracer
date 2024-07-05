use crate::vec3::{dot, reflect, refract, unit_vector, Vec3};
use crate::{hittable::HitRecord, ray::Ray};
use rand::Rng;

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
    Dielectric(Dielectric),
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
            MatType::Dielectric(d) => d,
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
        _r_in: &Ray,
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

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub index_refraction: f64,
}

impl Dielectric {
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Schlick's approximation for reflectance
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        let r1 = r0 * r0;
        r1 + (1. - r1) * f64::powf(1. - cosine, 5.)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Vec3::new(1., 1., 1.);
        let refraction_ratio = if rec.front {
            1. / self.index_refraction
        } else {
            self.index_refraction
        };

        let unit_dir = unit_vector(r_in.direction());
        let cos_theta = dot(-unit_dir, rec.norm).min(1.);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let dir;

        if refraction_ratio * sin_theta > 1.
            || Dielectric::reflectance(cos_theta, refraction_ratio)
                > rand::thread_rng().gen::<f64>()
        {
            dir = reflect(unit_dir, rec.norm);
        } else {
            dir = refract(unit_dir, rec.norm, refraction_ratio);
        }

        *scattered = Ray { orig: rec.p, dir };

        true
    }
}
