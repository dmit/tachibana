use rand::Rng;

use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Dielectric(f64),
    Lambertian(Vec3),
    Metal(Vec3, f64),
}

impl Material {
    #[inline]
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * v.dot(*n) * 2.
    }

    #[inline]
    fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
        let uv = v.unit();
        let dt = uv.dot(*n);
        let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);

        if discriminant > 0. {
            let refracted = (uv - *n * dt) * ni_over_nt - *n * discriminant.sqrt();
            Some(refracted)
        } else {
            None
        }
    }

    #[inline]
    fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 * r0 + (1. - r0 * r0) * (1. - cosine).powi(5)
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<(Vec3, Ray)> {
        use self::Material::*;

        match *self {
            Dielectric(ref_idx) => {
                let (outward_normal, ni_over_nt, cosine) = if ray_in.direction.dot(rec.normal) > 0.
                {
                    let cosine =
                        ref_idx * ray_in.direction.dot(rec.normal) / ray_in.direction.length();
                    (-rec.normal, ref_idx, cosine)
                } else {
                    let cosine = -ray_in.direction.dot(rec.normal) / ray_in.direction.length();
                    (rec.normal, 1. / ref_idx, cosine)
                };

                if let Some(refracted) =
                    Material::refract(&ray_in.direction, &outward_normal, ni_over_nt)
                {
                    let reflect_prob = Material::schlick(cosine, ref_idx);
                    let rnd: f64 = rng.gen();

                    let scattered = if rnd < reflect_prob {
                        let reflected = Material::reflect(&ray_in.direction, &rec.normal);
                        Ray {
                            origin: rec.point,
                            direction: reflected,
                        }
                    } else {
                        Ray {
                            origin: rec.point,
                            direction: refracted,
                        }
                    };
                    Some((Vec3::ONE, scattered))
                } else {
                    let reflected = Material::reflect(&ray_in.direction, &rec.normal);
                    let scattered = Ray {
                        origin: rec.point,
                        direction: reflected,
                    };
                    Some((Vec3::ONE, scattered))
                }
            }

            Lambertian(albedo) => {
                let rnd = crate::random_in_unit_sphere(rng);
                let target = rec.point + rec.normal + rnd;
                let scattered = Ray {
                    origin: rec.point,
                    direction: target - rec.point,
                };
                Some((albedo, scattered))
            }

            Metal(albedo, fuzz) => {
                let fuzz = fuzz.min(1.);
                let reflected = Material::reflect(&ray_in.direction.unit(), &rec.normal);
                let rnd = crate::random_in_unit_sphere(rng);
                let scattered = Ray {
                    origin: rec.point,
                    direction: reflected + rnd * fuzz,
                };

                if scattered.direction.dot(rec.normal) > 0. {
                    Some((albedo, scattered))
                } else {
                    None
                }
            }
        }
    }
}
