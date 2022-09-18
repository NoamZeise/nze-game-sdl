use super::LayerTiles;
use super::{Layer, Properties, error::TiledError, helper::*};

use quick_xml::reader::Reader;
use quick_xml::events::{BytesStart, BytesText};
use quick_xml::events::attributes::Attribute;

impl Layer {
    fn blank() -> Layer {
        Layer {
            props: Properties::blank(),
            tiles: Vec::new(),
            width: 0,
            height: 0,
            id: 0,
            name : String::new(),
        }
    }
    pub fn new(attribs : Vec<Attribute>, reader: &mut Reader<&[u8]>) -> Result<Layer, TiledError> {
        let mut layer = Layer::blank();
        layer.parse_attribs(attribs)?;
        parse_xml(&mut layer, reader)?;
        Ok(layer)
    }
    fn parse_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
             match a.key.as_ref() {
                 b"width" => self.width = get_value(&a.value)?,
                 b"height" => self.height = get_value(&a.value)?,
                 b"id" => self.id = get_value(&a.value)?,
                 b"name" => self.name = get_string(&a.value)?.to_string(),
                 _ => println!("warning: unrecognized atrribute {:?}", a.key),
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
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
            _ => println!("unrecognized empty tag {:?}", e.name()),
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
