mod lambertian;
mod metal;
pub mod prelude {
    pub use super::lambertian::Lambertian;
    pub use super::metal::Metal;
}

use super::{
    color::Color,
    ray::{HitResult, Ray},
    rng::RandomNumberGenerator,
};

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
