use image::Rgb;

pub type Color = glam::DVec3;

pub fn to_rgb(Color { x: r, y: g, z: b }: &Color) -> Rgb<u8> {
    let r = r.sqrt();
    let g = g.sqrt();
    let b = b.sqrt();
    let r = (256. * r.clamp(0., 1.)) as u8;
    let g = (256. * g.clamp(0., 1.)) as u8;
    let b = (256. * b.clamp(0., 1.)) as u8;
    Rgb([r, g, b])
}
