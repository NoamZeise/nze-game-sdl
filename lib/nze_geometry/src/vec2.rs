use std::ops;

/// A 2D Vector
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x : f64,
    pub y : f64,
}

impl Vec2 {
    /// Create a new [Vec2] with the suppied x and y components
    pub const fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    /// Returns a vector with `0.0` for x and y components
    pub const fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }
}

impl ops::Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, other : Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        *self = *self + other;
    }
}

impl ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, other : Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: f64) -> Vec2 {
        Vec2::new(self.x * other, self.y * other)
    }
}

impl ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x * other.x, self.y * other.y)
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, other: f64) -> Vec2 {
        Vec2::new(self.x / other, self.y / other)
    }
}

impl Eq for Vec2 {
    
}
