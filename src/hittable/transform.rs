use super::prelude::*;
use glam::{DQuat, DMat4};

pub struct Transform {
    object: Arc<dyn Hittable>,
    transform: DMat4,
    inverse_transform: DMat4,
}

impl Transform {
    pub fn new(object: Arc<dyn Hittable>, rotation: DQuat, center: DVec3) -> Self {
        let rotation_matrix = DMat4::from_quat(rotation);
        let translation_to_origin = DMat4::from_translation(-center);
        let translation_back = DMat4::from_translation(center);
        let transform = translation_back * rotation_matrix * translation_to_origin;
        let inverse_transform = transform.inverse();

        Transform {
            object,
            transform,
            inverse_transform,
        }
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        let origin = self.inverse_transform.transform_point3(ray.origin());
        let direction = self.inverse_transform.transform_vector3(ray.direction());
        Ray::new(origin, direction, ray.time())
    }
}

impl Hittable for Transform {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        let transformed_ray = self.transform_ray(ray);

        if let Some(mut hit) = self.object.hit(&transformed_ray, time_min, time_max) {
            hit.point = self.transform.transform_point3(hit.point);
            hit.normal = self.transform.transform_vector3(hit.normal).normalize();
            return Some(hit);
        }
        None
    }
}

impl Bounded for Transform {
    fn bounding_box(&self, time_min: f64, time_max: f64) -> BoundingBox {
        let bbox = self.object.bounding_box(time_min, time_max);

        let mut minimum = DVec3::splat(f64::INFINITY);
        let mut maximum = DVec3::splat(f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.maximum.x + (1 - i) as f64 * bbox.minimum.x;
                    let y = j as f64 * bbox.maximum.y + (1 - j) as f64 * bbox.minimum.y;
                    let z = k as f64 * bbox.maximum.z + (1 - k) as f64 * bbox.minimum.z;

                    let new_coords = self.transform.transform_point3(DVec3::new(x, y, z));

                    for c in 0..3 {
                        minimum[c] = f64::min(minimum[c], new_coords[c]);
                        maximum[c] = f64::max(maximum[c], new_coords[c]);
                    }
                }
            }
        }

        BoundingBox { minimum, maximum }
    }
}
