use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<Vec3<f32>> for Color {
    #[inline]
    fn from(a: Vec3<f32>) -> Self {
        Color {
            r: (a.x * 255.99) as u8,
            g: (a.y * 255.99) as u8,
            b: (a.z * 255.99) as u8,
        }
    }
}

impl From<Vec3<f64>> for Color {
    #[inline]
    fn from(a: Vec3<f64>) -> Self {
        Color {
            r: (a.x * 255.99) as u8,
            g: (a.y * 255.99) as u8,
            b: (a.z * 255.99) as u8,
        }
    }
}
