use super::prelude::*;
use glam::{Quat, Mat4, Vec3};

pub struct Transform {
    object: Arc<dyn Hittable>,
    transform: Mat4,
    inverse_transform: Mat4,
}

impl Transform {
    pub fn new(object: Arc<dyn Hittable>, rotation: Quat, center: DVec3) -> Self {
        let rotation_matrix = Mat4::from_quat(rotation);
        let translation_to_origin = Mat4::from_translation(Vec3::from(-center));
        let translation_back = Mat4::from_translation(Vec3::from(center));
        let transform = translation_back * rotation_matrix * translation_to_origin;
        let inverse_transform = transform.inverse();

        Transform {
            object,
            transform,
            inverse_transform,
        }
    }

    fn transform_ray(&self, ray: &Ray) -> Ray {
        let origin = self.inverse_transform.transform_point3(Vec3::from(ray.origin()));
        let direction = self.inverse_transform.transform_vector3(Vec3::from(ray.direction()));
        Ray::new(DVec3::from(origin), DVec3::from(direction), ray.time())
    }
}

impl Hittable for Transform {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitResult> {
        let transformed_ray = self.transform_ray(ray);

        if let Some(mut hit) = self.object.hit(&transformed_ray, time_min, time_max) {
            hit.point = DVec3::from(self.transform.transform_point3(Vec3::from(hit.point)));
            hit.normal = DVec3::from(self.transform.transform_vector3(Vec3::from(hit.normal)).normalize());
            return Some(hit);
        }
        None
    }
}

impl Bounded for Transform {
    fn bounding_box(&self, time_min: f32, time_max: f32) -> BoundingBox {
        let bbox = self.object.bounding_box(time_min, time_max);

        let mut minimum = DVec3::splat(f32::INFINITY);
        let mut maximum = DVec3::splat(f32::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f32 * bbox.maximum.x + (1 - i) as f32 * bbox.minimum.x;
                    let y = j as f32 * bbox.maximum.y + (1 - j) as f32 * bbox.minimum.y;
                    let z = k as f32 * bbox.maximum.z + (1 - k) as f32 * bbox.minimum.z;

                    let new_coords = self.transform.transform_point3(Vec3::from(DVec3::new(x, y, z)));

                    for c in 0..3 {
                        minimum[c] = f32::min(minimum[c], new_coords[c]);
                        maximum[c] = f32::max(maximum[c], new_coords[c]);
                    }
                }
            }
        }

        BoundingBox { minimum, maximum }
    }
}
