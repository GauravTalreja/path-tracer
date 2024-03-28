use super::prelude::*;

#[derive(Clone)]
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

    fn emitted(&self, _point: &Vec3A, _u: f32, _v: f32) -> Color {
        self.emit.color(_point, _u, _v)
    }
}
