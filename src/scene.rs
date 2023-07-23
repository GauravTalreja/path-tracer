use super::{
    color::Color,
    hittable::Hittable,
    material::Scatter,
    ray::{HitResult, Ray},
    rng::RandomNumberGenerator,
};
pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    fn closest_hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        self.hittables
            .iter()
            .filter_map(|h| h.hit(ray, time_min, time_max))
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
    }

    pub fn color(
        &self,
        ray: &Ray,
        depth: u64,
        rng: &RandomNumberGenerator,
        time_min: f64,
        time_max: f64,
    ) -> Color {
        if depth == 0 {
            return Color::ZERO;
        }
        match self.closest_hit(ray, time_min, time_max) {
            Some(hit_result) => {
                let material = hit_result.material.upgrade().unwrap();
                match material.scatter(ray, &hit_result, rng) {
                    Some(Scatter { ray, attenuation }) => {
                        attenuation * self.color(&ray, depth - 1, rng, time_min, time_max)
                    }
                    None => Color::ZERO,
                }
            }
            None => {
                let unit_direction = ray.direction().normalize();
                let t = 0.5 * (-unit_direction.y + 1.0);
                (1.0 - t) * Color::splat(1.) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
