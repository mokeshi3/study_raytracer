use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};

#[allow(dead_code)]
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3<f64>,
    pub t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point3::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: true,
        }
    }
}

#[allow(dead_code)]
impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3<f64>) {
        let front_face = dot(r.direction(), &outward_normal) < 0.;
        self.normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
