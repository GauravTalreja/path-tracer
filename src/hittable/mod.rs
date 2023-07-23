mod sphere;
pub use sphere::Sphere;

mod impl_prelude {
    pub use crate::hittable::*;
    pub use crate::material::Material;
    pub use crate::ray::{HitResult, Ray};
    pub use glam::DVec3;
    pub use std::sync::Arc;
}
use impl_prelude::*;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult>;
}
