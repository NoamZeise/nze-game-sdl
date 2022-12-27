use crate::resource::Texture;
use crate::Rect;
use crate::Error;

#[derive(Clone)]
pub struct Tile {
    pub tex : Texture,
    pub rect: Rect,
}

impl Tile {
    pub fn new() -> Tile {
        Tile {
            tex: Texture{ id: 0, width: 0, height: 0},
            rect: Rect::zero(),
        }
    }
}

pub fn load_tileset(tiles: &mut Vec<Tile>, ts: &tiled::Tileset, tex: Texture) -> Result<(), Error> {
    let mut id = ts.first_tile_id as usize;
    for y in 0..(ts.tile_count / ts.column_count) {
        for x in 0..ts.column_count {
            if id >= tiles.len() {
                return Err(Error::LoadFile(String::from("Map Tile Count did not match actual tilecount")));
            }
            
            tiles[id].tex = tex;
            tiles[id].rect = Rect::new(
                ts.margin as f64 + ((ts.tile_width + ts.spacing)  * x) as f64,
                ts.margin as f64 + ((ts.tile_height + ts.spacing) * y) as f64,
                ts.tile_width as f64,
                ts.tile_height as f64,
            );
            id += 1;
        }
    }
    Ok(())
}
