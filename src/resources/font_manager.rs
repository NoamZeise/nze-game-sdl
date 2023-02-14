use sdl2::render::{TextureCreator, Canvas};
use sdl2::{video::Window, pixels::Color, ttf};

use std::collections::HashMap;
use std::path::Path;

use crate::{resource::{Font, Text}, Colour, Error, rect_conversion::RectConversion};
use crate::{file_err, font_err, draw_err, unload_resource, load, load_resource_helper, draw, TextObject};
use geometry::*;

pub struct TextDraw {
    pub text: Text,
    pub rect: Rect,
    pub colour: Colour,
}

struct ResourceTextDraw<'a> {
    tex  : sdl2::render::Texture<'a>,
    rect : sdl2::rect::Rect,
}

pub(crate) struct DisposableTextDraw {
    pub font : Font,
    pub text: String,
    pub height : u32,
    pub pos : Vec2,
    pub colour : Colour,
    pub rect: Rect,
}

const FONT_LOAD_SIZE : u16 = 128;

/// Stores [sdl2::ttf::Font]s and creates [Font]s or [TextObject]s. Created and owned by [crate::Render]
pub struct FontManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    ttf_context: &'a ttf::Sdl2TtfContext,
    loaded_font_paths : HashMap<String, usize>,
    pub fonts : Vec<Option<ttf::Font<'a, 'static>>>,
    text_draws: Vec<Option<sdl2::render::Texture<'a>>>,
}

impl<'a, T: 'a> FontManager<'a, T> {
    
    //load a ttf font face to memory and get a [Font] object that references it
    pub fn load_font(&mut self, path : &Path) -> Result<Font, Error>{
        let font_index =
            load!(path, self.fonts, self.loaded_font_paths, self.ttf_context, "Font", FONT_LOAD_SIZE);
        Ok(
            Font {
            id: font_index,
        })
    }
    
    unload_resource!(
        ///unloades the [Font] stored by the sdl2 context, it can no longer be used
        ,unload, self, self.loaded_font_paths, self.fonts, font, Font, "font");
    
    /// return a [TextObject] that can be passed to 'Camera' to draw to the screen
    ///
    /// You can freely  modify the TextObject's public struct members, and the changes will automatically appear
    ///
    /// Note: The colour supplied as a function arg will be the coolour the font texture is generated with, the text drwa will be given the colour whit.
    /// The colour of the text draw is used to change the colour of the sdl2 draw commond,
    /// so setting the colour of the text draw will have a mixing effect on the colour of the text
    pub fn load_text_obj(&mut self, font: &Font, text: &str, colour : Colour, pos: Vec2, height: f64, parallax: Vec2) -> Result<TextObject, Error> {
        if self.fonts[font.id].is_none() {
            return Err(Error::MissingResource(String::from("Used a text with an unloaded font")));
        }
        let t = Self::gen_sdl2_texture(text, colour.to_sdl2_colour(), self.fonts[font.id].as_ref().unwrap(), self.texture_creator)?;
        //get dimensions before passing to check_and_push
        let tex_width = t.query().width;
        let tex_height = t.query().height;
        let index = load_resource_helper!(check_and_push(self.text_draws, Some(t)));
        let text_resource = Text { id: index, width: tex_width, height: tex_height};
        Ok(TextObject::new(
            text_resource,
            get_text_rect_from_height(
                Vec2::new(text_resource.width as f64, text_resource.height as f64),
                pos,
                height),
            parallax, Colour::white()
        ))
    }

    /// frees the texture stored and associated with the [TextObject], must not be used after freeing
    ///
    /// This function does not have to be used to free your textures at the end of the program.
    /// This function is only nessecary if you want to clear some room in memory to load more resources.
    /// For example you can unload assets from one level and load in the next which switching levels.
    pub fn unload_text_obj(&mut self, text_obj: TextObject) {
        self.text_draws[text_obj.texture.id] = None;
    }

    pub(crate) fn new(ttf_context : &'a ttf::Sdl2TtfContext, texture_creator : &'a TextureCreator<T>) -> Self {
        FontManager {
            texture_creator,
            ttf_context,
            loaded_font_paths: HashMap::new(),
            fonts : Vec::new(),
            text_draws: Vec::new(),
        }
    }

    /// draws the supplied text to the canvas in the supplied font at the given height and position
    fn draw(&self, canvas : &mut Canvas<Window>, font : &Font, text: &str, height : u32, pos : Vec2, colour : Color, rect: Rect) -> Result<(), Error> {
        if text.len() == 0 { return Ok(()); }
        let mut tex_draw = self.get_rendered_text(font, text, height, colour)?;
        tex_draw.rect.x = pos.x as i32;
        tex_draw.rect.y = pos.y as i32;
        tex_draw.rect.h = rect.h as i32;
        tex_draw.rect.w =
            (tex_draw.rect.w as f64 *
            (rect.w / height as f64)) as i32;
        Ok(draw_err!(canvas.copy(&tex_draw.tex, None, tex_draw.rect))?)
    }

    pub(crate) fn draw_disposable(&self, canvas: &mut Canvas<Window>, disposable: DisposableTextDraw) -> Result<(), Error> {
        self.draw(canvas, &disposable.font, &disposable.text, disposable.height, disposable.pos, disposable.colour.to_sdl2_colour(), disposable.rect)
    }

    draw!{
        fn draw_text_draw(self, text_draw: TextDraw) (
            self.text_draws,
            text_draw.text.id,
            text_draw.colour,
            None,
            text_draw.rect.to_sdl_rect())
    }

    
    fn get_rendered_text(&self, font: &Font, text: &str, height : u32, colour : Color) -> Result<ResourceTextDraw, Error> {
        self.get_rendered_text_at_position(font, text, height, Vec2::new(0.0, 0.0), colour)
    }

    fn get_rendered_text_at_position(&self, font: &Font, text: &str, height : u32, pos: Vec2, colour: Color) -> Result<ResourceTextDraw, Error> {
        if text.len() == 0 {
            return Err(Error::TextRender("text length should be greater than 0".to_string()));
        }
        if self.fonts[font.id].is_none() {
            return Err(Error::MissingResource("Font has been unloaded".to_string()))
        }
        let tex = Self::gen_sdl2_texture(
            text, colour, self.fonts[font.id].as_ref().unwrap(), self.texture_creator)?;
        Ok(
            ResourceTextDraw {
                rect : Self::get_rect_from_sdl_texture(&tex, pos, height),
                tex,
            }
        )
    }

    fn gen_sdl2_texture(text: &str, colour : Color, font: &ttf::Font<'a, 'static>, texture_creator : &'a TextureCreator<T>) -> Result<sdl2::render::Texture<'a>, Error> {
        let surface = font_err!(font.render(text).blended(colour))?;
        Ok(font_err!(texture_creator.create_texture_from_surface(&surface))?)
    }
    
    fn get_rect_from_sdl_texture(tex: &sdl2::render::Texture, pos: Vec2, height : u32) -> sdl2::rect::Rect {
        let dim  = Vec2::new(tex.query().width as f64, tex.query().height as f64);
        let rect = get_text_rect_from_height(dim, pos, height as f64);
        rect.to_sdl_rect()
    }
}

fn get_text_rect_from_height(dim: Vec2, pos: Vec2, height : f64) -> Rect {
    let ratio = dim.y / dim.x;
    Rect::new(
        pos.x,
        pos.y,
        height / ratio,
        height
    )
}
