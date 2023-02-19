use geometry::*;
use sdl2::pixels::Color;
use crate::resource;


/// An RGBA colour with values from `0` to `255` for each channel
#[derive(Clone, Copy)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
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

    pub(crate) fn to_sdl2_colour(&self) -> Color {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

/// used by [crate::camera::Camera] for drawing texures with texture rects and draw rects and a colour
#[derive(Clone, Copy)]
pub struct GameObject {
    texture: resource::Texture,
    pub rect: Option<Rect>,
    pub tex_rect: Option<Rect>,
    pub parallax: Vec2,
    pub colour: Colour,
    pub rotate: f64,
    pub pivot: Option<Vec2>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}

impl GameObject {
    pub fn new_from_tex(texture: resource::Texture) -> Self {
        Self::new(
            texture,
            Some(Rect::new(0.0, 0.0, texture.width as f64, texture.height as f64)),
            None, Vec2::new(1.0, 1.0), Colour::white())
    }
    
    pub fn new(texture : resource::Texture, rect : Option<Rect>, tex_rect: Option<Rect>, parallax : Vec2, colour: Colour) -> Self {
        Self {
            texture,
            rect,
            tex_rect,
            parallax,
            colour,
            rotate: 0.0,
            pivot: None,
            flip_horizontal: false,
            flip_vertical: false,
        }
    }
    pub(crate) fn get_texture(&self) -> resource::Texture {
        self.texture
    }
}


/// Used by [crate::camera::Camera]. An object that stores text with some drawing settings for rendering
pub type TextObject = GameObject;


/// holds a `Texture` and some `Rect`s for representing sprites
#[derive(Clone, Copy)]
pub(crate) struct TextureDraw {
    pub draw_rect : Option<Rect>,
    pub tex_rect : Option<Rect>,
    pub colour : Colour,
    pub tex  : resource::Texture,
    pub angle: f64,
    pub centre: Option<Vec2>,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
}
