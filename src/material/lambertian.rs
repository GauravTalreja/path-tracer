use super::impl_prelude::*;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        HitResult {
            normal,
            point,
            u,
            v,
            ..
        }: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        let direction = rng.in_hemishphere(normal);
        let ray = Ray::new(*point, direction, ray.time());
        let attenuation = self.albedo.color(point, u, v);
        Some(Scatter { ray, attenuation })
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color }),
        }
    }
}
