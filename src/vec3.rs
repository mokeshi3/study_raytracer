use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec3<T: Copy> {
    e1: T,
    e2: T,
    e3: T,
}

#[allow(dead_code)]
pub type Point3 = Vec3<f64>;

#[allow(dead_code)]
pub type Color = Vec3<f64>;

#[allow(dead_code)]
impl<T: Copy + Into<f64> + ops::Mul<Output = T> + Default> Vec3<T> {
    pub fn new() -> Self {
        Self {
            e1: T::default(),
            e2: T::default(),
            e3: T::default(),
        }
    }

    pub fn build(e1: T, e2: T, e3: T) -> Self {
        Self { e1, e2, e3 }
    }

    pub fn x(&self) -> T {
        return self.e1;
    }

    pub fn y(&self) -> T {
        return self.e2;
    }

    pub fn z(&self) -> T {
        return self.e3;
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    pub fn length_squared(&self) -> f64 {
        let e1: f64 = self.e1.into();
        let e2: f64 = self.e2.into();
        let e3: f64 = self.e3.into();

        return e1 * e1 + e2 * e2 + e3 * e3;
    }

    pub fn unit_vector(&self) -> Vec3<f64> {
        Vec3 {
            e1: self.e1.into() / self.length(),
            e2: self.e2.into() / self.length(),
            e3: self.e3.into() / self.length(),
        }
    }

    pub fn println_color(&self) {
        let r: f64 = self.e1.into() * 255.999;
        let b: f64 = self.e2.into() * 255.999;
        let g: f64 = self.e3.into() * 255.999;

        println!("{} {} {}", r as usize, b as usize, g as usize,);
    }
}

impl<T: ops::Neg<Output = T> + Copy> ops::Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3 {
            e1: -self.e1,
            e2: -self.e2,
            e3: -self.e3,
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
            e3: self.e3 + rhs.e3,
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: T) -> Self::Output {
        Vec3 {
            e1: self.e1 + rhs,
            e2: self.e2 + rhs,
            e3: self.e3 + rhs,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
            e3: self.e3 - rhs.e3,
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: T) -> Self::Output {
        Vec3 {
            e1: self.e1 - rhs,
            e2: self.e2 - rhs,
            e3: self.e3 - rhs,
        }
    }
}

impl<T: ops::Mul<Output = T> + Copy> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, t: T) -> Vec3<T> {
        Vec3 {
            e1: self.e1 * t,
            e2: self.e2 * t,
            e3: self.e3 * t,
        }
    }
}

impl<T: ops::Div<Output = T> + Copy> ops::Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, t: T) -> Vec3<T> {
        Vec3 {
            e1: self.e1 / t,
            e2: self.e2 / t,
            e3: self.e3 / t,
        }
    }
}

impl<T: Copy + Clone> ops::Index<usize> for Vec3<T> {
    type Output = T;

    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        match index {
            0 => &self.e1,
            1 => &self.e2,
            2 => &self.e3,
            _ => panic!("Vec3 takes index of from 0 to 2"),
        }
    }
}

impl<T: Copy + Clone> ops::IndexMut<usize> for Vec3<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut T {
        match index {
            0 => &mut self.e1,
            1 => &mut self.e2,
            2 => &mut self.e3,
            _ => panic!("Vec3 takes index of from 0 to 2"),
        }
    }
}

impl<T: Copy + std::fmt::Display> std::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e1, self.e2, self.e3)
    }
}

#[allow(dead_code)]
pub fn dot<T: ops::Mul<Output = T> + ops::Add<Output = T> + Copy>(v1: &Vec3<T>, v2: &Vec3<T>) -> T {
    v1.e1 * v2.e1 + v1.e2 * v2.e2 + v1.e3 * v2.e3
}

#[allow(dead_code)]
pub fn cross<T: ops::Mul<Output = T> + ops::Sub<Output = T> + Copy>(
    u: &Vec3<T>,
    v: &Vec3<T>,
) -> Vec3<T> {
    Vec3 {
        e1: u.e2 * v.e3 - u.e3 * v.e2,
        e2: u.e3 * v.e1 - u.e1 * v.e3,
        e3: u.e1 * v.e2 - u.e2 * v.e1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Vec3::new(),
            Vec3 {
                e1: f64::default(),
                e2: f64::default(),
                e3: f64::default(),
            }
        );
    }

    #[test]
    fn test_build() {
        assert_eq!(
            Vec3::build(1., 2., 3.),
            Vec3 {
                e1: 1.,
                e2: 2.,
                e3: 3.,
            }
        );
    }

    #[test]
    fn test_x() {
        let v = Vec3::build(1, 2, 3);
        assert_eq!(
            v.x(),
            1,
            );
    }

    #[test]
    fn test_y() {
        let v = Vec3::build(1, 2, 3);
        assert_eq!(
            v.y(),
            2,
            );
    }

    #[test]
    fn test_z() {
        let v = Vec3::build(1, 2, 3);
        assert_eq!(
            v.z(),
            3,
            );
    }

    #[test]
    fn test_length() {
        let v = Vec3::build(1, 2, 3);
        let ans = (1f64*1f64+2f64*2f64+3f64*3f64).sqrt();
        assert_eq!(v.length(), ans);
    }

    #[test]
    fn test_length_squared() {
        let v = Vec3::build(1, 2, 3);
        let ans = 1f64*1f64+2f64*2f64+3f64*3f64;
        assert_eq!(v.length_squared(), ans);
    }

    #[test]
    fn test_unit_vector() {
        let v = Vec3::build(1, 2, 2);
        let ans = Vec3 {
            e1: 1. / 3.,
            e2: 2. / 3.,
            e3: 2. / 3.,
        };

        assert_eq!(v.unit_vector(), ans);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::build(1, 2, 3);
        let ans = Vec3 {
            e1: -1,
            e2: -2,
            e3: -3,
        };

        assert_eq!(-v, ans);
    }

    #[test]
    fn test_add_vec3() {
        let v1 = Vec3::build(1, 1, 1);
        let v2 = Vec3::build(2, 2, 2);
        let ans = Vec3 {
            e1: 1+2,
            e2: 1+2,
            e3: 1+2,
        };

        assert_eq!(v1+v2, ans);
    }

    #[test]
    fn test_add_t() {
        let v1 = Vec3::build(1, 2, 3);
        let ans = Vec3:: build(2, 3, 4);
        assert_eq!(v1+1, ans);
    }

    #[test]
    fn test_sub_vec3() {
        let v1 = Vec3::build(1, 1, 1);
        let v2 = Vec3::build(2, 2, 2);
        let ans = Vec3 {
            e1: 1-2,
            e2: 1-2,
            e3: 1-2,
        };

        assert_eq!(v1-v2, ans);
    }

    #[test]
    fn test_sub_t() {
        let v1 = Vec3::build(1, 2, 3);
        let ans = Vec3::build(0, 1, 2);
        assert_eq!(v1-1, ans);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::build(1, 1, 1);
        let ans = Vec3 {
            e1: 1*2,
            e2: 1*2,
            e3: 1*2,
        };

        assert_eq!(v1*2, ans);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::build(1., 2., 3.);
        let ans = Vec3 {
            e1: 1./2.,
            e2: 2./2.,
            e3: 3./2.,
        };

        assert_eq!(v1/2., ans);
    }

    #[test]
    fn test_index() {
        let v = Vec3::build(1, 2, 3);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 2);
        assert_eq!(v[2], 3);
    }

    #[test]
    fn test_index_mut() {
        let mut v = Vec3::build(1, 2, 3);
        v[0] = 0;
        v[1] = 0;
        v[2] = 0;
        let ans = Vec3 {
            e1: 0,
            e2: 0,
            e3: 0,
        };
        assert_eq!(v, ans);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::build(1, 2, 3);
        let v2 = Vec3::build(1, 2, 3);
        assert_eq!(dot(&v1, &v2), 1+4+9);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::build(0, 1, 2);
        let v2 = Vec3::build(0, 1, 2);
        let ans = Vec3 {
            e1: 1*2 - 2*1,
            e2: 2*0 - 0*2,
            e3: 0*1 - 1*0,
        };
        assert_eq!(cross(&v1, &v2), ans);
    }
}
