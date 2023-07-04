use super::ray::Ray;
use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
}

impl Camera {
    pub fn new(look_from: DVec3, look_at: DVec3, vup: DVec3, vfov: f64, aspect_ratio: f64) -> Self {
        let theta = vfov.to_radians() / 2.;
        let h = theta.tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal + vertical) / 2. - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
            0.001,
            f64::INFINITY,
        )
    }
}
