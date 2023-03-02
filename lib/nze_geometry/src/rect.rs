use crate::Vec2;

///  A rectangle where x,y represents the coord of the upper left corner
///
/// Note: The axis the rectangle operates on is x is positive in the right direction
/// and y is positive in the down direction.
#[derive(Clone, Copy)]
pub struct Rect {
    /// x pos
    pub x : f64,
    /// y pos
    pub y : f64,
    /// Width
    pub w : f64,
    /// Height
    pub h : f64,
}

impl Rect {
    /// create a new [Rect] with supplied x, y, w, h values
    pub const fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rect { x, y, w, h }
    }

    /// construct a rect where the x, y, w, h components are all 0.0
    pub const fn zero() -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    /// construct a [Rect] that fills the area between the two [Vec2]s
    pub fn new_from_vec2s(v1 : &Vec2, v2 : &Vec2) -> Self {
        let mut smallest : Vec2 = *v1;
        let mut dim = Vec2::new(0.0, 0.0);

        if smallest.x > v2.x {
            smallest.x = v2.x;
            dim.x = v1.x - v2.x;
        } else {
            dim.x = v2.x - v1.x;
        }

        if smallest.y > v2.y {
            smallest.y = v2.y;
            dim.y = v1.y - v2.y;
        } else {
            dim.y = v2.y - v1.y;
        }
        
        Rect { x: smallest.x, y: smallest.y, w: dim.x, h: dim.y }
    }

    /// The centre of the [Rect] as a [Vec2], halfs the w,h components and adds them to the top left pos
    pub fn centre(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }

    /// Top left of the [Rect] as a [Vec2]
    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// check if this [Rect] intersects with another
    pub fn colliding(&self, rect : &Rect) -> bool {
        self.x < rect.x + rect.w &&
        self.x + self.w > rect.x &&
        self.y < rect.y + rect.h &&
        self.y + self.h > rect.y
    }

    // check if the passed [Vec2] is inside of this [Rect] 
    pub fn contains(&self, vec : &Vec2) -> bool {
        self.x          < vec.x &&
        self.x + self.w > vec.x &&
        self.y          < vec.y &&
        self.y + self.h > vec.y
    }
}
