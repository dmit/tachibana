#![feature(duration_float)]

use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use image;
use rand::prelude::*;
use rayon::prelude::*;
use structopt::{self, StructOpt};
use xoshiro::Xoshiro256Plus;

use tachibana::color::Color;
use tachibana::material::Material;
use tachibana::ray::Camera;
use tachibana::shape::{Shapes, Sphere};
use tachibana::vec::Vec3;

#[derive(Debug, StructOpt)]
#[structopt(name = "tachibana")]
struct Cfg {
    #[structopt(short, long, default_value = "2048")]
    width: u32,

    #[structopt(short, long, default_value = "1024")]
    height: u32,

    #[structopt(short, long, default_value = "100")]
    rays_per_pixel: u32,

    #[structopt(short = "b", long, default_value = "50")]
    max_bounces: u32,

    #[structopt(short = "s", long, default_value = "500")]
    max_spheres: u32,

    out_file: Option<String>,
}

fn main() -> io::Result<()> {
    let cfg = Cfg::from_args();

    let rng_seed = rand::thread_rng().gen();
    let mut rng = Xoshiro256Plus::from_seed_u64(rng_seed);

    let shapes: Shapes = {
        let mut s = Shapes::new();
        s.add(Sphere {
            center: Vec3 {
                x: 0.,
                y: -1000.,
                z: 0.,
            },
            radius: 1000.,
            material: Material::Lambertian(Vec3 {
                x: 0.5,
                y: 0.5,
                z: 0.5,
            }),
        });

        let middle = Vec3 {
            x: 4.,
            y: 0.2,
            z: 0.,
        };

        let ab_range = {
            let range_len = f64::from(cfg.max_spheres).sqrt().floor() as i32;
            let from = 0 - (range_len / 2);
            let to = from + range_len;
            from..to
        };
        for a in ab_range.clone() {
            for b in ab_range.clone() {
                let center = Vec3 {
                    x: f64::from(a) + rng.gen::<f64>() * 0.9,
                    y: 0.2,
                    z: f64::from(b) + rng.gen::<f64>() * 0.9,
                };

                if (center - middle).length() > 0.9 {
                    let rnd_material = rng.gen_range(0, 100);
                    match rnd_material {
                        0...79 => {
                            // diffuse
                            let rnd_albedo = Vec3 {
                                x: rng.gen::<f64>() * rng.gen::<f64>(),
                                y: rng.gen::<f64>() * rng.gen::<f64>(),
                                z: rng.gen::<f64>() * rng.gen::<f64>(),
                            };
                            s.add(Sphere {
                                center,
                                radius: 0.2,
                                material: Material::Lambertian(rnd_albedo),
                            });
                        }
                        80...94 => {
                            // metal
                            let albedo = Vec3 {
                                x: 0.5 * (1. + rng.gen::<f64>()),
                                y: 0.5 * (1. + rng.gen::<f64>()),
                                z: 0.5 * (1. + rng.gen::<f64>()),
                            };
                            let fuzz = 0.5 * rng.gen::<f64>();
                            s.add(Sphere {
                                center,
                                radius: 0.2,
                                material: Material::Metal(albedo, fuzz),
                            });
                        }
                        95...100 => {
                            // glass
                            s.add(Sphere {
                                center,
                                radius: 0.2,
                                material: Material::Dielectric(1.5),
                            });
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }

        s.add(Sphere {
            center: Vec3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            radius: 1.,
            material: Material::Dielectric(1.5),
        });
        s.add(Sphere {
            center: Vec3 {
                x: -4.,
                y: 1.,
                z: 0.,
            },
            radius: 1.,
            material: Material::Lambertian(Vec3 {
                x: 0.4,
                y: 0.2,
                z: 0.1,
            }),
        });
        s.add(Sphere {
            center: Vec3 {
                x: 4.,
                y: 1.,
                z: 0.,
            },
            radius: 1.,
            material: Material::Metal(
                Vec3 {
                    x: 0.7,
                    y: 0.6,
                    z: 0.5,
                },
                0.,
            ),
        });

        s
    };

    #[rustfmt::skip]
    let camera = {
        let look_from = Vec3 { x: 13., y: 2., z:  3. };
        let look_at   = Vec3 { x:  0., y: 0., z:  0. };
        let view_up   = Vec3 { x:  0., y: 1., z:  0. };

        Camera::new(
            look_from,
            look_at,
            view_up,
            30.,
            f64::from(cfg.width) / f64::from(cfg.height),
            0.1,
            10.,
        )
    };

    let total_rays = cfg.width * cfg.height * cfg.rays_per_pixel;
    let ten_percent = total_rays as usize / 10;
    let mut buf = image::ImageBuffer::new(cfg.width, cfg.height);

    println!(
        "Rendering {}x{} image with {} spheres and {} rays per pixel ({} max bounces per ray) = {} total rays (seed: {:x})",
        cfg.width,
        cfg.height,
        shapes.size(),
        cfg.rays_per_pixel,
        cfg.max_bounces,
        tachibana::delimited_int(',', total_rays),
        rng_seed,
    );

    let rays_counter = AtomicUsize::new(0);
    let start_time = Instant::now();

    let coords: Vec<(u32, u32)> = (0..cfg.height)
        .flat_map(|y| (0..cfg.width).map(move |x| (x, y)))
        .collect();

    let pixels: Vec<Color> = coords
        .par_iter()
        .map(|&(x, y)| {
            let y = cfg.height - y - 1; // tracer renders bottom to top
            let mut rng = Xoshiro256Plus::from_seed_u64(rand::thread_rng().gen());

            let c_vec = (0..cfg.rays_per_pixel).fold(Vec3::ZERO, |acc, _| {
                let u = (f64::from(x) + rng.gen::<f64>()) / f64::from(cfg.width);
                let v = (f64::from(y) + rng.gen::<f64>()) / f64::from(cfg.height);
                let ray = camera.ray(u, v, &mut rng);
                let c = tachibana::color_vec(&ray, &shapes, cfg.max_bounces, 0, &mut rng);

                let rays_rendered = rays_counter.fetch_add(1, Ordering::Relaxed) + 1;
                if rays_rendered % ten_percent == 0 {
                    let duration = start_time.elapsed();
                    let rays_per_s = rays_rendered as f64 / duration.as_float_secs();
                    println!(
                        "{:3}0% {:4}.{:0<3}s ({} rays/s)",
                        rays_rendered / ten_percent,
                        duration.as_secs(),
                        duration.subsec_millis(),
                        tachibana::delimited_int(',', rays_per_s.round() as i64)
                    );
                }

                acc + c
            }) / f64::from(cfg.rays_per_pixel);

            c_vec.map(tachibana::gamma_linear_to_srgb).into()
        })
        .collect();

    buf.enumerate_pixels_mut().for_each(|(x, y, p)| {
        let c = pixels[(y * cfg.width + x) as usize];
        *p = image::Rgb(c.as_array());
    });

    let filename = cfg.out_file.unwrap_or_else(|| "out.png".to_string());
    buf.save(&filename)?;

    Ok(())
}
