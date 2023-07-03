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
        let horizontal = DVec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = DVec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal / 2.
            - vertical / 2.
            - DVec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

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
