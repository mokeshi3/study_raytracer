use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

struct HitRecord {
    p: Point3,
    normal: Vec3<f64>,
    t: f64,
}

trait hittable {
    fn hit(r: &Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}
