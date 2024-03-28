use std::sync::Arc;

use super::prelude::*;

pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for Checker {
    fn color(&self, point: &Vec3A, u: &f32, v: &f32) -> Color {
        match (10. * point.x).sin() * (10. * point.y).sin() * (10. * point.z).sin() < 0. {
            true => self.odd.color(point, u, v),
            false => self.even.color(point, u, v),
        }
    }
}
