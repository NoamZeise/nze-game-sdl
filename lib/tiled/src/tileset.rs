use std::path::{Path, PathBuf};

use super::helper::*;
use super::error::TiledError;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::reader::Reader;

/// Holds information about a tileset. ie. image path, format, spacing
///
/// Tileset information comes from a .tsx file linked to the tilemap
/// and is automatically loaded when loading with `Map::new()`.
///
/// Tilesets can also be created independantly using `Tileset::load`
/// In which case the `first_tile_id` field will be `0`.
pub struct Tileset {
    /// The index of the first tile relative to the rest of the tilemaps
    /// in a `Map`.
    pub first_tile_id : u32,
    pub name : String,
    pub tile_width : u32,
    pub tile_height : u32,
    pub tile_count : u32,
    pub column_count : u32,

    pub margin : u32,
    pub spacing : u32,

    /// This path will have the same parent directory as the Tileset/Tilemap files.
    pub image_path : PathBuf,
    pub image_width : u32,
    pub image_height : u32,

    pub version : String,
    pub tiledversion : String,
}

impl Tileset {
    fn blank() -> Tileset {
        Tileset {
            first_tile_id : 0,
            name : String::from(""),
            tile_width : 0,
            tile_height : 0,
            tile_count : 0,
            column_count : 0,
            margin : 0,
            spacing : 0,
            image_path : PathBuf::new(),
            image_width : 0,
            image_height : 0,
            version : String::new(),
            tiledversion : String::new(),
            }
    }

    fn parse_tileset_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"name" => self.name = get_value(&a.value)?,
                b"tilewidth" => self.tile_width = get_value(&a.value)?,
                b"tileheight" => self.tile_height = get_value(&a.value)?,
                b"spacing" => self.spacing = get_value(&a.value)?,
                b"margin" => self.margin = get_value(&a.value)?,
                b"tilecount" => self.tile_count = get_value(&a.value)?,
                b"columns" => self.column_count = get_value(&a.value)?,
                b"version" => self.version = get_string(&a.value)?.to_string(),
                b"tiledversion" => self.tiledversion = get_string(&a.value)?.to_string(),
                _ => println!("warning: unrecognized attribute {:?}", a.key),
            }
        }
        Ok(())
    }

    fn parse_image_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"source" => self.image_path.push(get_string(&a.value)?),
                b"width" => self.image_width = get_value(&a.value)?,
                b"height" => self.image_height = get_value(&a.value)?,
                _ => println!("warning: unrecognized attribute {:?}", a.key),
            }
        }
        Ok(())
    }
    
    fn parse_xml(tileset : &mut Self, tsx_text : String) -> Result<(), TiledError>{
        let mut reader = Reader::from_str(&tsx_text);
        parse_xml(tileset, &mut reader)
    }

    /// Get the tileset data without making a tilemap
    ///
    /// Note: will have a first_tile_id of 0
    pub fn load(path: &Path) -> Result<Tileset, TiledError> {
        let mut tileset = Self::blank();
        match path.parent() {
            Some(parent_dir) => tileset.image_path.push(parent_dir),
            _ => (),
        }
        Self::parse_xml(
            &mut tileset,
            read_file_to_string(path)?
        )?;
        Ok(tileset)
    }
    
    pub(crate) fn new(attribs : Vec<Attribute>, path : &Path) -> Result<Tileset, TiledError> {
        let mut tmx_path = path.to_path_buf();
        let mut tileset = Self::blank();
        tileset.image_path.push(path);
        for a in attribs {
            match a.key.as_ref() {
                b"firstgid" => tileset.first_tile_id = get_value(&a.value)?,
                b"source" => {
                    Self::parse_xml(
                        &mut tileset,
                        read_file_to_string( {
                            tmx_path.push(get_string(&a.value)?);
                            &tmx_path
                        })?
                    )?;
                }
                _  => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(tileset)
    }
}

impl HandleXml for Tileset {
    fn start(&mut self, e : &BytesStart, _: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"tileset" => self.parse_tileset_attribs(collect_attribs(&e)?)?,
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"image" => self.parse_image_attribs(collect_attribs(&e)?)?,
            _ => println!("unrecognized empty tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        "tileset"
    }
}



#[cfg(test)]
mod tileset_tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn test_tileset() {
        let tileset = Tileset::load(Path::new("test-resources/test.tsx"));
        assert!(tileset.is_ok());
        let tileset = tileset.unwrap();
        assert!(tileset.first_tile_id == 0);
        assert!(tileset.tile_width == 10);
        assert!(tileset.tile_height == 10);
        assert!(tileset.spacing == 2);
        assert!(tileset.margin == 5);
        assert!(tileset.tile_count == 4);
        assert!(tileset.column_count == 2);
        assert!(tileset.image_path == Path::new("test-resources/test-tileset.png"));
        assert!(tileset.image_width == 32);
        assert!(tileset.image_height == 32);
    }
}
