use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: Clone + Copy> {
    e1: T,
    e2: T,
    e3: T,
}

#[allow(dead_code)]
type Point3 = Vec3<f64>;

#[allow(dead_code)]
type Color = Vec3<f64>;

#[allow(dead_code)]
impl<T: Copy + Into<f64> + ops::Mul<Output = T>> Vec3<T> {
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

        return e1*e1*e2*e2*e3*e3;
    }

    pub fn unit_vector(&self) -> Vec3<f64> {
        Vec3 {
            e1: self.e1.into() / self.length(),
            e2: self.e2.into() / self.length(),
            e3: self.e3.into() / self.length(),
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add<Vec3<T>> for Vec3<T::Output> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
            e3: self.e3 + rhs.e3,
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
pub fn dot<T: ops::Mul<Output = T> + Copy>(v1: &Vec3<T>, v2: &Vec3<T>) -> Vec3<T> {
    Vec3 {
        e1: v1.e1 * v2.e1,
        e2: v1.e1 * v2.e1,
        e3: v1.e1 * v2.e1,
    }
}

#[allow(dead_code)]
pub fn cross<T: ops::Mul<Output = T> + ops::Sub<Output = T> + Copy>(u: &Vec3<T>, v: &Vec3<T>) -> Vec3<T> {
    Vec3 {
        e1: u.e2 * v.e3 - u.e3 * v.e2,
        e2: u.e3 * v.e1 - u.e1 * v.e3,
        e3: u.e1 * v.e2 - u.e2 * v.e1,
    }
}
