use glam::DVec3;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use super::{
    hittable::Hittable,
    ray::{HitResult, Ray},
};

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        self.hittables
            .par_iter()
            .filter_map(|h| h.hit(ray))
            .min_by(|x, y| x.time.partial_cmp(&y.time).unwrap())
    }

    pub fn color(&self, ray: &Ray) -> DVec3 {
        if let Some(HitResult { normal, .. }) = self.hit(ray) {
            return 0.5
                * DVec3 {
                    x: 1. + normal.x,
                    y: 1. - normal.y,
                    z: 1. + normal.z,
                };
        }
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (-unit_direction.y + 1.0);
        (1.0 - t) * DVec3::splat(1.)
            + t * DVec3 {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            }
    }
}
