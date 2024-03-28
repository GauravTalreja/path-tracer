use super::prelude::*;

pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColor {
    fn color(&self, _point: &Vec3A, _u: &f32, _v: &f32) -> Color {
        self.color
    }
}
