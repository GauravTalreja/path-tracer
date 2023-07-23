use std::sync::Arc;

use super::impl_prelude::*;

pub struct Checker {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl Texture for Checker {
    fn color(&self, point: &DVec3, u: &f64, v: &f64) -> Color {
        match (10. * point.x).sin() * (10. * point.y).sin() * (10. * point.z).sin() < 0. {
            true => self.odd.color(point, u, v),
            false => self.even.color(point, u, v),
        }
    }
}
