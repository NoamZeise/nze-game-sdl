use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf;

use std::collections::HashMap;
use std::path::Path;

use geometry::Vec2;
use crate::{resource, Colour};

/// can be returned by [FontManager], stores an sdl2 texture and a rect for drawing to a canvas
pub struct TextDraw<'a> {
    tex  : sdl2::render::Texture<'a>,
    rect : sdl2::rect::Rect,
}

pub struct DisposableTextDraw {
    pub font : resource::Font,
    pub text: String,
    pub height : u32,
    pub pos : Vec2,
    pub colour : Colour, 
}

const FONT_LOAD_SIZE : u16 = 128;

/// Stores 'sdl2::ttf::Font' and returns textures or draws them
pub struct FontManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    ttf_context: &'a ttf::Sdl2TtfContext,
    loaded_font_paths : HashMap<String, usize>,
    pub fonts : Vec<ttf::Font<'a, 'static>>,
}

impl<'a, T> FontManager<'a, T> {
    pub(crate) fn new(ttf_context : &'a ttf::Sdl2TtfContext, texture_creator : &'a TextureCreator<T>) -> Result<Self, String> {
        Ok(FontManager {
            texture_creator,
            ttf_context,
            loaded_font_paths: HashMap::new(),
            fonts : Vec::new(),
        })
    }

    pub fn load_font(&mut self, path : &Path) -> Result<resource::Font, String>{
        let path_string = path.to_string_lossy().to_string();
        let font_index = match self.loaded_font_paths.contains_key(&path_string) {
            true => self.loaded_font_paths[&path_string],
            false => {
                self.fonts.push(
                    match self.ttf_context.load_font(path, FONT_LOAD_SIZE) {
                        Ok(s) => s,
                        Err(e) => { return Err(e.to_string()); }
                    }
                );
                self.loaded_font_paths.insert(path_string, self.fonts.len() - 1);
                self.fonts.len() - 1
            }
        };
        Ok(
            resource::Font {
            id: font_index,
        })
    }
    /// return a `TextDraw` that has a corrected `rect.width` based on the supplied height and the rendered font
    pub fn get_draw(&self, font: &resource::Font, text: &str, height : u32, colour : Color) -> Result<TextDraw, String> {
        self.get_draw_at_vec2(font, text, height, Vec2::new(0.0, 0.0), colour)
    }

    pub fn get_draw_at_vec2(&self, font: &resource::Font, text: &str, height : u32, pos: Vec2, colour: Color) -> Result<TextDraw, String> {
        if text.len() == 0 { Err("text length should be greater than 0")?; }
        let surface = match self.fonts[font.id]
            .render(text)
            .blended(colour) {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
        };
        let tex = match self.texture_creator.create_texture_from_surface(&surface) {
            Ok(t) => t,
            Err(e) => { return Err(e.to_string()); },
        };
        let ratio = tex.query().height as f64 / tex.query().width as f64;
        Ok(
        TextDraw {
            tex,
            rect:
             sdl2::rect::Rect::new(
                pos.x as i32,
                pos.y as i32,
                (height as f64 / ratio) as u32,
                height
             ),
        })
    }

    /// draws the supplied text to the canvas in the supplied font at the given height and position
    pub(crate) fn draw(&self, canvas : &mut Canvas<Window>, font : &resource::Font, text: &str, height : u32, pos : Vec2, colour : Color) -> Result<(), String> {
        if text.len() == 0 { return Ok(()); }
        let mut tex_draw = self.get_draw(font, text, height, colour)?;
        tex_draw.rect.x = pos.x as i32;
        tex_draw.rect.y = pos.y as i32;
        canvas.copy(&tex_draw.tex, None, tex_draw.rect)
    }

    pub(crate) fn draw_disposable(&self, canvas: &mut Canvas<Window>, disposable: DisposableTextDraw) -> Result<(), String> {
        self.draw(canvas, &disposable.font, &disposable.text, disposable.height, disposable.pos, disposable.colour.to_sdl2_colour())?;
        Ok(())
    }
}
