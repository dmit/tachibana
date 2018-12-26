use image;
use num::{One, Zero};
use std::env;
use std::ops::{Add, Mul};

use tachibana::color::Color;
use tachibana::vec::Vec3;

type Precision = f64;

#[derive(Clone, Copy, Debug)]
pub struct Ray<T> {
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T>
where
    T: Copy,
    Vec3<T>: Add<Vec3<T>, Output = Vec3<T>>,
    Vec3<T>: Mul<T, Output = Vec3<T>>,
{
    pub fn point_at(self, t: T) -> Vec3<T> {
        self.origin + self.direction * t
    }
}

fn color(r: &Ray<Precision>) -> Color {
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.);
    let color = Vec3 {
        x: 0.5,
        y: 0.7,
        z: 1.,
    };
    let c = Vec3::one() * (1. - t) + color * t;
    c.into()
}

fn main() {
    let mut args = env::args().skip(1).take(2).map(|x| x.parse::<u32>());
    let width: u32 = match args.next() {
        Some(Ok(w)) => w,
        Some(Err(e)) => panic!("Invalid format for image width: {}", e),
        _ => 2048,
    };
    let height: u32 = match args.next() {
        Some(Ok(h)) => h,
        Some(Err(e)) => panic!("Invalid format for image height: {}", e),
        _ => width / 2,
    };

    let lower_left_corner = Vec3 {
        x: -2.,
        y: -1.,
        z: -1.,
    };
    let horizontal = Vec3 {
        x: 4.,
        y: 0.,
        z: 0.,
    };
    let vertical = Vec3 {
        x: 0.,
        y: 2.,
        z: 0.,
    };

    let mut buf = image::ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let u = Precision::from(x) / Precision::from(width);
            let v = Precision::from(y) / Precision::from(height);
            let ray = Ray {
                origin: Vec3::zero(),
                direction: lower_left_corner + horizontal * u + vertical * v,
            };
            let color = color(&ray);

            let p = buf.get_pixel_mut(x, height - y - 1); // write image starting from the bottom row
            *p = image::Rgb(color.into());
        }
    }

    buf.save("out.png").expect("Unable to write output file");
}
