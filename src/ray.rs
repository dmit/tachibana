use crate::vec::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    #[rustfmt::skip]
    pub fn new(look_from: Vec3, look_at: Vec3, view_up: Vec3, v_fov_deg: f64, aspect: f64) -> Self {
        let theta = v_fov_deg.to_radians();
        let half_height = (theta / 2.).tan();
        let half_width = half_height * aspect;

        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from - u * half_width - v * half_height - w,
            horizontal: u * half_width * 2.,
            vertical: v * half_height * 2.,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin,
        }
    }
}
