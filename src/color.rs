use crate::{interval::Interval, Vec3};

pub type Color = Vec3;

pub fn write_color(pixel_color: &Color) -> [u32; 3] {
    let intensity: Interval = Interval::new(0.0, 0.999);

    // apply the linear-to-gamma transformation directly with conditional expressions
    let r = if pixel_color.x() > 0.0 {
        pixel_color.x().sqrt()
    } else {
        0.0
    };
    let g = if pixel_color.y() > 0.0 {
        pixel_color.y().sqrt()
    } else {
        0.0
    };
    let b = if pixel_color.z() > 0.0 {
        pixel_color.z().sqrt()
    } else {
        0.0
    };

    // clamp the values to the interval and convert to u32
    let ir: u32 = (256.0 * intensity.clamp(r)) as u32;
    let ig: u32 = (256.0 * intensity.clamp(g)) as u32;
    let ib: u32 = (256.0 * intensity.clamp(b)) as u32;
    [ir, ig, ib]
}
