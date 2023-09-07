//! Map multiple buttons across input methods to a single virtual button

use super::Controls;
use super::controller;
use super::mouse;
use super::keyboard::Key;
use nze_geometry::Vec2;

/// Defines a joystick and a direction.
/// This is passed to multibutton,
/// so that when the joystick is within
/// the activation threshold, the button
/// is pressed.
pub struct Joy {
    pub dir: Vec2,
    pub side: controller::Side,
    pub activation: f64,
}

impl Joy {
    pub fn new(dir: Vec2, side: controller::Side) -> Joy {
        Joy {
            dir,
            side,
            activation: 0.5
        }
    }
    
    fn hold(&self, index: usize, controls: &Controls) -> bool {
        let dir = controls.c.joy(index, self.side);
        let diff = self.dir - dir;
        let mag = diff.x.abs() + diff.y.abs();
        mag < self.activation
    }
}

/// Defines a virtual button that can be activated by
/// Multiple different inputs.
pub struct MultiButton {
    input: bool,
    prev_input: bool,
    key: Vec<Key>,
    mouse_btn: Vec<mouse::Button>,
    ctrl_btn: Vec<(usize, controller::Button)>,
    joy: Vec<(usize, Joy)>,
    /// reset pressed state when held for this amount of time
    /// 0.0 means it doesn't reset.
    timeout: f64,
    current_timer: f64,
}

impl MultiButton {
    pub fn new(timeout: f64) -> MultiButton {
        MultiButton {
            input: false,
            prev_input: false,
            key: Vec::new(),
            mouse_btn: Vec::new(),
            ctrl_btn: Vec::new(),
            joy: Vec::new(),
            timeout,
            current_timer: 0.0,
        }
    }
    
    pub fn register_key(&mut self, k: Key) {
        self.key.push(k);
    }

    pub fn register_mouse_btn(&mut self, b: mouse::Button) {
        self.mouse_btn.push(b);
    }

    pub fn register_controller_btn(&mut self, controller_index: usize, c: controller::Button) {
        self.ctrl_btn.push((controller_index, c));
    }

    pub fn register_joy(&mut self, controller_index: usize, j: Joy) {
        self.joy.push((controller_index, j));
    }

    pub fn update(&mut self, controls: &Controls) {
        self.prev_input = self.input;
        self.input = false;
        self.current_timer += controls.frame_elapsed;

        for k in self.key.iter() {
            self.input |=  controls.kb.hold(*k);
        }
        
        for b in self.mouse_btn.iter() {
            self.input |=  controls.m.hold(*b);
        }

        for (i, b) in self.ctrl_btn.iter() {
            self.input |= controls.c.hold(*i, *b);
        }

        for (i, j) in self.joy.iter_mut() {
            self.input |= j.hold(*i, &controls);
        }

        if !self.input {
            self.current_timer = 0.0;
        } else if self.timeout != 0.0 && self.current_timer > self.timeout {
            self.prev_input = false;
        }
    }
}

pub struct MuliBtnRef {
    id: usize,    
}

/// Holds MultiButtons to make it easier
/// to update them and query state using
/// `MultiButtonRef' references.
pub struct MultiInput {
    btns: Vec<MultiButton>,
}

impl MultiInput {
    pub fn new() -> MultiInput {
        MultiInput { btns: Vec::new() }
    }

    pub fn btn(&mut self, btn: &MuliBtnRef) -> &mut MultiButton {
        &mut self.btns[btn.id]
    }

    pub fn new_btn(&mut self) -> MuliBtnRef {
        self.btns.push(MultiButton::new(0.0));
        return MuliBtnRef { id: self.btns.len() - 1 }
    }

    pub fn add_btn(&mut self, mb: MultiButton) -> MuliBtnRef {
        self.btns.push(mb);
        return MuliBtnRef { id: self.btns.len() - 1 }
    }

    pub fn hold(&self, btn: &MuliBtnRef) -> bool {
        self.btns[btn.id].input
    }

    pub fn press(&self, btn: &MuliBtnRef) -> bool {
        self.btns[btn.id].input && !self.btns[btn.id].prev_input
    }

    pub fn update(&mut self, controls: &Controls) {
        for btn in self.btns.iter_mut() {
            btn.update(controls);
        }
    }
}
