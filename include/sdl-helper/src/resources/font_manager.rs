use sdl2::render::{TextureCreator, Canvas};
use sdl2::{video::Window, pixels::Color, ttf};

use std::collections::HashMap;
use std::path::Path;

use crate::{resource, Colour, Error, rect_conversion::RectConversion};
use crate::{file_err, font_err, draw_err, unload_resource, load, load_resource_helper, draw};
use geometry::*;

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
    pub text: resource::Text,
    pub rect: Rect,
    pub colour: Colour,
}

const FONT_LOAD_SIZE : u16 = 128;

/// Stores 'sdl2::ttf::Font's and returns resources that represent loaded resources to fonts or text textures
pub struct FontManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    ttf_context: &'a ttf::Sdl2TtfContext,
    loaded_font_paths : HashMap<String, usize>,
    pub fonts : Vec<Option<ttf::Font<'a, 'static>>>,
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

    //load a ttf font face to memory and get a [resource::Font] object that references it
    pub fn load(&mut self, path : &Path) -> Result<resource::Font, Error>{
        let font_index =
            load!(path, self.fonts, self.loaded_font_paths, self.ttf_context, "Font", FONT_LOAD_SIZE);
        Ok(
            resource::Font {
            id: font_index,
        })
    }
    
    unload_resource!(///unloades the [resource::Font] stored by the sdl2 context, it can no longer be used
       ,self, self.loaded_font_paths, self.fonts, font, resource::Font, "font");

    /// return a [resource::Text] that can be put into a 'TextObject' to be passed to 'Camera'
    pub fn load_text(&mut self, font: &resource::Font, text: &str, colour : Colour) -> Result<resource::Text, Error> {
        if self.fonts[font.id].is_none() {
            return Err(Error::MissingResource(String::from("Used a text with an unloaded font")));
        }
        let t = Self::get_sdl2_texture(text, colour.to_sdl2_colour(), self.fonts[font.id].as_ref().unwrap(), self.texture_creator)?;
        //get dimensions before passing to check_and_push
        let width = t.query().width;
        let height = t.query().height;
        let index = load_resource_helper!(check_and_push(self.text_draws, Some(t)));
        Ok(resource::Text { id: index, width, height})
    }

    /// frees the texture stored and associated with the [resource::Text], must not be used after freeing
    pub fn unload_text(&mut self, text_draw: resource::Text) {
        self.text_draws[text_draw.id] = None;
    }
    
    fn get_draw(&self, font: &resource::Font, text: &str, height : u32, colour : Color) -> Result<ResourceTextDraw, Error> {
        self.get_draw_at_vec2(font, text, height, Vec2::new(0.0, 0.0), colour)
    }

    fn get_draw_at_vec2(&self, font: &resource::Font, text: &str, height : u32, pos: Vec2, colour: Color)
                        -> Result<ResourceTextDraw, Error> {
        if text.len() == 0 {
            return Err(Error::TextRender("text length should be greater than 0".to_string()));
        }
        if self.fonts[font.id].is_none() {
            return Err(Error::MissingResource("Font has been unloaded".to_string()))
        }
        let tex = Self::get_sdl2_texture(
            text, colour, self.fonts[font.id].as_ref().unwrap(), self.texture_creator)?;
        Ok(
            ResourceTextDraw {
                rect : Self::get_text_rect(&tex, pos, height),
                tex,
            }
        )
    }

    fn get_sdl2_texture(text: &str, colour : Color, font: &ttf::Font<'a, 'static>, texture_creator : &'a TextureCreator<T>) -> Result<sdl2::render::Texture<'a>, Error> {
        let surface = font_err!(font.render(text).blended(colour))?;
        Ok(font_err!(texture_creator.create_texture_from_surface(&surface))?)
    }
    
    fn get_text_rect(tex: &sdl2::render::Texture, pos: Vec2, height : u32) -> sdl2::rect::Rect {
        let dim  = Vec2::new(tex.query().width as f64, tex.query().height as f64);
        let rect = get_text_rect_from_height(dim, pos, height as f64);
        rect.to_sdl_rect()
    }

    /// draws the supplied text to the canvas in the supplied font at the given height and position
    pub(crate) fn draw(&self, canvas : &mut Canvas<Window>, font : &resource::Font, text: &str, height : u32, pos : Vec2, colour : Color) -> Result<(), Error> {
        if text.len() == 0 { return Ok(()); }
        let mut tex_draw = self.get_draw(font, text, height, colour)?;
        tex_draw.rect.x = pos.x as i32;
        tex_draw.rect.y = pos.y as i32;
        Ok(draw_err!(canvas.copy(&tex_draw.tex, None, tex_draw.rect))?)
    }

    pub(crate) fn draw_disposable(&self, canvas: &mut Canvas<Window>, disposable: DisposableTextDraw) -> Result<(), Error> {
        self.draw(canvas, &disposable.font, &disposable.text, disposable.height, disposable.pos, disposable.colour.to_sdl2_colour())
    }

    draw!{
        fn draw_text_draw(self, text_draw: TextDraw) (
            self.text_draws,
            text_draw.text.id,
            text_draw.colour,
            None,
            text_draw.rect.to_sdl_rect())
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
