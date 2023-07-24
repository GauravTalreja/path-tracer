use super::impl_prelude::*;

pub struct Cuboid {
    minimum: DVec3,
    maximum: DVec3,
    sides: [Rectangle; 6],
}

impl Cuboid {
    pub fn new(minimum: DVec3, maximum: DVec3, material: Arc<dyn Material>) -> Self {
        let sides = [
            Rectangle::new(
                minimum.x,
                maximum.x,
                minimum.y,
                maximum.y,
                maximum.z,
                material.clone(),
                Plane::XY,
            ),
            Rectangle::new(
                minimum.x,
                maximum.x,
                minimum.y,
                maximum.y,
                minimum.z,
                material.clone(),
                Plane::XY,
            ),
            Rectangle::new(
                minimum.z,
                maximum.z,
                minimum.x,
                maximum.x,
                maximum.y,
                material.clone(),
                Plane::ZX,
            ),
            Rectangle::new(
                minimum.z,
                maximum.z,
                minimum.x,
                maximum.x,
                maximum.y,
                material.clone(),
                Plane::ZX,
            ),
            Rectangle::new(
                minimum.y,
                maximum.y,
                minimum.z,
                maximum.z,
                maximum.x,
                material.clone(),
                Plane::YZ,
            ),
            Rectangle::new(
                minimum.y,
                maximum.y,
                minimum.z,
                maximum.z,
                maximum.x,
                material,
                Plane::YZ,
            ),
        ];
        Self {
            minimum,
            maximum,
            sides,
        }
    }
}

impl Bounded for Cuboid {
    fn bounding_box(&self, _time_min: f64, _time_maxx: f64) -> BoundingBox {
        BoundingBox {
            minimum: self.minimum,
            maximum: self.maximum,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        self.sides
            .iter()
            .filter_map(|h| h.hit(ray, time_min, time_max))
            .min_by(|x, y| x.time.total_cmp(&y.time))
    }
}
