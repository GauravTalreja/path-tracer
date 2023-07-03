use super::material::Material;
use glam::DVec3;
use std::sync::Weak;

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
    time_min: f64,
    time_max: f64,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3, time_min: f64, time_max: f64) -> Self {
        Self {
            origin,
            direction,
            time_min,
            time_max,
        }
    }

    pub fn origin(&self) -> &DVec3 {
        &self.origin
    }

    pub fn direction(&self) -> &DVec3 {
        &self.direction
    }

    pub fn time_min(&self) -> f64 {
        self.time_min
    }

    pub fn time_max(&self) -> f64 {
        self.time_max
    }

    pub fn at_unchecked(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }

    pub fn exists_at(&self, time: f64) -> bool {
        self.time_min <= time && time <= self.time_max
    }
}

pub struct HitResult {
    pub normal: DVec3,
    pub time: f64,
    pub point: DVec3,
    pub material: Weak<dyn Material>,
}
