use super::material::Material;
use glam::DVec3;
use std::sync::Weak;

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> DVec3 { self.origin }

    pub fn direction(&self) -> DVec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}

pub struct HitResult {
    pub normal: DVec3,
    pub time: f64,
    pub u: f64,
    pub v: f64,
    pub point: DVec3,
    pub material: Weak<dyn Material>,
}
