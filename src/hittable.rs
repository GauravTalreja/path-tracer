mod impl_prelude {
    pub use crate::hittable::*;
    pub use crate::ray::{HitResult, Ray};
    pub use glam::DVec3;
}
use impl_prelude::*;
mod sphere;
pub mod prelude {
    pub use super::sphere::Sphere;
}
pub use sphere::Sphere;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}
