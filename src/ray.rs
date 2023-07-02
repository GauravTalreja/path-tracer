use glam::DVec3;
use image::Rgb;

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
    time_min: f64,
    time_max: f64,
}

impl Ray {
    pub fn origin(&self) -> &DVec3 {
        &self.origin
    }

    pub fn direction(&self) -> &DVec3 {
        &self.direction
    }

    pub fn at_unchecked(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn exists_at(&self, time: f64) -> bool {
        self.time_min <= time && time <= self.time_max
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
    pub time: f64,
}
