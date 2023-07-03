mod camera;
pub mod hittable;
mod ray;
mod render;
mod rng;
mod scene;

pub use render::Render;
pub use scene::Scene;
pub mod prelude {
    pub use super::hittable::prelude::*;
    pub use super::render::Render;
    pub use super::scene::Scene;
    pub use glam::DVec3;
}
