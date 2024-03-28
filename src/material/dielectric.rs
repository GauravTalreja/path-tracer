use super::prelude::*;
use rand::{prelude::Distribution, thread_rng};

pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        HitResult { normal, point, .. }: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        let attenuation = Color::splat(1.);
        let front_face = ray.direction().dot(*normal) < 0.;
        let normal = if front_face { *normal } else { -*normal };
        let refraction_ratio = if front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };
        let direction = ray.direction().normalize();
        let cos_theta = (-direction).dot(normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let mut thread_rng = thread_rng();
        let direction = if refraction_ratio * sin_theta > 1.
            || reflectance(cos_theta, refraction_ratio) > rng.uniform_0_1.sample(&mut thread_rng)
        {
            reflect(direction, normal)
        } else {
            refract(direction, normal, refraction_ratio)
        };
        let ray = Ray::new(*point, direction, ray.time());
        Some(Scatter { ray, attenuation })
    }
}

fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
    let r0 = (1. - refraction_ratio) / (1. + refraction_ratio);
    let r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cos_theta).powi(5)
}
