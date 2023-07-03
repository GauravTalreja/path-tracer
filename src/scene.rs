use glam::DVec3;
use image::Rgb;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{
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

    pub fn color(&self, ray: &Ray) -> Rgb<u8> {
        if let Some(HitResult { normal, .. }) = self.hit(ray) {
            return to_color(
                &(0.5
                    * DVec3 {
                        x: 1. + normal.x,
                        y: 1. - normal.y,
                        z: 1. + normal.z,
                    }),
            );
        }
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (-unit_direction.y + 1.0);
        to_color(
            &((1.0 - t) * DVec3::splat(1.)
                + t * DVec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                }),
        )
    }
}

fn to_color(DVec3 { x, y, z }: &DVec3) -> Rgb<u8> {
    let r = (256. * x.clamp(0., 1.)) as u8;
    let g = (256. * y.clamp(0., 1.)) as u8;
    let b = (256. * z.clamp(0., 1.)) as u8;
    Rgb([r, g, b])
}
