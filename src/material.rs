use rand::Rng;

use crate::ray::Ray;
use crate::shape::HitRecord;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian(Vec3),
    Metal(Vec3, f64),
}

impl Material {
    #[inline]
    fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - *n * v.dot(*n) * 2.
    }

    pub fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        rng: &mut impl Rng,
    ) -> Option<(Vec3, Ray)> {
        use self::Material::*;

        match *self {
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
