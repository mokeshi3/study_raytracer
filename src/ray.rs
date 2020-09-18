use crate::vec3::{Point3, Vec3};

#[allow(dead_code)]
pub struct Ray {
    orig: Point3,
    dir: Vec3<f64>,
}

#[allow(dead_code)]
impl Ray {
    fn new() -> Self {
        Self {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }

    fn build(orig: Point3, dir: Vec3<f64>) -> Self {
        Self {
            orig,
            dir,
        }
    }

    fn at(&self, t: f64) -> Vec3<f64> {
        self.orig + self.dir*t
    }
}
