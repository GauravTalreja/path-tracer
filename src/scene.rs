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
    background: Color,
}

impl Scene {
    pub fn new(
        hittables: &[Arc<dyn Hittable>],
        time_min: f32,
        time_max: f32,
        background: Color,
    ) -> Self {
        let mut hittables: Vec<(Arc<dyn Hittable>, BoundingBox)> = hittables
            .iter()
            .map(|h| (h.clone(), h.as_ref().bounding_box(time_min, time_max)))
            .collect();
        Self {
            bvh: BvhNode::new(&mut hittables, time_min, time_max),
            background,
        }
    }

    fn closest_hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitResult> {
        self.bvh.hit(ray, time_min, time_max)
    }

    pub fn color(
        &self,
        ray: &Ray,
        depth: u64,
        rng: &RandomNumberGenerator,
        time_min: f32,
        time_max: f32,
    ) -> Color {
        if depth == 0 {
            return Color::ZERO;
        }
        match self.closest_hit(ray, time_min, time_max) {
            Some(hit_result) => {
                let material = hit_result.material.upgrade().unwrap();
                let HitResult { u, v, point, .. } = hit_result;
                let emitted = material.emitted(&point, &u, &v);
                match material.scatter(ray, &hit_result, rng) {
                    Some(Scatter { ray, attenuation }) => {
                        emitted + attenuation * self.color(&ray, depth - 1, rng, time_min, time_max)
                    }
                    None => emitted,
                }
            }
            None => self.background,
        }
    }
}
