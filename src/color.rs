use std::u8;

use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[rustfmt::skip]
impl Color {
    pub const RED  : Color = Color{r: u8::MAX, g: 0      , b: 0      };
    pub const GREEN: Color = Color{r: 0      , g: u8::MAX, b: 0      };
    pub const BLUE : Color = Color{r: 0      , g: 0      , b: u8::MAX};
    pub const WHITE: Color = Color{r: u8::MAX, g: u8::MAX, b: u8::MAX};
    pub const BLACK: Color = Color{r: 0      , g: 0      , b: 0      };

    pub fn as_array(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<Vec3> for Color {
    #[inline]
    fn from(a: Vec3) -> Self {
        Color {
            r: (a.x * 255.99) as u8,
            g: (a.y * 255.99) as u8,
            b: (a.z * 255.99) as u8,
        }
    }
}
