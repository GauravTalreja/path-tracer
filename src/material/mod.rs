mod impl_prelude {
    pub use super::{reflect, refract, Material, Scatter};
    pub use crate::{
        color::Color,
        ray::{HitResult, Ray},
        rng::RandomNumberGenerator,
        texture::*,
    };
    pub use glam::DVec3;
    pub use std::sync::Arc;
}
use impl_prelude::*;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray: &Ray,
        hit_result: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter>;

    fn emitted(&self, _point: &DVec3, _u: &f64, _v: &f64) -> Color {
        Color::ZERO
    }
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

mod dielectric;
pub use dielectric::Dielectric;

mod lambertian;
pub use lambertian::Lambertian;

mod metal;
pub use metal::Metal;

mod diffuse_light;
pub use diffuse_light::DiffuseLight;

pub fn refract(uv: DVec3, normal: DVec3, refraction_ratio: f64) -> DVec3 {
    let cos_theta = (-uv).dot(normal).min(1.);
    let perpendicular = refraction_ratio * (uv + cos_theta * normal);
    let parallel = -(1. - perpendicular.length_squared()).abs().sqrt() * normal;
    perpendicular + parallel
}

pub fn reflect(v: DVec3, normal: DVec3) -> DVec3 {
    v - 2. * v.dot(normal) * normal
}
