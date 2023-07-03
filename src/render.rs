use super::{camera::Camera, scene::Scene};
use glam::DVec3;
use image::{Rgb, RgbImage};
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use rayon::prelude::*;

pub struct Render {
    width: u32,
    height: u32,
    samples: u64,
    camera: Camera,
    scene: Scene,
    rng: Uniform<f64>,
}

impl Render {
    pub fn new(width: u32, height: u32, samples_per_pixel: u64, scene: Scene) -> Self {
        let viewport_height = 2.0;
        let aspect_ratio = width as f64 / height as f64;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let camera = Camera::new(viewport_height, viewport_width, focal_length);
        let rng = Uniform::new(0., 1.);
        Render {
            width,
            height,
            samples: samples_per_pixel,
            camera,
            scene,
            rng,
        }
    }

    fn get_color(&self, x: u32, y: u32) -> Rgb<u8> {
        to_color(
            &((0..self.samples)
                .into_par_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    let u = (x as f64 + self.rng.sample(&mut rng)) / (self.width - 1) as f64;
                    let v = (y as f64 + self.rng.sample(&mut rng)) / (self.height - 1) as f64;
                    let r = self.camera.get_ray(u, v);
                    self.scene.color(&r)
                })
                .sum::<DVec3>()
                / self.samples as f64),
        )
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
    let r = (256. * x.clamp(0., 1.)) as u8;
    let g = (256. * y.clamp(0., 1.)) as u8;
    let b = (256. * z.clamp(0., 1.)) as u8;
    Rgb([r, g, b])
}
