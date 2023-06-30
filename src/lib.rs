use glam::DVec3;
use image::{Rgb, RgbImage};
use rayon::prelude::{ParallelBridge, ParallelIterator};

mod camera;
mod ray;
mod shape;
use camera::Camera;
use ray::{HitResult, Ray};
use shape::{Hittable, Sphere};

pub struct Render {
    width: u32,
    height: u32,
    camera: Camera,
}

impl Render {
    pub fn new(width: u32, height: u32) -> Self {
        let viewport_height = 2.0;
        let aspect_ratio = width as f64 / height as f64;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let camera = Camera::new(viewport_height, viewport_width, focal_length);
        Render {
            width,
            height,
            camera,
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
        let r = Ray { origin, direction };

        let sphere = Sphere::new(
            0.5,
            DVec3 {
                x: 0.,
                y: 0.,
                z: -1.,
            },
        );
        if let Some(HitResult { normal }) = sphere.hit(&r) {
            return to_color(
                &(0.5
                    * DVec3 {
                        x: 1. + normal.x,
                        y: 1. - normal.y,
                        z: 1. + normal.z,
                    }),
            );
        }

        r.color()
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

fn to_color(DVec3 { x, y, z }: &DVec3) -> Rgb<u8> {
    let r = (255.999 * x) as u8;
    let g = (255.999 * y) as u8;
    let b = (255.999 * z) as u8;
    Rgb([r, g, b])
}
