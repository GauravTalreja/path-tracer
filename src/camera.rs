use std::ops::Div;

use glam::DVec3;

pub struct Camera {
    height: f64,
    width: f64,
    focal_length: f64,
    origin: DVec3,
    horizontal: DVec3,
    vertical: DVec3,
    lower_left_corner: DVec3,
}

impl Camera {
    pub fn new(viewport_height: f64, viewport_width: f64, focal_length: f64) -> Self {
        let height = viewport_height;
        let width = viewport_width;
        let origin = DVec3::ZERO;
        let horizontal = DVec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = DVec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };
        let lower_left_corner = origin
            - horizontal.div(2.0)
            - vertical.div(2.0)
            - DVec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            };

        Self {
            height,
            width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn origin(&self) -> &DVec3 {
        &self.origin
    }

    pub fn horizontal(&self) -> &DVec3 {
        &self.horizontal
    }

    pub fn vertical(&self) -> &DVec3 {
        &self.vertical
    }

    pub fn lower_left_corner(&self) -> &DVec3 {
        &self.lower_left_corner
    }
}
