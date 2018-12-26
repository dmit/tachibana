use num::{One, Zero};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Zero> Zero for Vec3<T> {
    // Would be nice to have this const, but it's blocked on https://github.com/rust-num/num-traits/pull/91
    fn zero() -> Self {
        Vec3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T: One + Copy> One for Vec3<T> {
    // Would be nice to have this const, but it's blocked on https://github.com/rust-num/num-traits/pull/91
    fn one() -> Self {
        Vec3 {
            x: T::one(),
            y: T::one(),
            z: T::one(),
        }
    }
}

impl<T: Add> Add<Vec3<T>> for Vec3<T>
where
    T: Add<T, Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn add(self, a: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x + a.x,
            y: self.y + a.y,
            z: self.z + a.z,
        }
    }
}

impl<T: Sub> Sub<Vec3<T>> for Vec3<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn sub(self, a: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x - a.x,
            y: self.y - a.y,
            z: self.z - a.z,
        }
    }
}

impl<T: Mul + Copy> Mul<Vec3<T>> for Vec3<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, a: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x * a.x,
            y: self.y * a.y,
            z: self.z * a.z,
        }
    }
}

impl<T: Div + Copy> Div<Vec3<T>> for Vec3<T>
where
    T: Div<T, Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn div(self, a: Vec3<T>) -> Self::Output {
        Vec3 {
            x: self.x / a.x,
            y: self.y / a.y,
            z: self.z / a.z,
        }
    }
}

impl<T: Mul + Copy> Mul<T> for Vec3<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn mul(self, a: T) -> Self::Output {
        Vec3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

impl<T: Div + Copy> Div<T> for Vec3<T>
where
    T: Div<T, Output = T>,
{
    type Output = Vec3<T>;

    #[inline]
    fn div(self, a: T) -> Self::Output {
        Vec3 {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }
}

impl Vec3<f32> {
    #[inline]
    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn unit(self) -> Vec3<f32> {
        self / self.length()
    }

    #[inline]
    pub fn dot(self, a: Vec3<f32>) -> f32 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    #[inline]
    pub fn cross(self, a: Vec3<f32>) -> Vec3<f32> {
        Vec3 {
            x: self.y * a.z - self.z * a.y,
            y: self.x * a.z - self.z * a.x,
            z: self.x * a.y - self.y * a.x,
        }
    }
}

impl Vec3<f64> {
    #[inline]
    pub fn squared_length(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(self) -> f64 {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn unit(self) -> Vec3<f64> {
        self / self.length()
    }

    #[inline]
    pub fn dot(self, a: Vec3<f64>) -> f64 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    #[inline]
    pub fn cross(self, a: Vec3<f64>) -> Vec3<f64> {
        Vec3 {
            x: self.y * a.z - self.z * a.y,
            y: self.x * a.z - self.z * a.x,
            z: self.x * a.y - self.y * a.x,
        }
    }
}
