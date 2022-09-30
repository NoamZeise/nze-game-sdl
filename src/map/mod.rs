use std::path::Path;

use tiled;
use crate::{TextDraw, GameObject};
use crate::{TextureManager, resource::Texture};
use sdl2::render::Canvas;
use sdl2::video::Window;
use geometry::Rect;

#[derive(Clone)]
struct Tile {
    pub tex : Texture,
    pub rect: Rect,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            tex: Texture{ id: 0, width: 0, height: 0},
            rect: Rect::blank(),
        }
    }
}

struct Layer {
    tile_draws: Vec<GameObject>,
}

impl Layer {
    fn new(l: &tiled::Layer, tiles: &Vec<Tile>) -> Layer {
        let mut layer = Layer { tile_draws: Vec::new() };
        for y in 0..l.height {
            for x in 0..l.width {
                let tile = &tiles[l.tiles[(y * l.width + x) as usize] as usize];
                layer.tile_draws.push(
                    GameObject::new(
                        tile.tex,
                        Rect::new(
                            l.info.offset.x + (x as f64 * tile.rect.w),
                            l.info.offset.y + (y as f64 * tile.rect.h),
                            tile.rect.w,
                            tile.rect.h,
                        ),
                        tile.rect,
                    )
                );
            }
        }
        layer
    }
}

pub struct Map {
    tiled_map: tiled::Map,
    tiles : Vec<Tile>,
    layers : Vec<Layer>,
}

impl Map {
    pub fn new<'sdl, TexType>(filename: &str, tex_manager : &'sdl mut TextureManager<TexType>) -> Result<Self, String> {
        let mut map = Self {
            tiled_map: tiled::Map::new(filename).unwrap(),
            tiles: Vec::new(),
            layers: Vec::new(),
        };

        map.load_tilesets(tex_manager)?;
        map.set_map_draws();
        
        Ok(map)
    }

    pub fn draw<'sdl, TexType>(&self, canvas : &mut Canvas<Window>, tex_manager : &'sdl TextureManager<TexType>) -> Result<(), String> {
        for l in self.layers.iter() {
            for t in l.tile_draws.iter() {
                tex_manager.draw(canvas, &t)?;
            }
        }
        Ok(())
    }


    fn load_tilesets<'sdl, TexType>(&mut self, tex_manager : &'sdl mut TextureManager<TexType>) -> Result<(), String> {
        self.tiles.resize(self.tiled_map.total_tiles as usize, Tile::new());
        self.tiles[0].rect.w = self.tiled_map.tile_width as f64;
        self.tiles[0].rect.h = self.tiled_map.tile_height as f64;
        println!("{}", self.tiled_map.total_tiles);
        for ts in self.tiled_map.tilesets.iter() {
            let tex = tex_manager.load(&Path::new(&ts.image_path))?;
            let mut id = ts.first_tile_id as usize;
            for y in 0..(ts.tile_count / ts.column_count) {
                for x in 0..ts.column_count {
                    if id >= self.tiles.len() {
                        return Err(String::from("Map Tile Count did not match actual tilecount"));
                    }
                    self.tiles[id].tex = tex;
                    self.tiles[id].rect = Rect::new(
                        (ts.tile_width * x) as f64,
                        (ts.tile_height * y) as f64,
                        ts.tile_width as f64,
                        ts.tile_height as f64,
                    );
                    id += 1;
                }
            }
        }
        Ok(())
    }

    fn set_map_draws(&mut self) {
        for l in self.tiled_map.layers.iter() {
            self.layers.push(Layer::new(&l, &self.tiles));
        }
    }
}
