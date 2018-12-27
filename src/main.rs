use image;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::env;

use tachibana::color::Color;
use tachibana::ray::{Camera, Ray};
use tachibana::shape::{Shape, Shapes, Sphere};
use tachibana::vec::Vec3;

fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
    let mut p: Vec3;
    while {
        let rnd = Vec3 {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        };
        p = rnd * 2. - Vec3::ONE;

        p.squared_length() >= 1.
    } {}

    p
}

fn color_vec(ray: &Ray, world: &Shapes, rng: &mut impl Rng) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f64::MAX) {
        let rnd = random_in_unit_sphere(rng);
        let target = rec.point + rec.normal + rnd;

        let new_ray = Ray {
            origin: rec.point,
            direction: target - rec.point,
        };
        color_vec(&new_ray, world, rng) * 0.5
    } else {
        let unit_direction = ray.direction.unit();
        let t = (unit_direction.y + 1.) * 0.5;
        let color = Vec3 {
            x: 0.5,
            y: 0.7,
            z: 1.,
        };
        Vec3::ONE * (1. - t) + color * t
    }
}

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
        s.add(Sphere{ center: Vec3{x: 0., y:    0. , z: -1.}, radius:   0.5 });
        s.add(Sphere{ center: Vec3{x: 0., y: -100.5, z: -1.}, radius: 100. });
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
                    let c = color_vec(&ray, &shapes, &mut rng);
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
