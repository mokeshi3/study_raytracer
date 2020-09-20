use crate::hit::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point3, dot};

struct Sphere {
    center: Point3,
    radius: f64,
}

#[allow(dead_code)]
impl Sphere {
    fn new() -> Self {
        Self {
            center: Point3::new(),
            radius: 0.,
        }
    }

    fn build(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, tmin: f64, tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = *r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0. {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < tmax && temp > tmin {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return true;
            }

            let temp = (-half_b + root) / a;
            if temp < tmax && temp > tmin {
                rec.t = temp;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                return true;
            }
        }

        false
    }
}
