//! Shape structs with `f64` members

use sdl2::rect;

use std::ops;

///  A rectangle where x,y represents the coord of the upper left corner
#[derive(Clone)]
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
    
    /// Use an `sdl2::rect::Rect` to construct a `Rect`
    pub fn new_from_sdl_rect(sdl_rect : &rect::Rect) -> Self {
        Rect {
            x: sdl_rect.x as f64,
            y: sdl_rect.y as f64,
            w: sdl_rect.w as f64,
            h: sdl_rect.h as f64
        }
    }
    /// construct an `sdl2::rect::Rect` using this `Rect`
    pub fn to_sdl_rect(&self) -> rect::Rect {
        rect::Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }

    pub fn centre(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }

    pub fn colliding(&self, rect : &Rect) -> bool {
        self.x < rect.x + rect.w &&
        self.x + self.w > rect.x &&
        self.y < rect.y + rect.h &&
        self.y + self.h > rect.y
    }

    pub fn contains(&self, vec : &Vec2) -> bool {
        self.x          < vec.x &&
        self.x + self.w > vec.x &&
        self.y          < vec.y &&
        self.y + self.h > vec.y
    }
}

/// A 2D Vector
#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x : f64,
    pub y : f64,
}

impl Vec2 {
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
