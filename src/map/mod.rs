use tiled;

pub struct Map {
    tiled_map: tiled::Map,
}

impl Map {
    pub fn new(filename: &str) -> Result<Self, ()> {
        let map = Self {
            tiled_map : tiled::Map::new(filename).unwrap(),
        };
        Ok(map)
    }
}
