use std::path::Path;
use crate::{GameObject, TextObject, Colour, resource, manager::FontManager, Camera, Error};
use super::tile::*;
use geometry::*;

#[derive(Clone)]
pub struct Layer {

    pub tile_draws: Vec<GameObject>,
    pub image_draw: Option<GameObject>,
    pub text_draw: Vec<TextObject>,
    pub visible: bool,
}

impl Layer {
    pub fn blank() -> Layer {
        Layer { tile_draws: Vec::new(), image_draw: None, text_draw: Vec::new(), visible: true,}
    }
    pub fn new_tile_layer(l: &tiled::Layer, tiles: &Vec<Tile>) -> Layer {
        let mut layer = Self::blank();
        for y in 0..l.height {
            for x in 0..l.width {
                let tile_id = l.tiles[(y * l.width + x) as usize] as usize;
                if tile_id == 0 { continue; }
                let tile = &tiles[tile_id];
                layer.tile_draws.push(
                    GameObject::new(
                        tile.tex,
                        Rect::new(
                            l.info.offset.x + (x as f64 * tile.rect.w),
                            l.info.offset.y + (y as f64 * tile.rect.h),
                            tile.rect.w,
                            tile.rect.h,
                        ),
                        Some(tile.rect),
                        l.info.parallax,
                        Colour::new(
                            l.info.tint.r as u8,
                            l.info.tint.g as u8,
                            l.info.tint.b as u8,
                            (l.info.opacity * 255.0) as u8,
                        )
                    )
                );
            }
        }
        layer.visible = l.info.visible;
        layer
    }
    
    pub fn new_image_layer(l: &tiled::ImageLayer, tex: resource::Texture ) -> Layer {
        let mut layer = Self::blank();
        layer.image_draw = Some(
            GameObject::new(
                tex,
                Rect::new(
                    l.info.offset.x,
                    l.info.offset.y,
                    l.width as f64,
                    l.height as f64),
                Some(Rect::new(0.0, 0.0, l.width as f64, l.height as f64)),
                l.info.parallax,
                Colour::new(
                    l.info.colour.r as u8,
                    l.info.colour.g as u8,
                    l.info.colour.b as u8,
                    l.info.colour.a as u8)
            )
        );
        layer
    }

    pub fn new_object_layer<'sdl, TexType>(font_folder: &Path, l: &tiled::ObjGroup, font_manager : &'sdl mut FontManager<TexType>) -> Result<Layer, Error> {
        let mut layer = Layer::blank();
        let layer_colour = Colour::new(
            l.info.colour.r as u8,
            l.info.colour.g as u8,
            l.info.colour.b as u8,
            l.info.colour.a as u8
            );
        for t in l.text.iter() {
            let font = font_manager.load_font(&font_folder.join(Path::new(&(t.font_family.replace(" ", "-") + ".ttf"))))?;

            layer.text_draw.push(
                font_manager.load_text_obj(
                    &font,
                    &t.text,
                    layer_colour, //use layer colour to generate text texture
                    t.obj.rect.top_left(),
                    t.obj.rect.h,
                    l.info.parallax
                )?
            );
            // change the text draw's colour to the colour of the text object,
            // so that the layer and text object colours are mixed
            layer.text_draw.last_mut().unwrap().colour = Colour::new(
                t.colour.r as u8,
                t.colour.g as u8,
                t.colour.b as u8,
                t.colour.a as u8
            );
        }
        Ok(layer)
    }

    pub fn draw(&self, cam: &mut Camera) {
        for t in self.tile_draws.iter() {
            cam.draw(t);
        }
        match self.image_draw {
            Some(g) => cam.draw(&g),
            None => (),
        }
        for t in self.text_draw.iter() {
            cam.draw_text(t);
        }
    }

    pub fn is_blank(&self) -> bool {
        self.image_draw.is_none() && self.tile_draws.len() == 0 && self.text_draw.len() == 0
    }
}
