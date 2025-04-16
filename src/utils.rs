pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min { min }
    else if val > max { max }
    else { val }
}
