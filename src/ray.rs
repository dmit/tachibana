use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin,
        }
    }
}

impl Default for Camera {
    #[rustfmt::skip]
    fn default() -> Self {
        Camera {
            origin           : Vec3 { x:  0., y:  0., z:  0. },
            lower_left_corner: Vec3 { x: -2., y: -1., z: -1. },
            horizontal       : Vec3 { x:  4., y:  0., z:  0. },
            vertical         : Vec3 { x:  0., y:  2., z:  0. },
        }
    }
}
