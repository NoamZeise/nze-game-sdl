use sdl2::render::{TextureCreator, Canvas};
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::ttf;

use std::collections::HashMap;
use std::path::Path;

use geometry::*;
use crate::rect_conversion::RectConversion;
use crate::{resource, Colour};
use resource::Text;

struct ResourceTextDraw<'a> {
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

pub struct TextDraw {
    pub text: Text,
    pub rect: Rect,
    pub colour: Colour,
}

const FONT_LOAD_SIZE : u16 = 128;

/// Stores 'sdl2::ttf::Font's and returns resources that represent loaded resources to fonts or text textures
pub struct FontManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    ttf_context: &'a ttf::Sdl2TtfContext,
    loaded_font_paths : HashMap<String, usize>,
    pub fonts : Vec<ttf::Font<'a, 'static>>,
    text_draws: Vec<Option<sdl2::render::Texture<'a>>>,
}

impl<'a, T: 'a> FontManager<'a, T> {
    pub(crate) fn new(ttf_context : &'a ttf::Sdl2TtfContext, texture_creator : &'a TextureCreator<T>) -> Self {
        FontManager {
            texture_creator,
            ttf_context,
            loaded_font_paths: HashMap::new(),
            fonts : Vec::new(),
            text_draws: Vec::new(),
        }
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
                println!("loaded font: {}", path.to_str().unwrap());
                self.loaded_font_paths.insert(path_string, self.fonts.len() - 1);
                self.fonts.len() - 1
            }
        };
        Ok(
            resource::Font {
            id: font_index,
        })
    }
    /// return a [resource::Text] that can be put into a [TextObject] to be passed to [Camera]
    pub fn get_text(&mut self, font: &resource::Font, text: &str, colour : Colour) -> Result<Text, String>    {
        let t = Self::get_sdl2_texture(text, colour.to_sdl2_colour(), &self.fonts[font.id], self.texture_creator)?;
        let width = t.query().width;
        let height = t.query().height;
        for (i, e) in self.text_draws.iter_mut().enumerate() {
            if e.is_none() {
                *e = Some(t);
                return Ok(Text{id: i, width, height});
            }
        }
        self.text_draws.push(Some(t));
        Ok(Text { id: self.text_draws.len() - 1, width, height})
    }

    /// frees the texture stored and associated with the [TextDraw].
    /// The [TextDraw] must not be used after freeing
    pub fn unload_text_draw(&mut self, text_draw: Text) {
        self.text_draws[text_draw.id] = None;
    }
    
    fn get_draw(&self, font: &resource::Font, text: &str, height : u32, colour : Color) -> Result<ResourceTextDraw, String> {
        self.get_draw_at_vec2(font, text, height, Vec2::new(0.0, 0.0), colour)
    }

    fn get_draw_at_vec2(&self, font: &resource::Font, text: &str, height : u32, pos: Vec2, colour: Color) -> Result<ResourceTextDraw, String> {
        if text.len() == 0 { Err("text length should be greater than 0")?; }
        let tex = Self::get_sdl2_texture(text, colour, &self.fonts[font.id], self.texture_creator)?;
        Ok(
            ResourceTextDraw {
                rect : Self::get_text_rect(&tex, pos, height),
                tex,
            }
        )
    }

    fn get_sdl2_texture(text: &str, colour : Color, font: &ttf::Font<'a, 'static>, texture_creator : &'a TextureCreator<T>) -> Result<sdl2::render::Texture<'a>, String> {
        let surface = match font
            .render(text)
            .blended(colour) {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };
        match texture_creator.create_texture_from_surface(&surface) {
            Ok(t) => Ok(t),
            Err(e) => { return Err(e.to_string()); },
        }
    }
    
    fn get_text_rect(tex: &sdl2::render::Texture, pos: Vec2, height : u32) -> sdl2::rect::Rect {
        let dim  = Vec2::new(tex.query().width as f64, tex.query().height as f64);
        let rect = get_text_rect_from_height(dim, pos, height as f64);
        rect.to_sdl_rect()
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

    pub(crate) fn draw_text_draw(&mut self, canvas : &mut Canvas<Window>, text_draw: TextDraw) -> Result<(), String> {
        match &mut self.text_draws[text_draw.text.id] {
            Some(t) => {
                t.set_color_mod(text_draw.colour.r, text_draw.colour.g, text_draw.colour.b);
                t.set_alpha_mod(text_draw.colour.a);
                canvas.copy(&t, None, text_draw.rect.to_sdl_rect())
            },
            None => Err("text_draw used after free".to_string()),
        }
    }

}


    pub fn get_text_rect_from_height(dim: Vec2, pos: Vec2, height : f64) -> Rect {
        let ratio = dim.y / dim.x;
        Rect::new(
                pos.x,
                pos.y,
                height / ratio,
                height
             )
    }
