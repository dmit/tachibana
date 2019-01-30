use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.,
        y: 0.,
        z: 0.,
    };

    pub const ONE: Vec3 = Vec3 {
        x: 1.,
        y: 1.,
        z: 1.,
    };

    #[inline]
    pub fn map<F: Fn(f32) -> f32>(&self, f: F) -> Vec3 {
        Vec3 {
            x: f(self.x),
            y: f(self.y),
            z: f(self.z),
        }
    }

    #[inline]
    pub fn squared_length(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(self) -> f32 {
        self.squared_length().sqrt()
    }

    #[inline]
    pub fn unit(self) -> Vec3 {
        self / self.length()
    }

    #[inline]
    pub fn dot(&self, a: Vec3) -> f32 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    #[inline]
    pub fn cross(&self, a: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * a.z - self.z * a.y,
            y: self.z * a.x - self.x * a.z,
            z: self.x * a.y - self.y * a.x,
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn add(self, a: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + a.x,
            y: self.y + a.y,
            z: self.z + a.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn sub(self, a: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - a.x,
            y: self.y - a.y,
            z: self.z - a.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, a: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * a.x,
            y: self.y * a.y,
            z: self.z * a.z,
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, a: Vec3) -> Self::Output {
        Vec3 {
            x: self.x / a.x,
            y: self.y / a.y,
            z: self.z / a.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn mul(self, a: f32) -> Self::Output {
        Vec3 {
            x: self.x * a,
            y: self.y * a,
            z: self.z * a,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    #[inline]
    fn div(self, a: f32) -> Self::Output {
        Vec3 {
            x: self.x / a,
            y: self.y / a,
            z: self.z / a,
        }
    }
}
