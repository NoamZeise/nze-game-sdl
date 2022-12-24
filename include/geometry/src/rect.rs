use crate::Vec2;

///  A rectangle where x,y represents the coord of the upper left corner
#[derive(Clone, Copy)]
pub struct Rect {
    pub x : f64,
    pub y : f64,
    pub w : f64,
    pub h : f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Self {
        Rect { x, y, w, h }
    }

    /// construct a rect where the x, y, w, h components are all 0.0
    pub fn blank() -> Rect {
        Rect::new(0.0, 0.0, 0.0, 0.0)
    }

    /// construct a vector where the smallest x, y values from each vector are used
    /// and the dimentions are the largest vector take away the smallest vector in each direction
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

    pub fn centre(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }

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

    // check if the [Vec2] is inside of this [Rect] 
    pub fn contains(&self, vec : &Vec2) -> bool {
        self.x          < vec.x &&
        self.x + self.w > vec.x &&
        self.y          < vec.y &&
        self.y + self.h > vec.y
    }
}
