mod camera;
mod color;
mod ray;
mod render;
mod rng;
mod scene;

pub use camera::Camera;
pub use color::{hex_color, Color};

pub use glam::{Quat, Vec3A};
pub use render::Render;
pub use rng::RandomNumberGenerator;
pub use scene::Scene;
pub use std::sync::Arc;

pub mod hittable;
pub mod material;
pub mod random_scene;
pub mod texture;
