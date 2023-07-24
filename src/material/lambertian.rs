use super::impl_prelude::*;
use std::f64::consts::PI;

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
        let pdf = 0.5 / PI;
        Some(Scatter {
            ray,
            attenuation,
            albedo: attenuation,
            pdf,
        })
    }

    fn scattering_pdf(&self, _ray: &Ray, hit_result: &HitResult, scatter: &Scatter) -> f64 {
        let cos = hit_result.normal.dot(scatter.ray.direction().normalize());
        match cos < 0. {
            true => 0.,
            false => cos / PI,
        }
    }
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color }),
        }
    }
}
