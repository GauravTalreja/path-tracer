use super::prelude::*;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn new(color: Color) -> Self {
        Self {
            albedo: Arc::new(SolidColor { color }),
        }
    }

    pub fn new_with_texture(texture: Arc<dyn Texture>) -> Self {
        Self {
            albedo: texture,
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray: &Ray,
        hit_result: &HitResult,
        rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        let scattered_direction = rng.in_unit_sphere().normalize();
        let scattered = Ray::new(hit_result.point, scattered_direction, ray.time());
        let attenuation = self.albedo.color(&hit_result.point, hit_result.u, hit_result.v);
        Some(Scatter {
            ray: scattered,
            attenuation,
        })
    }
}
