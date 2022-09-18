
use super::helper::*;
use super::error::TiledError;
use super::{ObjGroup, Obj, Poly, Properties};
use crate::Rect;
use crate::geometry::Vec2;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::reader::Reader;

impl Obj {
    fn blank() -> Obj {
        Obj { props: Properties::blank(), rect: Rect::new(0.0, 0.0, 0.0, 0.0), id: 0, poly: None}
    }
    
    pub fn new(attribs : Vec<Attribute>, reader: Option<&mut Reader<&[u8]>>) -> Result<Obj, TiledError> {
        let mut obj = Obj::blank();
        obj.parse_attribs(attribs)?;
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
                b"id" => self.id = get_value(&a.value)?,
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }
}

impl HandleXml for Obj {
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"properties" => parse_xml(&mut self.props, reader)?,
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"polyline" => self.poly = Some(Box::new(Poly::new(collect_attribs(&e)?, false)?)),
            b"polygon" => self.poly = Some(Box::new(Poly::new(collect_attribs(&e)?, true)?)),
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
    pub fn new(attribs : Vec<Attribute>, closed : bool) -> Result<Poly, TiledError> {
        let mut poly = Poly::blank();
        poly.parse_attribs(attribs, closed)?;
        Ok(poly)
    }
}

impl ObjGroup {    
    fn blank() -> ObjGroup {
        ObjGroup { props: Properties::blank(), objs: Vec::new(), polys: Vec::new(), id: 0, name: String::new()}
    }
    fn parse_attribs(&mut self, attribs : Vec<Attribute>) -> Result<(), TiledError> {
        for a in attribs {
            match a.key.as_ref() {
                b"id" => self.id = get_value(&a.value)?,
                b"name" => self.name = get_string(&a.value)?.to_string(),
                _ => println!("warning: unrecognized atrribute {:?}", a.key),
            }
        }
        Ok(())
    }
    pub fn new(attribs : Vec<Attribute>, reader: &mut Reader<&[u8]>) -> Result<ObjGroup, TiledError> {
        let mut og = ObjGroup::blank();
        og.parse_attribs(attribs)?;
        parse_xml(&mut og, reader)?;
        let mut obj_list : Vec::<Obj> = Vec::new();
        while og.objs.len() > 0{
            let mut obj = og.objs.pop().unwrap();
            if let Some(mut poly) = obj.poly.take() {
                poly.obj = obj;
                og.polys.push(*poly);
                println!("added poly");
            } else {
                obj_list.push(obj);
            }
        }
        og.objs = obj_list;
        Ok(og)
    }
}

impl HandleXml for ObjGroup {
    fn start(&mut self, e : &BytesStart, reader: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        match e.name().as_ref() {
            b"properties" => parse_xml(&mut self.props, reader)?,
            b"object" => self.objs.push(Obj::new(collect_attribs(&e)?, Some(reader))?),
            _ => println!("unrecognized tag {:?}", e.name()),
        }
        Ok(())
    }
    fn empty(&mut self, e : &BytesStart) -> Result<(), TiledError> {
        match e.name().as_ref() {
             b"object" => self.objs.push(Obj::new(collect_attribs(&e)?, None)?),
            _ => println!("unrecognized empty tag {:?}", e.name()),
        }
        Ok(())
    }
    fn self_tag() -> &'static str {
        "objectgroup"
    }
}
