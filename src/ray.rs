use crate::vec3::{Point3, Vec3};

#[allow(dead_code)]
pub struct Ray {
    orig: Point3,
    dir: Vec3<f64>,
}

#[allow(dead_code)]
impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }

    pub fn build(orig: Point3, dir: Vec3<f64>) -> Self {
        Self {
            orig,
            dir,
        }
    }

    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.orig + self.dir*t
    }

    pub fn direction(&self) -> &Vec3<f64> {
        &self.dir
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }
}
