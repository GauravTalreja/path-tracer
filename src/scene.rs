use super::{
    color::Color,
    hittable::Hittable,
    material::Scatter,
    ray::{HitResult, Ray},
    rng::RandomNumberGenerator,
};
use glam::DVec3;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    fn closest_hit(&self, ray: &Ray) -> Option<HitResult> {
        self.hittables
            .par_iter()
            .filter_map(|h| h.hit(ray))
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
    }

    pub fn color(&self, ray: &Ray, depth: u64, rng: &RandomNumberGenerator) -> Color {
        if depth == 0 {
            return DVec3::ZERO;
        }
        match self.closest_hit(ray) {
            Some(hit_result) => {
                let material = hit_result.material.upgrade().unwrap();
                match material.scatter(ray, &hit_result, rng) {
                    Some(Scatter { ray, attenuation }) => {
                        attenuation * self.color(&ray, depth - 1, rng)
                    }
                    None => Color::ZERO,
                }
            }
            None => {
                let unit_direction = ray.direction().normalize();
                let t = 0.5 * (-unit_direction.y + 1.0);
                (1.0 - t) * DVec3::splat(1.)
                    + t * DVec3 {
                        x: 0.5,
                        y: 0.7,
                        z: 1.0,
                    }
            }
        }
    }
}
