use glam::DVec3;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

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
    pub uniform_213_343: Uniform<f64>,
    pub uniform_227_332: Uniform<f64>,
}

impl Scene {
    pub fn new(
        hittables: &[Arc<dyn Hittable>],
        time_min: f64,
        time_max: f64,
        background: Color,
    ) -> Self {
        let mut hittables: Vec<(Arc<dyn Hittable>, BoundingBox)> = hittables
            .iter()
            .map(|h| (h.clone(), h.as_ref().bounding_box(time_min, time_max)))
            .collect();
        Self {
            bvh: BvhNode::new(&mut hittables, time_min, time_max),
            background,
            uniform_213_343: Uniform::new_inclusive(213., 343.),
            uniform_227_332: Uniform::new_inclusive(227., 332.),
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
                let HitResult { u, v, point, .. } = hit_result;
                let emitted = material.emitted(&point, &u, &v);
                let scatter = material.scatter(ray, &hit_result, rng);
                match scatter {
                    Some(Scatter {
                        ray,
                        attenuation,
                        albedo,
                        pdf,
                    }) => {
                        let thread_rng = &mut thread_rng();
                        let on_light = DVec3::new(
                            self.uniform_213_343.sample(thread_rng),
                            554.,
                            self.uniform_227_332.sample(thread_rng),
                        );
                        let to_light = on_light - hit_result.point;
                        let distance_squared = to_light.length_squared();
                        let to_light = to_light.normalize();
                        if to_light.dot(hit_result.normal) < 0. {
                            return emitted;
                        }
                        let light_area = (343 - 213) * (332 - 227);
                        let light_cos = to_light.y.abs();
                        if light_cos < 0.000001 {
                            return emitted;
                        }
                        let pdf = distance_squared / (light_cos * light_area as f64);
                        let ray = Ray::new(hit_result.point, to_light, ray.time());

                        emitted
                            + albedo
                                * material.scattering_pdf(&ray, &hit_result, &scatter.unwrap())
                                * self.color(&ray, depth - 1, rng, time_min, time_max)
                                / pdf
                    }
                    None => emitted,
                }
            }
            None => self.background,
        }
    }
}
