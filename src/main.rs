use image;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::env;

use tachibana::color::Color;
use tachibana::material::Material;
use tachibana::ray::Camera;
use tachibana::shape::{Shapes, Sphere};
use tachibana::vec::Vec3;

fn main() {
    let mut args = env::args().skip(1).take(3).map(|x| x.parse::<u32>());
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
    let num_rays: u32 = match args.next() {
        Some(Ok(n)) => n,
        Some(Err(e)) => panic!("Invalid format for number of rays: {}", e),
        _ => 100,
    };

    #[rustfmt::skip]
    let shapes: Shapes = {
        let mut s = Shapes::new();
        s.add(Sphere{ center: Vec3{x:  0., y:    0. , z: -1.}, radius:   0.5, material: Material::Lambertian(Vec3{x: 0.8, y: 0.3, z: 0.3})});
        s.add(Sphere{ center: Vec3{x:  0., y: -100.5, z: -1.}, radius: 100. , material: Material::Lambertian(Vec3{x: 0.8, y: 0.8, z: 0.0})});
        s.add(Sphere{ center: Vec3{x:  1., y:    0. , z: -1.}, radius:   0.5, material: Material::Metal(Vec3{x: 0.8, y: 0.6, z: 0.2}, 1.0)});
        s.add(Sphere{ center: Vec3{x: -1., y:    0. , z: -1.}, radius:   0.5, material: Material::Metal(Vec3{x: 0.8, y: 0.8, z: 0.8}, 0.3)});
        s
    };

    let mut buf = image::ImageBuffer::new(width, height);
    let camera = Camera::default();
    let mut rng = Pcg64Mcg::new(rand::thread_rng().gen());

    for y in 0..height {
        for x in 0..width {
            let color: Color = {
                let c = (0..num_rays).fold(Vec3::ZERO, |acc, _| {
                    let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(width);
                    let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(height);
                    let ray = camera.ray(u, v);
                    let c = tachibana::color_vec(&ray, &shapes, 0, &mut rng);
                    acc + c
                }) / f64::from(num_rays);
                c.map(|f| f.sqrt()).into()
            };

            let p = buf.get_pixel_mut(x, height - y - 1); // write image starting from the bottom row
            *p = image::Rgb(color.into());
        }
    }

    buf.save("out.png").expect("Unable to write output file");
}
