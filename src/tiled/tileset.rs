use super::helper::*;
use super::error::TiledError;
use super::Tileset;

use std::io::Read;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

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
            image_path : String::from(""),
            image_width : 0,
            image_height : 0,
            }
    }

    fn parse_tileset_attribs(tileset : &mut Self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"name" => tileset.name = get_value(&a.value)?,
                b"tilewidth" => tileset.tile_width = get_value(&a.value)?,
                b"tileheight" => tileset.tile_height = get_value(&a.value)?,
                b"spacing" => tileset.spacing = get_value(&a.value)?,
                b"margin" => tileset.margin = get_value(&a.value)?,
                b"tilecount" => tileset.tile_count = get_value(&a.value)?,
                b"columns" => tileset.column_count = get_value(&a.value)?,
                _ => println!("warning: unrecognized attribute {:?}", a.key),
            }
        }
        Ok(())
    }

    fn parse_image_attribs(tileset: &mut Self, attribs : Vec<Attribute>, path : &String) -> Result<(), TiledError> {
        tileset.image_path = path.to_owned();
        for a in attribs {
            match a.key.as_ref() {
                b"source" => tileset.image_path.push_str(get_string(&a.value)?),
                b"width" => tileset.image_width = get_value(&a.value)?,
                b"height" => tileset.image_height = get_value(&a.value)?,
                _ => println!("warning: unrecognized attribute {:?}", a.key),
            }
        }
        Ok(())
    }
    
    fn parse_xml(tileset : &mut Self, tsx_text : String, path : &String) -> Result<(), TiledError>{
        let mut reader = Reader::from_str(&tsx_text);
        loop {
            match reader.read_event() {
                Err(e) => {
                    return Err(TiledError::ParseError(e.to_string()));
                }
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {                
                    let mut s = String::new();
                    e.name().as_ref().read_to_string(&mut s).unwrap();
                    println!("tag: {}", s);
                    
                    match e.name().as_ref() {
                        b"tileset" => Self::parse_tileset_attribs(tileset, collect_attribs(&e)?)?,
                        _ => println!("unrecognized tag {:?}", e.name()),
                    }
                },
                Ok(Event::Empty(e)) => {
                    let mut s = String::new();
                    e.name().as_ref().read_to_string(&mut s).unwrap();
                    println!("empty tag: {}", s);
                    match e.name().as_ref() {
                        b"image" => Self::parse_image_attribs(tileset, collect_attribs(&e)?, path)?,
                        _ => println!("unrecognized empty tag {:?}", e.name()),
                    }
                }
                _ => (),
            }
        }
        
        Ok(())
    }
    
    pub fn new(attribs : Vec<Attribute>, path : String) -> Result<Tileset, TiledError> {
        let mut tmx_path = path.clone();
        let mut tileset = Self::blank();
        for a in attribs {
            match a.key.as_ref() {
                b"firstgid" => tileset.first_tile_id = get_value(&a.value)?,
                b"source" => {
                    Self::parse_xml(
                        &mut tileset,
                        read_file_to_string( {
                            tmx_path.push_str(get_string(&a.value)?);
                            &tmx_path
                        })?,
                        &path
                    )?;
                }
                _  => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        
        Ok(tileset)
    }
}
