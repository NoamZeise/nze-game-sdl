use super::{TiledError, HandleXml, helper::*, LayerData, Properties};

use quick_xml::{events::{BytesStart, attributes::Attribute}, Reader};

use std::path::PathBuf;

pub struct ImageLayer {
    pub image_path : PathBuf,
    pub width: u32,
    pub height: u32,
    pub repeat_x: bool,
    pub repeat_y: bool,
    pub info: LayerData,
    pub props: Properties,
}

impl ImageLayer {
    fn blank() -> ImageLayer {
        ImageLayer {
            image_path: PathBuf::new(),
            width: 0,
            height: 0,
            repeat_x: false,
            repeat_y: false,
            info: LayerData::new(),
            props: Properties::blank(),
        }
    }
    pub(crate) fn new(attribs : Vec<Attribute>, reader : &mut Reader<&[u8]>, layer_index: u32) -> Result<ImageLayer, TiledError> {
        let mut img_layer = Self::blank();
        img_layer.parse_base_attribs(attribs)?;
        parse_xml(&mut img_layer, reader)?;
        img_layer.info.layer_position = layer_index;
        Ok(img_layer)
    }

    fn parse_base_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            if let Some(()) = self.info.handle_attrib(&a)? {
                match a.key.as_ref() {
                    b"repeatx" => self.repeat_x = get_value::<i32>(&a.value)? == 1,
                    b"repeaty" => self.repeat_y = get_value::<i32>(&a.value)? == 1,
                    _ => println!("warning: unrecognized atrribute {:?}", a.key),
                }
            }
        }
        Ok(())
    }

    fn parse_image_attributes(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"source" => self.image_path.push(get_string(&a.value)?),
                b"width" => self.width = get_value(&a.value)?,
                b"height" => self.height = get_value(&a.value)?,
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
   }
    
}

impl HandleXml for ImageLayer {
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"image" => self.parse_image_attributes(collect_attribs(&e)?)?,
            _ => println!("unrecognized empty tag {:?}", e.name()),
        }
        Ok(())
    }
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"properties" => parse_xml(&mut self.props, reader)?,
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        "imagelayer"
    }
}
