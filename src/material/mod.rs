mod dielectric;
mod lambertian;
mod metal;
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

mod impl_prelude {
    pub use super::{reflect, refract, Material, Scatter};
    pub use crate::{
        color::Color,
        ray::{HitResult, Ray},
        rng::RandomNumberGenerator,
    };
    pub use glam::DVec3;
}
use impl_prelude::*;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray: &Ray,
        hit_result: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter>;
}

pub fn refract(uv: DVec3, normal: DVec3, refraction_ratio: f64) -> DVec3 {
    let cos_theta = (-uv).dot(normal).min(1.);
    let perpendicular = refraction_ratio * (uv + cos_theta * normal);
    let parallel = -(1. - perpendicular.length_squared()).abs().sqrt() * normal;
    perpendicular + parallel
}

pub fn reflect(v: DVec3, normal: DVec3) -> DVec3 {
    v - 2. * v.dot(normal) * normal
}
