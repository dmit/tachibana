use image;
use std::env;

use tachibana::color::Color;
use tachibana::ray::Ray;
use tachibana::shape::{Shape, Shapes, Sphere};
use tachibana::vec::Vec3;

fn color(r: &Ray, world: &Shapes) -> Color {
    if let Some(rec) = world.hit(r, 0., std::f64::MAX) {
        let c = Vec3 {
            x: rec.normal.x + 1.,
            y: rec.normal.y + 1.,
            z: rec.normal.z + 1.,
        } * 0.5;
        c.into()
    } else {
        let unit_direction = r.direction.unit();
        let t = (unit_direction.y + 1.) * 0.5;
        let color = Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.,
        };
        let c = Vec3::ONE * (1. - t) + color * t;
        c.into()
    }
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

    #[rustfmt::skip]
    let shapes: Shapes = {
        let mut s = Shapes::new();
        s.add(Sphere{ center: Vec3{x: 0., y:    0. , z: -1.}, radius:   0.5 });
        s.add(Sphere{ center: Vec3{x: 0., y: -100.5, z: -1.}, radius: 100. });
        s
    };

    let mut buf = image::ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let u = f64::from(x) / f64::from(width);
            let v = f64::from(y) / f64::from(height);
            let ray = Ray {
                origin: Vec3::ZERO,
                direction: lower_left_corner + horizontal * u + vertical * v,
            };
            let color = color(&ray, &shapes);

            let p = buf.get_pixel_mut(x, height - y - 1); // write image starting from the bottom row
            *p = image::Rgb(color.into());
        }
    }

    buf.save("out.png").expect("Unable to write output file");
}
