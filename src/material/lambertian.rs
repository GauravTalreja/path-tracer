use super::impl_prelude::*;

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: &Ray,
        HitResult { normal, point, .. }: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        let direction = rng.in_hemishphere(normal);
        let ray = Ray::new(*point, direction, ray.time_min(), ray.time_max());
        let attenuation = self.albedo;
        Some(Scatter { ray, attenuation })
    }
}
