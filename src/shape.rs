use std::fmt::Debug;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub distance: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Shape: Debug {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant > 0. {
            let t = (-b - discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let point = ray.point_at(t);
                let rec = HitRecord {
                    distance: t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material,
                };
                return Some(rec);
            }

            let t = (-b + discriminant.sqrt()) / a;
            if t > t_min && t < t_max {
                let point = ray.point_at(t);
                let rec = HitRecord {
                    distance: t,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: self.material,
                };
                return Some(rec);
            }
        }

        None
    }
}

#[derive(Debug, Default)]
pub struct Shapes<'a>(Vec<Box<Shape + 'a>>);

impl<'a> Shapes<'a> {
    pub fn new() -> Shapes<'a> {
        let v = Vec::new();
        Shapes(v)
    }

    pub fn add<T: Shape + 'a>(&mut self, shape: T) {
        self.0.push(Box::new(shape));
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Shape for Shapes<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.0.iter().fold(None, |acc, s| {
            s.hit(r, t_min, acc.map(|r| r.distance).unwrap_or(t_max))
                .or(acc)
        })
    }
}
