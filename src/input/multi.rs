//! Map multiple buttons/input-types to a single button

use super::Controls;
use super::controller;
use super::mouse;
use super::keyboard::Key;
use nze_geometry::Vec2;

use std::collections::HashMap;

pub struct Joy {
    dir: Vec2,
    side: controller::Side,
    activation: f64,
    tolerance: f64,
    timeout: f64,
}



pub struct MultiButton {
    input: bool,
    prev_input: bool,
    key: Vec<Key>,
    mouse_btn: Vec<mouse::Button>,
    ctrl_btn: Vec<controller::Button>,
    joy: Vec<Joy>,
}

impl MultiButton {
    pub fn new() -> MultiButton {
        MultiButton {
            input: false,
            prev_input: false,
            key: Vec::new(),
            mouse_btn: Vec::new(),
            ctrl_btn: Vec::new(),
            joy: Vec::new()
        }
    }
    pub fn register_key(&mut self, k: Key) {
        self.key.push(k);
    }

    pub fn update(&mut self, controls: &Controls) {
        self.prev_input = self.input;
        self.input = false;

        for k in self.key.iter() {
            if controls.kb.down(*k) {
                self.input = true;
            }
        }
    }
}

pub struct MuliBtnRef {
    id: usize,
    
}

pub struct MulitInput {
    btns: Vec<MultiButton>,
}

impl MulitInput {
    pub fn new() -> MulitInput {
        MulitInput { btns: Vec::new() }
    }

    pub fn new_btn(&mut self) -> MuliBtnRef {
        self.btns.push(MultiButton::new());
        return MuliBtnRef { id: self.btns.len() - 1 }
    }

    pub fn hold(&self, btn: &MuliBtnRef) -> bool {
        self.btns[btn.id].input
    }

    pub fn update(&mut self, controls: &Controls) {
        for btn in self.btns.iter_mut() {
            btn.update(controls);
        }
    }
}
