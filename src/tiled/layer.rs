use super::{Layer, Properties};

use std::collections::HashMap;

use quick_xml::reader::Reader;

impl Layer {
    pub fn blank() -> Layer {
        Layer {
            props: Properties { booleans: HashMap::new(), integers: HashMap::new() },
            tiles: Vec::new(),
        }
    }
    pub fn new(reader : Reader::<R>) -> Layer {

    }

}
