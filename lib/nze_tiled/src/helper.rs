use std::fs::File;
use std::io::Read;
use std::path::Path;
use core;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event, BytesText};
use quick_xml::reader::Reader;

use super::Colour;
use super::error::TiledError;


pub fn read_file_to_string(filepath : &Path) -> Result<String, TiledError> {
    let mut file = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => {
            return Err(TiledError::FileReadError(filepath.to_string_lossy().to_string(), e.to_string()));
        }
    };
    
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => (),
        Err(e) => {
            return Err(TiledError::FileReadError(filepath.to_string_lossy().to_string(), e.to_string()));
        }
    };
    Ok(text)
}

pub fn get_string<'a>(data : &'a std::borrow::Cow<[u8]>) -> Result<&'a str, TiledError>  {
    match core::str::from_utf8(data) {
        Ok(v) => Ok(v),
        Err(_) => Err(TiledError::ParseBytesError())
    }
}

pub fn get_value<T : std::str::FromStr>(data : &std::borrow::Cow<[u8]>)  -> Result<T, TiledError> {
    match get_string(data)?.parse() {
        Ok(v) => Ok(v),
        Err(_) => Err(TiledError::ParseBytesError()),
    }
}

fn hex_from_str(txt : &str) -> Result<u8, TiledError> {
    match u8::from_str_radix(txt, 16) {
        Ok(v) => Ok(v),
        Err(_) => Err(TiledError::ParseError(String::from("hex value could not be parsed to integer"))),
    }
}

pub fn get_colour(data : &std::borrow::Cow<[u8]>)  -> Result<Colour, TiledError> {
    let mut col = Colour { r: 255, g: 255, b: 255, a: 255};
    let txt = get_string(data)?;
    let txt = match txt.strip_prefix("#") {
        Some(txt) => txt,
        None => { return Err(TiledError::ParseError(String::from("colour value didnt start with hash"))); },
    };
    let mut i = 0;
    if txt.len() == 6 {
        i = 0;
    } else if txt.len() == 8 {
        col.a = hex_from_str(&txt[i..i+2])?;
        i += 2;
    } else {
        return Err(TiledError::ParseError(String::from("colour value didn't have 6 or 8 chars")));
    }
    col.r = hex_from_str(&txt[i..i+2])?;
    i += 2;
    col.g = hex_from_str(&txt[i..i+2])?;
    i += 2;
    col.b = hex_from_str(&txt[i..i+2])?;
    Ok(col)
}

/*pub fn get_bool(data : &std::borrow::Cow<[u8]>)  -> Result<bool, TiledError> {
    match get_string(data)? {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(TiledError::ParseBytesError()),
    }
}*/
    
pub fn collect_attribs<'a>(byte_start: &'a BytesStart) -> Result<Vec::<Attribute<'a>>, TiledError> {
    let mut attribs : Vec<Attribute<'a>> = Vec::new();
    for a in byte_start.attributes() {
        attribs.push( match a {
            Ok(a) => a,
            Err(e) => {
                return Err(TiledError::ParseError(
                    format!("tag name: {:?} error: {}", byte_start.name(), e.to_string())
                ));
            }
        });
    }
    Ok(attribs)
}

pub trait HandleXml {
    fn start(&mut self, _ : &BytesStart, _: &mut Reader<&[u8]>) -> Result<(), TiledError> {
        Ok(())
    }
    fn empty(&mut self, _ : &BytesStart) -> Result<(), TiledError> {
        Ok(())
    }
    fn text(&mut self, _ : &BytesText) -> Result<(), TiledError> {
        Ok(())
    }
    fn self_tag() -> &'static str;
}

pub fn parse_xml<T : HandleXml>(this: &mut T, reader: &mut Reader::<&[u8]>) -> Result<(), TiledError> {
    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(TiledError::ParseError(e.to_string()));
            },
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                // let mut s = String::new();
                // e.name().as_ref().read_to_string(&mut s).unwrap();
                // println!("start tag: {}", s);
                
                <T as HandleXml>::start(this, &e, reader)?;
            },
            Ok(Event::End(e)) => {
               // let mut s = String::new();
              //  e.name().as_ref().read_to_string(&mut s).unwrap();
              //  println!("end tag: {}", s);
                if e.name().as_ref() == <T as HandleXml>::self_tag().as_bytes() {
                    return Ok(())
                }
            }
            Ok(Event::Text(e)) => {
                <T as HandleXml>::text(this, &e)?;
            }
            Ok(Event::Empty(e)) => {
                //  let mut s = String::new();
                //  e.name().as_ref().read_to_string(&mut s).unwrap();
                //  println!("empty tag: {}", s);
                
                <T as HandleXml>::empty(this, &e)?;
            }
            _ => (),
        } 
    }  

    Ok(())
}

