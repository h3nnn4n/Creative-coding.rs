pub fn rgb_array_to_tuple(color_vector: [u32; 3]) -> (f32, f32, f32) {
    let r = color_vector[0] as f32;
    let g = color_vector[1] as f32;
    let b = color_vector[2] as f32;

    (r / 255.0, g / 255.0, b / 255.0)
}
