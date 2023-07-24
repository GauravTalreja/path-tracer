use super::impl_prelude::*;

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(color: Color) -> Self {
        Self {
            emit: Arc::new(SolidColor::new(color)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray: &Ray,
        _hit_result: &HitResult,
        _rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        None
    }

    fn emitted(&self, point: &DVec3, u: &f64, v: &f64) -> Color {
        self.emit.color(point, u, v)
    }
}
