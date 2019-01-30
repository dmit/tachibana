pub mod color;
pub mod material;
pub mod ray;
pub mod shape;
pub mod vec;

use rand::Rng;

use crate::ray::Ray;
use crate::shape::{Shape, Shapes};
use crate::vec::Vec3;

pub fn random_in_unit_sphere(rng: &mut impl Rng) -> Vec3 {
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

pub fn color_vec(
    ray: &Ray,
    world: &Shapes,
    max_depth: u32,
    depth: u32,
    rng: &mut impl Rng,
) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f64::MAX) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec, rng) {
            if depth < max_depth {
                return attenuation * color_vec(&scattered, world, max_depth, depth + 1, rng);
            }
        }
        return Vec3::ZERO;
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

pub fn gamma_linear_to_srgb(x: f64) -> f64 {
    if x <= 0.0031308 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}

pub fn delimited_int<T: ToString>(delim: char, value: T) -> String {
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
