
#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vector2 { x, y }
    }
}

pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2::new(x, y)
}
