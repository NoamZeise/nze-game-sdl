use super::{Properties, error::TiledError, helper::*, Colour};
use nze_geometry::*;

use quick_xml::reader::Reader;
use quick_xml::events::{BytesStart, BytesText};
use quick_xml::events::attributes::Attribute;



pub struct LayerData {
    pub id: u32,
    pub name: String,
    pub visible: bool,
    pub locked: bool,
    pub opacity: f64,
    pub colour : Colour,
    pub tint : Colour,
    pub index_draw_order: bool,
    pub parallax: Vec2,
    pub offset: Vec2,
    pub layer_position: u32,
} 

pub type LayerTiles = Vec<u32>;

pub struct Layer {
    pub props : Properties,
    pub tiles : LayerTiles,
    pub width : i64,
    pub height: i64,
    pub info: LayerData,
}


impl Layer {
    fn blank() -> Layer {
        Layer {
            props: Properties::blank(),
            tiles: Vec::new(),
            width: 0,
            height: 0,
            info: LayerData::new(),
        }
    }
    pub(crate) fn new(attribs : Vec<Attribute>, reader: &mut Reader<&[u8]>, layer_index: u32) -> Result<Layer, TiledError> {
        let mut layer = Layer::blank();
        layer.parse_attribs(attribs)?;
        parse_xml(&mut layer, reader)?;
        layer.info.layer_position = layer_index;
        Ok(layer)
    }
    fn parse_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            if let Some(()) = self.info.handle_attrib(&a)? {
                match a.key.as_ref() {
                    b"width" => self.width = get_value(&a.value)?,
                    b"height" => self.height = get_value(&a.value)?,
                    _ => println!("warning: unrecognized atrribute {:?}", a.key),
                }
            }
        }
        Ok(())
    }
}

impl HandleXml for Layer {
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"data" => parse_xml(&mut self.tiles, reader)?,
            b"properties" => parse_xml(&mut self.props, reader)?,
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        "layer"
    }
    
 }

impl HandleXml for LayerTiles {
    fn text(&mut self, e : &BytesText) -> Result<(), TiledError> {
        let data = match e.unescape() {
            Ok(s) => s,
            Err(_) => { return Err(TiledError::ParseError(String::from("tile data in layer could not be retrieved"))); },
        };
        for num in data.split(",") {
            self.push( match num.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    return Err(TiledError::ParseError(
                        String::from("tile data could not be parsed to an integer: ") + num)
                    );
                },
            });
        }
        Ok(())
    }
    
    fn self_tag() -> &'static str {
        "data"
    }
}

impl LayerData {
    pub(crate) fn new() -> LayerData {
        LayerData {
            id: 0,
            name: String::from(""),
            visible: true,
            locked: false,
            opacity: 1.0,
            colour: Colour{r: 255, g: 255, b: 255, a: 255 },
            tint: Colour { r: 255, g: 255, b: 255, a: 255 },
            index_draw_order: false,
            parallax: Vec2::new(1.0, 1.0),
            offset: Vec2::new(0.0, 0.0),
            layer_position: 0,
        }
    }
    pub(crate) fn handle_attrib(&mut self, a : &Attribute) -> Result<Option<()>, TiledError> {
        match a.key.as_ref() {
            b"id" => self.id = get_value(&a.value)?,
            b"name" => self.name = get_string(&a.value)?.to_string(),
            b"visible" => self.visible = get_value::<i32>(&a.value)? == 1,
            b"locked" => self.locked = get_value::<i32>(&a.value)? == 1,
            b"opacity" => self.opacity = get_value(&a.value)?,
            b"color" => self.colour = get_colour(&a.value)?,
            b"tintcolor" => self.tint = get_colour(&a.value)?,
            b"draworder" => self.index_draw_order = get_string(&a.value)? == "index",
            b"offsetx" => self.offset.x = get_value(&a.value)?,
            b"offsety" => self.offset.y = get_value(&a.value)?,
            b"parallaxx" => self.parallax.x = get_value(&a.value)?,
            b"parallaxy" => self.parallax.y = get_value(&a.value)?,
            _ => { return Ok(Some(())); }
        }
        Ok(None)
    }
}
