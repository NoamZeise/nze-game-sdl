use super::*;

use quick_xml::events::{BytesStart, attributes::Attribute};
use quick_xml::Reader;


pub enum Orientation {
    Orthogonal,
    Isometric,
    IsometricStaggered,
    HexagonalStaggered,
}

pub enum RenderOrder {
    RightDown,
    RightUp,
    LeftDown,
    LeftUp,
}

pub struct MapMetadata {
    pub version : String,
    pub tiled_version : String,
    pub render_order : RenderOrder,
    pub next_layer_id : u32,
    pub next_object_id : u32,
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

    pub path : String,
    pub metadata : MapMetadata,
    // for counting what layer we are at
    current_layer: u32,
    
}

impl Map {
    pub fn new(filename : &str) -> Result<Map, TiledError> {
        let path = match filename.rsplit_once('/') {
            Some((path, _)) => path,
            None => "",
        };
        let mut path = path.to_owned();
        path.push('/');
        Self::load_and_parse_xml(
            read_file_to_string(filename)?,
            &path
        )
    }

    fn blank_map(path: String) -> Map {
        Map {
            width : 0,
            height : 0,
            tile_width : 0,
            tile_height : 0,
            total_tiles : 1,
            infinite : false,
            orientation : Orientation::Orthogonal,

            tilesets : Vec::new(),
            layers : Vec::new(),
            obj_groups : Vec::new(),
            img_layers : Vec::new(),
            texts : Vec::new(),
            path,
            metadata : MapMetadata {
                version: "".to_string(),
                tiled_version: "".to_string(),
                render_order: RenderOrder::RightDown,
                next_layer_id: 0,
                next_object_id: 0,
            },

            current_layer: 0,
        }
    }

    fn parse_map_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"width" => self.width = get_value(&a.value)?,
                b"height" => self.height = get_value(&a.value)?,
                b"tilewidth" => self.tile_width = get_value(&a.value)?,
                b"tileheight" => self.tile_height = get_value(&a.value)?,
                b"infinite" => self.infinite = get_value::<u32>(&a.value)? == 1,
                b"orientation" => self.orientation =  match a.value.as_ref() {
                    b"orthogonal" => Orientation::Orthogonal,
                    b"isometric" => Orientation::Isometric,
                    b"staggard" => Orientation::IsometricStaggered,
                    b"hexagonal" => Orientation::HexagonalStaggered,
                    _ => panic!("unrecognized map orientation"),
                },
                b"version" => self.metadata.version = get_string(&a.value)?.to_string(),
                b"tiledversion" => self.metadata.tiled_version = get_string(&a.value)?.to_string(),
                b"nextlayerid" => self.metadata.next_layer_id = get_value(&a.value)?,
                b"nextobjectid" => self.metadata.next_object_id = get_value(&a.value)?,
                b"renderorder" => self.metadata.render_order = match a.value.as_ref() {
                    b"right-down" => RenderOrder::RightDown,
                    b"right-up" => RenderOrder::RightUp,
                    b"left-down" => RenderOrder::LeftDown,
                    b"left-up" => RenderOrder::LeftUp,
                    _ => { return Err(TiledError::UnsupportedType()); },
                },
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }

    fn load_and_parse_xml(map_file_text : String, path : &str) -> Result<Map, TiledError> {
        let mut reader = Reader::from_str(&map_file_text);
        let mut map = Self::blank_map(path.to_string());
        parse_xml(&mut map, &mut reader)?;
        Ok(map)
    }
}

impl HandleXml for Map {
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            
            b"map" => self.parse_map_attribs(collect_attribs(&e)?)?,
            b"layer" => {
                self.layers.push(Layer::new(collect_attribs(&e)?, reader, self.current_layer)?);
                self.current_layer+=1;
            }, //add layer properly
            b"objectgroup" => {
                self.obj_groups.push(ObjGroup::new(collect_attribs(&e)?, reader, self.path.clone(), self.current_layer)?);
                self.current_layer += 1;
            },
            b"imagelayer" => {
                self.img_layers.push(ImageLayer::new(collect_attribs(&e)?, reader, self.current_layer)?);
                self.current_layer += 1;
            },
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"tileset" => {
                self.tilesets.push(
                    Tileset::new(collect_attribs(&e)?, self.path.clone())?
                );
                self.total_tiles += self.tilesets.last().unwrap().tile_count;
            },
            _ => println!("unrecognized empty tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        ""
    }
}
