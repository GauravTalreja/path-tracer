mod prelude {
    pub use super::{reflect, refract, Material, Scatter};
    pub use crate::{
        color::Color,
        ray::{HitResult, Ray},
        rng::RandomNumberGenerator,
        texture::*,
    };
    pub use crate::Vec3A;
    pub use std::sync::Arc;
}
use prelude::*;

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        ray: &Ray,
        hit_result: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter>;

    fn emitted(&self, _point: &Vec3A, _u: f32, _v: f32) -> Color {
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

mod isotropic;
pub use isotropic::Isotropic;

pub use diffuse_light::DiffuseLight;

pub fn refract(uv: Vec3A, normal: Vec3A, refraction_ratio: f32) -> Vec3A {
    let cos_theta = (-uv).dot(normal).min(1.);
    let perpendicular = refraction_ratio * (uv + cos_theta * normal);
    let parallel = -(1. - perpendicular.length_squared()).abs().sqrt() * normal;
    perpendicular + parallel
}

pub fn reflect(v: Vec3A, normal: Vec3A) -> Vec3A {
    v - 2. * v.dot(normal) * normal
}
