use glam::DVec3;
use image::Rgb;

pub struct Ray {
    pub origin: DVec3,
    pub direction: DVec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Rgb<u8> {
        let unit_direction = self.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        crate::to_color(
            &(t * DVec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            } + (1.0 - t)
                * DVec3 {
                    x: 0.5,
                    y: 0.7,
                    z: 1.0,
                }),
        )
    }
}

pub struct HitResult {
    pub normal: DVec3,
}
