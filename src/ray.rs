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
