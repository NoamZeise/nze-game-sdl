use sdl2::pixels::Color;
use std::clone::Clone;

pub mod input;
use geometry::*;
pub mod map;
pub mod camera;
pub mod texture_manager;
pub mod font_manager;
pub mod resource;

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

#[derive(Clone, Copy)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Colour {
        Colour { r, g, b, a }
    }
    pub fn new_from_floats(r: f64, g: f64, b: f64, a: f64) -> Colour {
        Self::new(
            (r / 255.0) as u8,
            (g / 255.0) as u8,
            (b / 255.0) as u8,
            (a / 255.0) as u8,
        ) 
    }
    pub fn white() -> Colour {
        Self::new(255, 255, 255, 255)
    }

    pub fn to_sdl2_colour(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

#[derive(Clone, Copy)]
pub struct GameObject {
    texture: resource::Texture,
    rect: Rect,
    tex_rect: Rect,
    parallax: Vec2,
    colour: Colour
}

impl GameObject {
    pub fn new_from_tex(texture: resource::Texture) -> Self {
        let r = Rect::new(0.0, 0.0, texture.width as f64, texture.height as f64);
        Self {
            texture,
            rect: r,
            tex_rect : r,
            parallax: Vec2::new(1.0, 1.0),
            colour: Colour::white(),
        }
    }
    pub fn new(texture : resource::Texture, rect : Rect, tex_rect: Rect, parallax : Vec2, colour: Colour) -> Self {
        Self {
            texture,
            rect,
            tex_rect,
            parallax,
            colour,
        }
    }
}

