use image::Rgb;
pub type Color = glam::Vec3;

pub fn hex_color(x: u32) -> Color {
    let r = ((x >> 16) & 0xff) as f32 / 255.0;
    let g = ((x >> 8) & 0xff) as f32 / 255.0;
    let b = (x & 0xff) as f32 / 255.0;
    Color::new(r.powi(2), g.powi(2), b.powi(2))
}

pub fn to_rgb(Color { x, y, z }: &Color) -> Rgb<u8> {
    let r = x.sqrt();
    let g = y.sqrt();
    let b = z.sqrt();
    let r = (256. * r.clamp(0., 1.)) as u8;
    let g = (256. * g.clamp(0., 1.)) as u8;
    let b = (256. * b.clamp(0., 1.)) as u8;
    Rgb([r, g, b])
}
