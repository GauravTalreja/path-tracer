use super::{ray::Ray, rng::RandomNumberGenerator};
use crate::Vec3A;
use rand::distributions::Uniform;
use rand::prelude::Distribution;
use rand::thread_rng;

#[derive(Copy, Clone)]
pub struct Camera {
    look_from: Vec3A,
    pub viewport_u: Vec3A,
    pub viewport_v: Vec3A,
    pub viewport_upper_left: Vec3A,
    defocus_angle: f32,
    defocus_disk_u: Vec3A,
    defocus_disk_v: Vec3A,
    time: Uniform<f32>
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        up: Vec3A,

        fov: f32,
        aspect_ratio: f32,

        defocus_angle: f32,
        focus_distance: f32,

        time_min: f32,
        time_max: f32,
    ) -> Self {
        let theta = fov.to_radians() / 2.;
        let h = theta.tan();
        let viewport_height = 2. * h * focus_distance;
        let viewport_width =  viewport_height * aspect_ratio;

        let w = (look_from - look_at).normalize();
        let u = up.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let viewport_upper_left = look_from - (focus_distance * w) - viewport_u / 2. - viewport_v / 2.;

        let defocus_radius = focus_distance * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let time = Uniform::new(time_min, time_max);

        Self {
            look_from,
            viewport_u,
            viewport_v,
            viewport_upper_left,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            time
        }
    }

    pub fn get_ray(&self, viewport_pos: Vec3A, rng: &RandomNumberGenerator) -> Ray {
        let origin = if self.defocus_angle <= 0. {
            self.look_from
        } else {
            let p = rng.in_unit_disk();
            self.look_from + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
        };
        let direction = viewport_pos - origin;
        let time = self.time();
        Ray::new(
            origin,
            direction,
            time
        )
    }

    pub fn time(&self) -> f32 {
            let mut rng = thread_rng();
            self.time.sample(&mut rng)
    }
}
