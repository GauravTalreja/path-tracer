use super::ray::Ray;
use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64) -> Self {
        let origin = DVec3::ZERO;
        let horizontal = DVec3::new(viewport_width, 0., 0.);
        let vertical = DVec3::new(0., viewport_height, 0.);
        let lower_left_corner =
            origin - (horizontal + vertical) / 2. - DVec3::new(0., 0., focal_length);
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
