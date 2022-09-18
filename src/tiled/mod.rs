use crate::geometry::{Rect, Vec2};

use std::collections::HashMap;
use std::io::Read;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

mod tileset;
mod layer;
mod helper;
use helper::*;
pub mod error;
use error::TiledError;

pub struct Properties {
    pub booleans : HashMap<String, bool>,
    pub integers : HashMap<String, i64>,
}

pub struct Layer {
    pub props : Properties,
    pub tiles : Vec<u32>,
}

pub struct Obj {
    pub props : Properties,
    pub rect : Rect,
}

pub struct Poly {
    pub points : Vec<Vec2>,
    pub obj : Obj,
    pub closed : bool,
}

pub struct ObjGroup {
    pub props : Properties,
    pub objs  : Vec<Obj>,
    pub polys : Vec<Poly>,
}

pub struct ImageLayer {
    pub image_path : String,
    pub position  : Vec2,
}

pub struct Colour {
    pub r : u32,
    pub g : u32,
    pub b : u32,
    pub a : u32,
}

pub struct Text {
    pub obj : Obj,
    pub colour : Colour,
    pub text : String,
    pub pixel_size : u32,
    pub wrap : i32,
    pub font_family : String,
}

pub struct Tileset {
    pub first_tile_id : u32,
    pub name : String,
    pub tile_width : u32,
    pub tile_height : u32,
    pub tile_count : u32,
    pub column_count : u32,

    pub margin : u32,
    pub spacing : u32,

    pub image_path : String,
    pub image_width : u32,
    pub image_height : u32,
}

pub enum Orientation {
    Orthogonal,
    Isometric,
    IsometricStaggered,
    HexagonalStaggered,
}

pub struct Map {
    pub width : u32,
    pub height : u32,
    pub tile_width : u32,
    pub tile_height : u32,
    pub total_tiles : u32,
    pub infinite : bool,
    pub orientation : Orientation,

    pub tilesets : Vec<Tileset>,
    pub layers : Vec<Layer>,
    pub obj_groups : Vec<ObjGroup>,
    pub img_layers : Vec<ImageLayer>,
    pub texts : Vec<Text>,
}

impl Map {
    pub fn new(filename : &str) -> Result<Map, TiledError> {
        let path = match filename.rsplit_once('/') {
            Some((path, _)) => path,
            None => "",
        };
        let mut path = path.to_owned();
        path.push('/');
        Self::parse_xml(
            read_file_to_string(filename)?,
            &path
        )
    }

    fn blank_map() -> Map {
        Map {
            width : 0,
            height : 0,
            tile_width : 0,
            tile_height : 0,
            total_tiles : 0,
            infinite : false,
            orientation : Orientation::Orthogonal,

            tilesets : Vec::new(),
            layers : Vec::new(),
            obj_groups : Vec::new(),
            img_layers : Vec::new(),
            texts : Vec::new(),
        }
    }

    fn parse_map_attribs(map : &mut Map, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"width" => map.width = get_value(&a.value)?,
                b"height" => map.height = get_value(&a.value)?,
                b"tilewidth" => map.tile_width = get_value(&a.value)?,
                b"tileheight" => map.tile_height = get_value(&a.value)?,
                b"infinite" => map.infinite = get_value::<u32>(&a.value)? == 1,
                b"orientation" => map.orientation =  match a.value.as_ref() {
                    b"orthogonal" => Orientation::Orthogonal,
                    b"isometric" => Orientation::Isometric,
                    b"staggard" => Orientation::IsometricStaggered,
                    b"hexagonal" => Orientation::HexagonalStaggered,
                    _ => panic!("unrecognized map orientation"),
                },
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }

    fn parse_xml(map_file_text : String, path : &str) -> Result<Map, TiledError> {
        let mut reader = Reader::from_str(&map_file_text);
        let mut map = Self::blank_map();
        loop {
            match reader.read_event() {
                Err(e) => {
                    return Err(TiledError::ParseError(e.to_string()));
                },
                Ok(Event::Eof) => break,
                Ok(Event::Start(e)) => {

                    let mut s = String::new();
                    e.name().as_ref().read_to_string(&mut s).unwrap();
                    println!("tag: {}", s);
                    
                    match e.name().as_ref() {
                        b"map" => Self::parse_map_attribs(&mut map, collect_attribs(&e)?)?,
                        b"layer" => map.layers.push(Layer::blank()), //add layer properly
                        _ => println!("unrecognized tag {:?}", e.name()),
                    }
                },
                Ok(Event::End(e)) => {
                    let mut s = String::new();
                    e.name().as_ref().read_to_string(&mut s).unwrap();
                    println!("end tag: {}", s);
                    
                }
                Ok(Event::Empty(e)) => {
                    let mut s = String::new();
                    e.name().as_ref().read_to_string(&mut s).unwrap();
                    println!("empty tag: {}", s);

                    match e.name().as_ref() {
                        b"tileset" => map.tilesets.push(
                            Tileset::new(collect_attribs(&e)?, String::from(path))?
                        ),
                        _ => println!("unrecognized empty tag {:?}", e.name()),
                    }
                    
                }
                
                _ => (),

            }

        }  

        Ok(map)
    }
}

#[cfg(test)]
mod tiled_tests {
    use super::*;
    #[test]
    fn test_map() {
        let map = Map::new("test-resources/test.tmx").unwrap();
        assert!(map.width == 4);
        assert!(map.height == 4);
        assert!(map.tile_width == 10);
        assert!(map.tile_height == 10);
        assert!(!map.infinite);
        assert!(match map.orientation {
            Orientation::Orthogonal => true,
            _ => false,
        });

        assert!(map.tilesets.len() == 1);
        assert!(map.tilesets[0].first_tile_id == 1);
        assert!(map.tilesets[0].tile_width == 10);
        assert!(map.tilesets[0].tile_height == 10);
        assert!(map.tilesets[0].spacing == 2);
        assert!(map.tilesets[0].margin == 5);
        assert!(map.tilesets[0].tile_count == 4);
        assert!(map.tilesets[0].column_count == 2);
        assert!(map.tilesets[0].image_path == "test-resources/test-tileset.png");
        assert!(map.tilesets[0].image_width == 32);
        assert!(map.tilesets[0].image_height == 32);

        assert!(map.layers.len() == 2);
        assert!(map.layers[0].props.booleans["collidable"] == false);
        

        assert!(false, "more to check");
    }
}
