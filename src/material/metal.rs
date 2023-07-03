use glam::DVec3;

use super::{Material, Scatter};
use crate::{
    color::Color,
    ray::{HitResult, Ray},
    rng::RandomNumberGenerator,
};

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        HitResult { normal, point, .. }: &HitResult,
        _rng: &RandomNumberGenerator,
    ) -> Option<Scatter> {
        let reflected = reflect(ray.direction().normalize(), *normal);
        let ray = Ray::new(
            *point,
            reflected + self.fuzz * _rng.in_unit_sphere(),
            ray.time_min(),
            ray.time_max(),
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

fn reflect(v: DVec3, normal: DVec3) -> DVec3 {
    v - 2. * v.dot(normal) * normal
}
