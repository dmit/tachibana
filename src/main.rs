use std::{
    error::Error,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
    time::Instant,
};

use argh::FromArgs;
use fastrand::Rng;
use rayon::prelude::*;
use tachibana::{
    color::Color,
    material::Material,
    ray::Camera,
    shape::{Shapes, Sphere},
    tracer::Tracer,
    vec::Vec3,
};

#[derive(Debug, FromArgs)]
#[argh(name = "tachibana", description = "Tachibana - a toy ray tracer")]
struct Cfg {
    #[argh(option, short = 'w', description = "output image width in pixels", default = "2048")]
    width: u32,

    #[argh(option, short = 'h', description = "output image height in pixels", default = "1024")]
    height: u32,

    #[argh(option, short = 'r', description = "rays per pixel", default = "100")]
    rays_per_pixel: u32,

    #[argh(
        option,
        short = 'b',
        description = "maximum number of bounces a ray performs",
        default = "50"
    )]
    max_bounces: u32,

    #[argh(
        option,
        short = 's',
        description = "maximum number of randomly placed spheres in the scene",
        default = "500"
    )]
    max_spheres: u32,

    #[argh(option, short = 'c', description = "parallelism chunk size", default = "1")]
    chunk_size: usize,

    #[argh(positional, default = r#"PathBuf::from("out.png")"#)]
    out_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cfg: Cfg = argh::from_env();

    let rng_seed = fastrand::u64(..);
    let mut rng = Rng::with_seed(rng_seed);

    let shapes = gen_scene(cfg.max_spheres, &mut rng);

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
            cfg.width as f32 / cfg.height as f32,
            0.1,
            10.,
        )
    };

    let total_rays = cfg.width * cfg.height * cfg.rays_per_pixel;

    println!(
        "Rendering {}x{}={}px image (in chunks of {}) with {} spheres and {} rays per pixel ({} max bounces per ray) = {} total rays (seed: {:x})",
        cfg.width,
        cfg.height,
        tachibana::delimited_int(',', cfg.width * cfg.height),
        cfg.chunk_size,
        shapes.size(),
        cfg.rays_per_pixel,
        cfg.max_bounces,
        tachibana::delimited_int(',', total_rays),
        rng_seed,
    );

    let tracer = Tracer::new(&camera, &shapes, cfg.width, cfg.height, cfg.max_bounces);

    let ray_counter = AtomicUsize::new(0);
    let chunk_counter = AtomicUsize::new(0);

    let coords: Vec<(u32, u32)> =
        (0..cfg.height).flat_map(|y| (0..cfg.width).map(move |x| (x, y))).collect();
    let ten_percent = coords.len() / cfg.chunk_size as usize / 10;

    let start_time = Instant::now();
    let pixels: Vec<Color> = coords
        .par_chunks(cfg.chunk_size)
        .flat_map(|chunk: &[(u32, u32)]| {
            let mut rng = Rng::new();
            let pixels: Vec<Color> = chunk
                .iter()
                .map(|&(x, y)| {
                    let y = cfg.height - y - 1; // tracer renders bottom to top
                    tracer.trace_pixel(x, y, cfg.rays_per_pixel, &mut rng)
                })
                .collect();

            let chunks_processed = chunk_counter.fetch_add(1, Ordering::Relaxed) + 1;
            let rays_in_this_chunk = chunk.len() * cfg.rays_per_pixel as usize;
            let rays_rendered =
                ray_counter.fetch_add(rays_in_this_chunk, Ordering::Relaxed) + rays_in_this_chunk;
            if chunks_processed % ten_percent == 0 {
                let duration = start_time.elapsed();
                let rays_per_s = rays_rendered as f64 / duration.as_secs_f64();
                let micros_per_ray = duration.as_micros() as f32 / rays_rendered as f32;
                println!(
                    "{:3}0% {:4}.{:0<3}s ({} rays/s, {:.3} μs/ray)",
                    chunks_processed / ten_percent,
                    duration.as_secs(),
                    duration.subsec_millis(),
                    tachibana::delimited_int(',', rays_per_s.round() as i64),
                    micros_per_ray,
                );
            }

            pixels
        })
        .collect();

    let mut buf = image::ImageBuffer::new(cfg.width, cfg.height);
    buf.enumerate_pixels_mut().for_each(|(x, y, p)| {
        let c = pixels[(y * cfg.width + x) as usize];
        *p = image::Rgb(c.as_array());
    });

    buf.save(&cfg.out_file)?;

    Ok(())
}

fn gen_scene(max_spheres: u32, rng: &mut Rng) -> Shapes {
    let mut s = Shapes::new();
    s.add(Sphere {
        center: Vec3 { x: 0., y: -1000., z: 0. },
        radius: 1000.,
        material: Material::Lambertian(Vec3 { x: 0.5, y: 0.5, z: 0.5 }),
    });

    let middle = Vec3 { x: 4., y: 0.2, z: 0. };

    let ab_range = {
        let range_len = f64::from(max_spheres).sqrt().floor() as i32;
        let from = 0 - (range_len / 2);
        let to = from + range_len;
        from..to
    };
    for a in ab_range.clone() {
        for b in ab_range.clone() {
            let center =
                Vec3 { x: a as f32 + rng.f32() * 0.9, y: 0.2, z: b as f32 + rng.f32() * 0.9 };

            if (center - middle).length() > 0.9 {
                let rnd_material = rng.u8(0..=100);
                match rnd_material {
                    0..=79 => {
                        // diffuse
                        let rnd_albedo = Vec3 {
                            x: rng.f32() * rng.f32(),
                            y: rng.f32() * rng.f32(),
                            z: rng.f32() * rng.f32(),
                        };
                        s.add(Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Lambertian(rnd_albedo),
                        });
                    }
                    80..=94 => {
                        // metal
                        let albedo = Vec3 {
                            x: 0.5 * (1. + rng.f32()),
                            y: 0.5 * (1. + rng.f32()),
                            z: 0.5 * (1. + rng.f32()),
                        };
                        let fuzz = 0.5 * rng.f32();
                        s.add(Sphere {
                            center,
                            radius: 0.2,
                            material: Material::Metal(albedo, fuzz),
                        });
                    }
                    95..=100 => {
                        // glass
                        s.add(Sphere { center, radius: 0.2, material: Material::Dielectric(1.5) });
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    s.add(Sphere {
        center: Vec3 { x: 0., y: 1., z: 0. },
        radius: 1.,
        material: Material::Dielectric(1.5),
    });
    s.add(Sphere {
        center: Vec3 { x: -4., y: 1., z: 0. },
        radius: 1.,
        material: Material::Lambertian(Vec3 { x: 0.4, y: 0.2, z: 0.1 }),
    });
    s.add(Sphere {
        center: Vec3 { x: 4., y: 1., z: 0. },
        radius: 1.,
        material: Material::Metal(Vec3 { x: 0.7, y: 0.6, z: 0.5 }, 0.),
    });

    s
}
