use rayon::{
    prelude::{IntoParallelRefIterator, ParallelIterator},
    slice::ParallelSliceMut,
};

use crate::{
    hittable::Hittable,
    ray::{HitResult, Ray},
};

pub struct Scene {
    pub hittables: Vec<Box<dyn Hittable>>,
}

impl Scene {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        let mut closest = None;
        let hit_results = self
            .hittables
            .par_iter()
            .map(|h| h.hit(ray))
            .for_each(|hit| {
                if let Some(HitResult {
                    time: time_closest, ..
                }) = closest
                {
                    if let Some(HitResult { time, .. }) = hit {
                        if time < time_closest {
                            closest = hit;
                        }
                    }
                } else {
                    closest = hit;
                }
            });
        closest
    }
}
