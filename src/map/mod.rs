use std::path::Path;

pub use tiled;
use crate::Camera;
use crate::FontManager;
use crate::TextureManager;
use crate::Error;

mod tile;
mod layer;

use tile::*;
use layer::*;


/// Used for drawing [tiled] maps
pub struct Map {
    pub tiled_map: tiled::Map,
    tiles : Vec<Tile>,
    pub layers : Vec<Layer>,
}

impl Map {
    /// Loads the tiled map at the path into memory and loads any resources referenced by the map
    ///
    /// Will throw a `LoadFile` error if there is an error loading the tiled map with `tiled`
    ///
    /// Will pass on any errors from loading resources 
    pub fn new<'sdl, TexType>(filename: &Path, tex_manager : &'sdl mut TextureManager<TexType>, font_folder: &Path, font_manager: &'sdl mut FontManager<TexType>) -> Result<Self, Error> {
        let mut map = Self {
            tiled_map: tiled::Map::new(filename).map_err(|e| { Error::LoadFile(format!("{:?}", e))})?,
            tiles: Vec::new(),
            layers: Vec::new(),
        };

        map.layers.resize(
            map.tiled_map.layers.len() + map.tiled_map.img_layers.len() + map.tiled_map.obj_groups.len(),
            Layer::blank()
        );

        map.load_tilesets(tex_manager)?;
        map.set_map_draws();
        map.set_img_layers(tex_manager)?;
        map.set_obj_group_layers(font_folder, font_manager)?;
        
        map.clear_blank_layers();
        
        Ok(map)
    }

    /// Draw the map to the camera's buffer, adjusted to the camera's offset and scale
    pub fn draw(&self, cam: &mut Camera) {
        for l in self.layers.iter() {
            l.draw(cam);
        }
    }

    fn load_tilesets<'sdl, TexType>(&mut self, tex_manager : &'sdl mut TextureManager<TexType>) -> Result<(), Error> {
        self.tiles.resize(self.tiled_map.total_tiles as usize, Tile::new());
        // blank tile
        self.tiles[0].rect.w = self.tiled_map.tile_width as f64;
        self.tiles[0].rect.h = self.tiled_map.tile_height as f64;
        for ts in self.tiled_map.tilesets.iter() {
            load_tileset(&mut self.tiles, ts, tex_manager.load(&Path::new(&ts.image_path))?)?;
        }
        Ok(())
    }

    fn set_map_draws(&mut self) {
        for l in self.tiled_map.layers.iter() {
            self.layers[l.info.layer_position as usize] = Layer::new_tile_layer(&l, &self.tiles);
        }
    }

    fn set_img_layers<'sdl, TexType>(&mut self, tex_manager : &'sdl mut TextureManager<TexType>) -> Result<(), Error> {
        for l in self.tiled_map.img_layers.iter() {
            self.layers[l.info.layer_position as usize] = Layer::new_image_layer(
                l, tex_manager.load(&self.tiled_map.tilemap_directory.join(&l.image_path))?
            )
        }
        Ok(())
    }

    fn set_obj_group_layers<'sdl, TexType>(&mut self, font_folder: &Path, font_manager : &'sdl mut FontManager<TexType>) -> Result<(), Error> {
        for l in self.tiled_map.obj_groups.iter() {
            self.layers[l.info.layer_position as usize] = Layer::new_object_layer(
                font_folder,
                l,
                font_manager
            )?;
        }

        Ok(())
    }

    fn clear_blank_layers(&mut self) {
        let mut i = 0;
        while i < self.layers.len() {
            if self.layers[i].is_blank() {
                self.layers.remove(i);
                i -= 1;
            }
            i += 1;
        }
    }
}
