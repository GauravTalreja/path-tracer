use super::prelude::*;

pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(path: &str) -> Self {
        let image = image::open(path)
            .expect(&format!("{path} not found"))
            .to_rgb8();
        let (width, height) = image.dimensions();
        let data = image.into_raw();
        Self {
            data,
            width,
            height,
        }
    }
}

impl Texture for Image {
    fn color(&self, _point: &Vec3A, u: &f32, v: &f32) -> Color {
        let (width, height) = (self.width as usize, self.height as usize);
        let u = u.clamp(0., 1.);
        let v = 1. - v.clamp(0., 1.);
        let i = (u * width as f32) as usize;
        let j = (v * height as f32) as usize;
        let i = i.clamp(0, width - 1);
        let j = j.clamp(0, height - 1);
        let idx = 3 * i + 3 * self.width as usize * j;
        let a: [f32; 3] = self.data[idx..idx + 3]
            .iter()
            .map(|p| *p as f32)
            .collect::<Vec<f32>>()
            .try_into()
            .unwrap();
        Color::from_array(a) / 255.
    }
}
