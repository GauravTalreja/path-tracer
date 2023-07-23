use super::impl_prelude::*;

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _point: &DVec3, _u: &f64, _v: &f64) -> Color {
        self.color
    }
}
