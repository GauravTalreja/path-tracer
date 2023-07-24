mod impl_prelude {
    pub use crate::hittable::*;
    pub use crate::material::Material;
    pub use crate::ray::{HitResult, Ray};
    pub use glam::DVec3;
    pub use std::sync::Arc;
}
use impl_prelude::*;
use rand::{thread_rng, Rng};

pub trait Bounded {
    fn bounding_box(&self, time_min: f64, time_max: f64) -> BoundingBox;
}

pub trait Hittable: Bounded + Send + Sync {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult>;
}

mod sphere;
pub use sphere::Sphere;

mod rectangle;
pub use rectangle::Plane;
pub use rectangle::Rectangle;

#[derive(Clone)]
pub struct BoundingBox {
    pub minimum: DVec3,
    pub maximum: DVec3,
}

impl BoundingBox {
    pub fn surrounding(bounding_boxes: &[&Self]) -> BoundingBox {
        let (minimum, maximum) = bounding_boxes.iter().fold(
            (DVec3::splat(f64::INFINITY), DVec3::splat(f64::NEG_INFINITY)),
            |(minimum_acc, maximum_acc), BoundingBox { minimum, maximum }| {
                (minimum_acc.min(*minimum), maximum_acc.max(*maximum))
            },
        );
        assert!(minimum.is_finite() && maximum.is_finite());
        BoundingBox { minimum, maximum }
    }

    /* Alexander MajeArcik, Cyril Crassin, Peter Shirley, and Morgan McGuire, A Ray-Box Intersection Algorithm and Efficient Dynamic Voxel Rendering, Journal of Computer Graphics Techniques (JCGT), vol. 7, no. 3, 66-81, 2018
    Available online http://jcgt.org/published/0007/03/04/ */
    pub fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> bool {
        let ray_origin = *ray.origin();
        let inv_ray_direction = ray.direction().recip();
        let t0 = (self.minimum - ray_origin) * inv_ray_direction;
        let t1 = (self.maximum - ray_origin) * inv_ray_direction;
        let t_min = t0.min(t1);
        let t_max = t0.max(t1);
        let time_min = time_min.max(t_min.max_element());
        let time_max = time_max.max(t_max.min_element());
        time_min < time_max
    }
}

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: BoundingBox,
}

impl BvhNode {
    pub fn new(
        hittables: &mut [(Arc<dyn Hittable>, BoundingBox)],
        time_min: f64,
        time_max: f64,
    ) -> Self {
        match hittables.len() {
            1 => {
                let left = hittables[0].0.clone();
                let right = left.clone();
                let bounding_box = hittables[0].1.clone();
                BvhNode {
                    left,
                    right,
                    bounding_box,
                }
            }
            2 => {
                let left = hittables[0].0.clone();
                let right = hittables[1].0.clone();
                let bounding_box = BoundingBox::surrounding(&[&hittables[0].1, &hittables[1].1]);
                BvhNode {
                    left,
                    right,
                    bounding_box,
                }
            }
            _ => {
                match thread_rng().gen_range(0..=2) {
                    0 => hittables.sort_unstable_by(|(_, box0), (_, box1)| {
                        box0.minimum.x.total_cmp(&box1.minimum.x)
                    }),
                    1 => hittables.sort_unstable_by(|(_, box0), (_, box1)| {
                        box0.minimum.y.total_cmp(&box1.minimum.y)
                    }),
                    _ => hittables.sort_unstable_by(|(_, box0), (_, box1)| {
                        box0.minimum.z.total_cmp(&box1.minimum.z)
                    }),
                };
                let (left, right) = hittables.split_at_mut(hittables.len() / 2);
                let left = Arc::new(BvhNode::new(left, time_min, time_max));
                let right = Arc::new(BvhNode::new(right, time_min, time_max));
                let bounding_box = BoundingBox::surrounding(&[
                    &left.as_ref().bounding_box,
                    &right.as_ref().bounding_box,
                ]);
                BvhNode {
                    left,
                    right,
                    bounding_box,
                }
            }
        }
    }
}

impl Bounded for BvhNode {
    fn bounding_box(&self, _time_min: f64, _time_max: f64) -> BoundingBox {
        self.bounding_box.clone()
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        if self.bounding_box.hit(ray, time_min, time_max) {
            let left = self.left.hit(ray, time_min, time_max);
            let right = match left {
                Some(HitResult { time, .. }) => self.right.hit(ray, time_min, time),
                None => self.right.hit(ray, time_min, time_max),
            };
            return right.or_else(|| left);
        }
        None
    }
}
