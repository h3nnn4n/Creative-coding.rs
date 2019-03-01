use random_color::{Color, Luminosity, RandomColor};

pub fn random_rgb_color() -> (f32, f32, f32) {
    let color = RandomColor::new().to_rgb_array();
    let r = color[0] as f32;
    let g = color[1] as f32;
    let b = color[2] as f32;

    (r / 255.0, g / 255.0, b / 255.0)
}
