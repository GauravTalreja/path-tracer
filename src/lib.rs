mod camera;
mod color;
mod hittable;
mod material;
mod ray;
mod render;
mod rng;
mod scene;

pub mod prelude {
    pub use super::color::Color;
    pub use super::hittable::prelude as hittable;
    pub use super::material::prelude as material;
    pub use super::render::Render;
    pub use super::scene::Scene;
    pub use glam::DVec3;
    pub use std::sync::Arc;
}
