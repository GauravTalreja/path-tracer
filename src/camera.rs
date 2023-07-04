use super::{ray::Ray, rng::RandomNumberGenerator};
use glam::DVec3;

pub struct Camera {
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
    u: DVec3,
    v: DVec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        look_from: DVec3,
        look_at: DVec3,
        vup: DVec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vfov.to_radians() / 2.;
        let h = theta.tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - (horizontal + vertical) / 2. - focus_distance * w;
        let lens_radius = aperture / 2.;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &RandomNumberGenerator) -> Ray {
        let random = self.lens_radius * rng.in_unit_disk();
        let offset = self.u * random.x + self.v * random.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            0.001,
            f64::INFINITY,
        )
    }
}
