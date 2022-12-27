use std::ops;

/// A 2D Vector
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x : f64,
    pub y : f64,
}

impl Vec2 {
    /// Create a new [Vec2] with the suppied x and y components
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, other : Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}
