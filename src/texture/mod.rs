mod prelude {
    pub use super::Texture;
    pub use crate::color::Color;
    pub use crate::Vec3A;
}
use prelude::*;

pub trait Texture: Send + Sync {
    fn color(&self, point: &Vec3A, u: f32, v: f32) -> Color;
}

mod solid_color;
pub use solid_color::SolidColor;

mod checker;
pub use checker::Checker;

mod image;
pub use self::image::Image;
