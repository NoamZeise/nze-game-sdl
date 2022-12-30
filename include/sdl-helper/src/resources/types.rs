use geometry::*;
use sdl2::pixels::Color;
use crate::resource;


/// An RGBA colour with values from 0-255 for each channel
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

/// used by `Camera` for drawing texures with texture rects and draw rects and a colour
#[derive(Clone, Copy)]
pub struct GameObject {
    texture: resource::Texture,
    pub rect: Rect,
    pub tex_rect: Rect,
    pub parallax: Vec2,
    pub colour: Colour
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
    pub(crate) fn get_texture(&self) -> resource::Texture {
        self.texture
    }
}

/// used by `Camera` for drawing loaded font texts
#[derive(Clone, Copy)]
pub struct TextObject {
    pub(crate) texture: resource::Text,
    pub rect: Rect,
    pub parallax: Vec2,
    pub colour: Colour,
}

impl TextObject {
    pub fn new(text: resource::Text, rect: Rect, parallax: Vec2, colour: Colour) -> TextObject {
        TextObject { texture: text, rect, parallax, colour}
    }
}
