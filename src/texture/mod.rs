mod impl_prelude {
    pub use super::Texture;
    pub use crate::color::Color;
    pub use glam::DVec3;
}
use impl_prelude::*;

pub trait Texture: Send + Sync {
    fn color(&self, point: &DVec3, u: &f64, v: &f64) -> Color;
}

mod solid_color;
pub use solid_color::SolidColor;

mod checker;
pub use checker::Checker;
