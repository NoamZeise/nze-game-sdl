
use std::path::{Path, PathBuf};

use super::{helper::*, LayerData, Colour};
use super::error::TiledError;
use super::Properties;
use geometry::{Rect, Vec2};

use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, BytesText};
use quick_xml::reader::Reader;

pub struct ObjData {
    pub id: u32,
    pub name: String,
    pub type_name: String,
    pub visible: bool,
}

pub struct Obj {
    pub props : Properties,
    pub rect : Rect,
    pub rotation: f64,
    pub info: ObjData,
    poly : Option<Box<Poly>>,
    text : Option<Box<Text>>,
    point: bool,
    ellipse: bool,
    pub template: Option<PathBuf>,
}

pub struct Poly {
    pub points : Vec<Vec2>,
    pub obj : Obj,
    pub closed : bool,
}

pub type Point = Obj;
pub type Ellipse = Obj;

#[derive(Eq, PartialEq)]
pub enum TextHorizontalAlign {
    Left,
    Center,
    Right,
    Justify
}

#[derive(Eq, PartialEq)]
pub enum TextVerticalAlign {
    Top,
    Center,
    Bottom
}

pub struct Text {
    pub obj: Obj,
    pub text: String,
    pub font_family: String,
    pub pixel_size: u32,
    pub wrap: bool,
    pub bold: bool,
    pub italic: bool,
    pub horizontal_align : TextHorizontalAlign,
    pub vertical_align : TextVerticalAlign,
    pub colour: Colour,
}

pub struct ObjGroup {
    pub props : Properties,
    pub objs  : Vec<Obj>,
    pub polys : Vec<Poly>,
    pub points: Vec<Point>,
    pub ellipse: Vec<Ellipse>,
    pub text: Vec<Text>,
    pub info: LayerData,
    path: PathBuf,
}

impl ObjData {
    pub(crate) fn blank() -> ObjData {
        ObjData {
            id: 0,
            name: String::new(),
            type_name: String::new(),
            visible: true,
        }
    }
}

impl Obj {
    fn blank() -> Obj {
        Obj { props: Properties::blank(), rect: Rect::new(0.0, 0.0, 0.0, 0.0), info: ObjData::blank(), poly: None, text: None, rotation: 0.0, point: false, ellipse: false, template: None}
    }
    
    pub(crate) fn new(attribs : Vec<Attribute>, reader: Option<&mut Reader<&[u8]>>, path: &Path) -> Result<Obj, TiledError> {
        let mut obj = Obj::blank();
        obj.parse_attribs(attribs)?;
        if let Some(template) = &obj.template {
            let path: PathBuf = [path, template].iter().collect();
            let file = &read_file_to_string(&path)?;
            let mut reader = Reader::from_str(&file);
            parse_xml(&mut obj, &mut reader)?;
        }
        if let Some(reader) = reader {
            parse_xml(&mut obj, reader)?;
        }
        Ok(obj)
    }
    
    fn parse_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"x" => self.rect.x = get_value(&a.value)?,
                b"y" => self.rect.y = get_value(&a.value)?,
                b"width" => self.rect.w = get_value(&a.value)?,
                b"height" => self.rect.h = get_value(&a.value)?,
                b"id" => self.info.id = get_value(&a.value)?,
                b"name" => self.info.name = get_string(&a.value)?.to_string(),
                b"type" => self.info.type_name = get_string(&a.value)?.to_string(),
                b"visible" => self.info.visible = get_string(&a.value)? == "1",
                b"rotation" => self.rotation = get_value(&a.value)?,
                b"template" => self.template = Some(PathBuf::from(get_string(&a.value)?)),
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }
}

impl HandleXml for Obj {
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"template" => (),
            b"object" => self.parse_attribs(collect_attribs(&e)?)?,
            b"properties" => parse_xml(&mut self.props, reader)?,
            b"text" => self.text = Some(Box::new(Text::new(collect_attribs(&e)?, reader)?)),
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"object" => self.parse_attribs(collect_attribs(&e)?)?,
            b"polyline" => self.poly = Some(Box::new(Poly::new(collect_attribs(&e)?, false)?)),
            b"polygon" => self.poly = Some(Box::new(Poly::new(collect_attribs(&e)?, true)?)),
            b"ellipse" => self.ellipse = true,
            b"point" => self.point = true,
            _ => println!("unrecognized empty tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        "object"
    }
}

impl Poly {
    fn blank() -> Poly {
        Poly { points: Vec::new(), obj: Obj::blank(), closed: false }
    }
    fn parse_attribs(&mut self, attribs : Vec<Attribute>, closed : bool) -> Result<(), TiledError> {
        self.closed = closed;
        for a in attribs {
            match a.key.as_ref() {
                b"points" => {
                    let points = get_string(&a.value)?;
                    for pair in points.split(' ') {
                        let (x, y) = match pair.split_once(',') {
                            Some((x, y)) => match (x.parse(), y.parse()) {
                                (Ok(x), Ok(y)) => (x, y),
                                _ => { return Err(TiledError::ParseError(String::from("failed to parse poly points to floats"))); },
                            },
                            _ => { return Err(TiledError::MissingPoint()); },
                        };
                        self.points.push(Vec2::new(x, y));
                    }
                }
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }
    pub(crate) fn new(attribs : Vec<Attribute>, closed : bool) -> Result<Poly, TiledError> {
        let mut poly = Poly::blank();
        poly.parse_attribs(attribs, closed)?;
        Ok(poly)
    }
}

impl Text {
    fn blank() -> Text {
        Text {
            obj: Obj::blank(),
            text: String::new(),
            font_family: String::from("sans-serif"),
            pixel_size: 16,
            wrap: false, bold: false, italic: false,
            horizontal_align: TextHorizontalAlign::Left, vertical_align: TextVerticalAlign::Top,
            colour : Colour { r: 0, g: 0, b: 0, a:255 }
        }
    }
    fn parse_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"fontfamily" => self.font_family = get_string(&a.value)?.to_string(),
                b"pixelsize" => self.pixel_size = get_value(&a.value)?,
                b"wrap" => self.wrap = get_value::<i32>(&a.value)? == 1,
                b"bold" => self.bold = get_value::<i32>(&a.value)? == 1,
                b"italic" => self.italic = get_value::<i32>(&a.value)? == 1,
                b"halign" => self.horizontal_align = match get_string(&a.value)? {
                    "justify" => TextHorizontalAlign::Justify,
                    "right" => TextHorizontalAlign::Right,
                    "center" => TextHorizontalAlign::Center,
                    "left" => TextHorizontalAlign::Left,
                    _ => {
                        return Err(TiledError::ParseError(String::from("text alignment not recognized")));
                    },
                },
                b"valign" => self.vertical_align = match get_string(&a.value)? {
                    "top" => TextVerticalAlign::Top,
                    "bottom" => TextVerticalAlign::Bottom,
                    "center" => TextVerticalAlign::Center,
                    _ => {
                        return Err(TiledError::ParseError(String::from("text alignment not recognized")));
                    },
                },
                b"color" => self.colour = get_colour(&a.value)?,
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }
    pub(crate) fn new(attribs : Vec<Attribute>, reader: &mut Reader<&[u8]>) -> Result<Text, TiledError> {
        let mut text = Text::blank();
        text.parse_attribs(attribs)?;
        parse_xml(&mut text, reader)?;
        Ok(text)
    }
}

impl HandleXml for Text {
    fn text(&mut self, e : &BytesText) -> Result<(), TiledError> {
        let data = match e.unescape() {
            Ok(s) => s,
            Err(_) => { return Err(TiledError::ParseError(String::from("text data could not be retrieved"))); },
        };
        self.text = data.to_string();
        Ok(())
    }
    fn self_tag() -> &'static str {
        "text"
    }
}

impl ObjGroup {    
    fn blank() -> ObjGroup {
        ObjGroup {
            props: Properties::blank(),
            objs: Vec::new(),
            polys: Vec::new(),
            points: Vec::new(),
            ellipse: Vec::new(),
            text: Vec::new(),
            info: LayerData::new(),
            path: PathBuf::new(),
        }
    }
    fn parse_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            if let Some(()) = self.info.handle_attrib(&a)? {
                println!("warning: unrecognized atrribute {:?}", a.key);
            }
        }
        Ok(())
    }
    pub(crate) fn new(attribs : Vec<Attribute>, reader: &mut Reader<&[u8]>, path: &Path, layer_index: u32) -> Result<ObjGroup, TiledError> {
        let mut og = ObjGroup::blank();
        og.path = path.to_path_buf();
        og.parse_attribs(attribs)?;
        parse_xml(&mut og, reader)?;
        let mut obj_list : Vec::<Obj> = Vec::new();
        while og.objs.len() > 0{
            let mut obj = og.objs.pop().unwrap();
            if let Some(mut poly) = obj.poly.take() {
                poly.obj = obj;
                og.polys.push(*poly);
            } else {
                if let Some(mut text) = obj.text.take() {
                    text.obj = obj;
                    og.text.push(*text);
                } else {
                    if obj.ellipse {
                        og.ellipse.push(obj);
                    } else if obj.point {
                        og.points.push(obj);
                    } else {
                        obj_list.push(obj);
                    }
                }
            }
        }
        og.objs = obj_list;
        og.info.layer_position = layer_index;
        Ok(og)
    }
}

impl HandleXml for ObjGroup {
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"properties" => parse_xml(&mut self.props, reader)?,
            b"object" => self.objs.push(Obj::new(collect_attribs(&e)?, Some(reader), &self.path)?),
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
             b"object" => self.objs.push(Obj::new(collect_attribs(&e)?, None, &self.path)?),
            _ => println!("unrecognized empty tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        "objectgroup"
    }
}
