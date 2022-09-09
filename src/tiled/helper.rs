use std::fs::File;
use std::io::Read;
use core;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;

use super::error::TiledError;

pub fn read_file_to_string(filename : &str) -> Result<String, TiledError> {
    let mut file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            return Err(TiledError::FileReadError(filename.to_string(), e.to_string()));
        }
    };
    
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => (),
        Err(e) => {
            return Err(TiledError::FileReadError(filename.to_string(), e.to_string()));
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
