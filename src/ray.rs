use rand::Rng;

use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    #[rustfmt::skip]
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        v_fov_deg: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = v_fov_deg.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = half_height * aspect;

        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        let lower_left_corner = look_from - u * half_width * focus_dist - v * half_height * focus_dist - w * focus_dist;
        let horizontal = u * half_width * 2. * focus_dist;
        let vertical = v * half_height * 2. * focus_dist;

        Camera {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.,
        }
    }

    fn random_in_unit_disk(rng: &mut impl Rng) -> Vec3 {
        let v = Vec3 {
            x: 1.,
            y: 1.,
            z: 0.,
        };

        let mut p: Vec3;
        while {
            let rnd = Vec3 {
                x: rng.gen(),
                y: rng.gen(),
                z: 0.,
            };
            p = rnd * 2. - v;

            p.dot(p) >= 1.
        } {}

        p
    }

    pub fn ray(&self, s: f32, t: f32, rng: &mut impl Rng) -> Ray {
        let rd = Camera::random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
