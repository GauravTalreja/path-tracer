use crate::{camera::Camera, ray::Ray, scene::Scene};
use image::{Rgb, RgbImage};
use rayon::prelude::{ParallelBridge, ParallelIterator};

pub struct Render {
    width: u32,
    height: u32,
    camera: Camera,
    scene: Scene,
}

impl Render {
    pub fn new(width: u32, height: u32, scene: Scene) -> Self {
        let viewport_height = 2.0;
        let aspect_ratio = width as f64 / height as f64;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let camera = Camera::new(viewport_height, viewport_width, focal_length);
        Render {
            width,
            height,
            camera,
            scene,
        }
    }

    fn get_color(&self, x: u32, y: u32) -> Rgb<u8> {
        let u = x as f64 / (self.width - 1) as f64;
        let v = y as f64 / (self.height - 1) as f64;
        let direction = *self.camera.lower_left_corner()
            + (*self.camera.horizontal()) * u
            + (*self.camera.vertical()) * v
            - *self.camera.origin();
        let origin = *self.camera.origin();
        let r = Ray::new(origin, direction, 0., f64::INFINITY);
        self.scene.color(&r)
    }

    pub fn to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        let _ = image
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| *pixel = self.get_color(x, y));
        image
    }
}
