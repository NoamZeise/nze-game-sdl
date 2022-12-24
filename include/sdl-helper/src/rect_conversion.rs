use geometry::Rect;
use sdl2;

pub trait RectConversion {
    fn new_from_sdl_rect(sdl_rect : &sdl2::rect::Rect) -> Self;
    fn to_sdl_rect(&self) -> sdl2::rect::Rect;
}

impl RectConversion for Rect{
    /// Use an `sdl2::rect::Rect` to construct a `Rect`
    fn new_from_sdl_rect(sdl_rect : &sdl2::rect::Rect) -> Self {
        Rect {
            x: sdl_rect.x as f64,
            y: sdl_rect.y as f64,
            w: sdl_rect.w as f64,
            h: sdl_rect.h as f64
        }
    }
    
    /// construct an `sdl2::rect::Rect` using this `Rect`
    fn to_sdl_rect(&self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32)
    }
}
