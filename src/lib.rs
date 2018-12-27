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

pub fn color_vec(ray: &Ray, world: &Shapes, depth: usize, rng: &mut impl Rng) -> Vec3 {
    if let Some(rec) = world.hit(ray, 0.001, std::f64::MAX) {
        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec, rng) {
            if depth < 50 {
                return attenuation * color_vec(&scattered, world, depth + 1, rng);
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
