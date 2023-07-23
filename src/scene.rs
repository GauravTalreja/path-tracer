use super::{
    color::Color,
    hittable::{BoundingBox, BvhNode, Hittable},
    material::Scatter,
    ray::{HitResult, Ray},
    rng::RandomNumberGenerator,
};
use std::sync::Arc;
pub struct Scene {
    bvh: BvhNode,
}

impl Scene {
    pub fn new(hittables: &[Arc<dyn Hittable>], time_min: f64, time_max: f64) -> Self {
        let mut hittables: Vec<(Arc<dyn Hittable>, BoundingBox)> = hittables
            .iter()
            .map(|h| (h.clone(), h.as_ref().bounding_box(time_min, time_max)))
            .collect();
        Self {
            bvh: BvhNode::new(&mut hittables, time_min, time_max),
        }
    }

    fn closest_hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        self.bvh.hit(ray, time_min, time_max)
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
