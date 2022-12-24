use sdl2::render::{TextureCreator, Texture, Canvas};
use sdl2::image::LoadTexture;
use sdl2::video::Window;
use sdl2::pixels::Color;

use std::collections::HashMap;
use std::path::Path;

use crate::resource;
use crate::rect_conversion::RectConversion;
use crate::types::Colour;
use geometry::*;

/// holds a `Texture` and some `Rect`s for representing sprites
#[derive(Clone, Copy)]
pub struct TextureDraw {
    pub draw_rect : Rect,
    pub tex_rect : Rect,
    pub colour : Colour,
    pub tex  : resource::Texture,
}

impl TextureDraw {
    pub fn new(tex : resource::Texture, draw_rect : Rect, tex_rect: Rect, colour: Colour) -> Self {
        TextureDraw {
            draw_rect,
            tex_rect,
            colour,
            tex
        }
    }
}


/// stores textures that are referenced by a `resource::Texture` object
pub struct TextureManager<'a, T> {
    texture_creator : &'a TextureCreator<T>,
    loaded_texture_paths : HashMap<String,  usize>,
    textures     : Vec<Texture<'a>>,
}

impl<'a, T> TextureManager<'a, T> {
    pub fn new(tex_creator: &'a TextureCreator<T>) -> Self {

        TextureManager {
            texture_creator : tex_creator,
            loaded_texture_paths: HashMap::new(),
            textures : Vec::new(),
        }
    }
/// load a texture to memory and get a `resource::Texture` object that references it
    pub fn load(&mut self, path : &Path) -> Result<resource::Texture, String> {
        let path_as_string = path.to_string_lossy().to_string();
        let tex_index = match self.loaded_texture_paths.contains_key(&path_as_string) {
            true => self.loaded_texture_paths[&path_as_string],
            false => {
                self.textures.push(self.texture_creator.load_texture(path)?);
                self.loaded_texture_paths.insert(path_as_string, self.textures.len() - 1);

                println!("loaded: {}", path.to_str().unwrap());

                self.textures.len() - 1
            },
        };
        let last_tex = &self.textures[tex_index];
        Ok(
        resource::Texture {
            id: tex_index,
            width: last_tex.query().width,
            height: last_tex.query().height,
        })

    }
/// draw a `GameObject` to the canvas
    pub fn draw(&mut self, canvas : &mut Canvas<Window>, tex_draw: TextureDraw) -> Result<(), String> {
        self.textures[tex_draw.tex.id].set_color_mod(
            tex_draw.colour.r,
            tex_draw.colour.g,
            tex_draw.colour.b
        );
        self.textures[tex_draw.tex.id].set_alpha_mod(tex_draw.colour.a);
        canvas.copy(
            &self.textures[tex_draw.tex.id],
            tex_draw.tex_rect.to_sdl_rect(),
            tex_draw.draw_rect.to_sdl_rect()
        )
    }

    pub fn draw_rect(&self, canvas : &mut Canvas<Window>, rect : &geometry::Rect, colour : &geometry::Rect) -> Result<(), String> {
        canvas.set_draw_color(Color::RGBA(colour.x as u8, colour.y as u8, colour.w as u8, colour.h as u8));
        canvas.fill_rect(rect.to_sdl_rect())?;
        Ok(())
    }
}

