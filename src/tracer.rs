use rand::Rng;

use crate::color::Color;
use crate::ray::{Camera, Ray};
use crate::shape::{Shape, Shapes};
use crate::vec::Vec3;

pub struct Tracer<'a> {
    camera: &'a Camera,
    shapes: &'a Shapes<'a>,
    width: u32,
    height: u32,
    max_bounces: u32,
}

impl<'a> Tracer<'a> {
    pub fn new(
        camera: &'a Camera,
        shapes: &'a Shapes,
        width: u32,
        height: u32,
        max_bounces: u32,
    ) -> Tracer<'a> {
        Tracer {
            camera,
            shapes,
            width,
            height,
            max_bounces,
        }
    }

    fn color_vec(
        ray: &Ray,
        world: &Shapes,
        max_depth: u32,
        depth: u32,
        rng: &mut impl Rng,
    ) -> Vec3 {
        if let Some(rec) = world.hit(ray, 0.001, std::f32::MAX) {
            if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec, rng) {
                if depth < max_depth {
                    return attenuation
                        * Self::color_vec(&scattered, world, max_depth, depth + 1, rng);
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

    fn gamma_linear_to_srgb(x: f32) -> f32 {
        if x <= 0.0031308 {
            x * 12.92
        } else {
            1.055 * x.powf(1.0 / 2.4) - 0.055
        }
    }

    pub fn trace_pixel(&self, x: u32, y: u32, rays_per_pixel: u32, mut rng: &mut impl Rng) -> Color {
        let c_vec = (0..rays_per_pixel).fold(Vec3::ZERO, |acc, _| {
            let u = (x as f32 + rng.gen::<f32>()) / self.width as f32;
            let v = (y as f32 + rng.gen::<f32>()) / self.height as f32;
            let ray = self.camera.ray(u, v, &mut rng);
            let c = Self::color_vec(&ray, self.shapes, self.max_bounces, 0, &mut rng);
            acc + c
        }) / rays_per_pixel as f32;

        c_vec.map(Self::gamma_linear_to_srgb).into()
    }
}
