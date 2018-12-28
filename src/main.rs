#![feature(duration_float)]

use image;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use std::env;
use std::time::Instant;

use tachibana::color::Color;
use tachibana::material::Material;
use tachibana::ray::Camera;
use tachibana::shape::{Shapes, Sphere};
use tachibana::vec::Vec3;

fn delimited_int<T: ToString>(delim: char, value: T) -> String {
    let as_str = value.to_string();
    let mut iter = as_str.chars().rev().peekable();
    let mut delimited = String::new();
    let mut char_count = 0;
    while let Some(ch) = iter.next() {
        delimited.insert(0, ch);
        char_count += 1;
        if char_count % 3 == 0 && iter.peek().is_some() {
            delimited.insert(0, delim);
        }
    }
    delimited
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
    let rays_per_pixel: u32 = match args.next() {
        Some(Ok(n)) => n,
        Some(Err(e)) => panic!("Invalid format for number of rays per pixel: {}", e),
        _ => 100,
    };

    let mut rng = Pcg64Mcg::new(rand::thread_rng().gen());

    #[rustfmt::skip]
    let shapes: Shapes = {
        let mut s = Shapes::new();
        s.add(Sphere{ center: Vec3{x:  0., y:    0. , z: -1.}, radius:   0.5 , material: Material::Lambertian(Vec3{x: 0.1, y: 0.2, z: 0.5})});
        s.add(Sphere{ center: Vec3{x:  0., y: -100.5, z: -1.}, radius: 100.  , material: Material::Lambertian(Vec3{x: 0.8, y: 0.8, z: 0.0})});
        s.add(Sphere{ center: Vec3{x:  1., y:    0. , z: -1.}, radius:   0.5 , material: Material::Metal(Vec3{x: 0.8, y: 0.6, z: 0.2}, 0.)});
        s.add(Sphere{ center: Vec3{x: -1., y:    0. , z: -1.}, radius:   0.5 , material: Material::Dielectric(1.5)});
        s.add(Sphere{ center: Vec3{x: -1., y:    0. , z: -1.}, radius:  -0.45, material: Material::Dielectric(1.5)});
        s
    };

    #[rustfmt::skip]
    let camera = {
        let look_from = Vec3 { x: -2., y: 2., z:  1. };
        let look_at   = Vec3 { x:  0., y: 0., z: -1. };
        let view_up   = Vec3 { x:  0., y: 1., z:  0. };
        Camera::new(look_from, look_at, view_up, 90., f64::from(width) / f64::from(height))
    };

    let total_rays = width * height * rays_per_pixel;
    let ten_percent = total_rays / 10;
    let mut buf = image::ImageBuffer::new(width, height);

    println!(
        "Rendering {}x{} image with {} rays per pixel = {} total rays",
        width,
        height,
        rays_per_pixel,
        delimited_int(',', total_rays)
    );

    let mut rays_rendered: u32 = 0;
    let start_time = Instant::now();

    for y in 0..height {
        for x in 0..width {
            let color: Color = {
                let c = (0..rays_per_pixel).fold(Vec3::ZERO, |acc, _| {
                    let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(width);
                    let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(height);
                    let ray = camera.ray(u, v);
                    let c = tachibana::color_vec(&ray, &shapes, 0, &mut rng);

                    rays_rendered += 1;
                    if rays_rendered % ten_percent == 0 {
                        let duration = start_time.elapsed();
                        let rays_per_s = f64::from(rays_rendered) / duration.as_float_secs();
                        println!(
                            "{:3}0% {:4}.{:0<3}s ({} rays/s)",
                            rays_rendered / ten_percent,
                            duration.as_secs(),
                            duration.subsec_millis(),
                            delimited_int(',', rays_per_s.round() as i64)
                        );
                    }

                    acc + c
                }) / f64::from(rays_per_pixel);
                c.map(|f| f.sqrt()).into()
            };

            let p = buf.get_pixel_mut(x, height - y - 1); // write image starting from the bottom row
            *p = image::Rgb(color.into());
        }
    }

    buf.save("out.png").expect("Unable to write output file");
}
