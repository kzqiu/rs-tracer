use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for obj in self.objects.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                rec.t = tmp_rec.t;
                rec.norm = tmp_rec.norm;
                rec.p = tmp_rec.p;
                rec.front = tmp_rec.front;
            }
        }

        hit_anything
    }
}
