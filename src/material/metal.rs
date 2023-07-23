use super::impl_prelude::*;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        HitResult { normal, point, .. }: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        let reflected = reflect(ray.direction().normalize(), *normal);
        let ray = Ray::new(
            *point,
            reflected + self.fuzz * rng.in_unit_sphere(),
            ray.time(),
        );
        match ray.direction().dot(*normal) > 0. {
            true => Some(Scatter {
                ray,
                attenuation: self.albedo,
            }),
            false => None,
        }
    }
}
