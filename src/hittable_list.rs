use crate::hit::{Hittable, HitRecord};
use crate::ray::Ray;

struct HittableList<'a> {
    objects: Vec<&'a dyn Hittable>,
}

#[allow(dead_code)]
impl<'a> HittableList<'a> {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn add<T: Hittable>(&mut self, object: &'a T) {
        self.objects.push(object);
    }

    fn clear(&mut self) {
        self.objects.clear();
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = tmax;

        for object in self.objects.iter() {
            if object.hit(r, tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
