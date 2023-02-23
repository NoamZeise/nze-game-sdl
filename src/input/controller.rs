//! Holds some enums that can be used to query controller state
//! using `c` memeber of  [crate::input::Controls]

use std::collections::HashMap;

use crate::geometry::Vec2;
use sdl2::{
    event::Event,
    GameControllerSubsystem,
    controller::Axis,
};

use sdl2::controller::GameController as sdlController;

/// represents each of the buttons on a standart xInput controller
pub type Button = sdl2::controller::Button;

/// represents each side of the controls, eg left joystick/trigger and right joystick/trigger
pub enum Side {
    Left,
    Right,
}

pub struct ControllerHandler {
    /// update this value to change the deadzones of the controller axes
    pub axis_deadzone: f64,
    controllers: Vec<Controller>,
    prev_controllers: Vec<Controller>,
    controller_subsystem: GameControllerSubsystem,
    sdl_controllers: HashMap<u32, sdlController>,
    input_changed: bool,
}

impl ControllerHandler {
    pub(super) fn new(controller_subsystem: GameControllerSubsystem) -> ControllerHandler {
        ControllerHandler {
            axis_deadzone: 0.1,
            controller_subsystem,
            sdl_controllers: HashMap::new(),
            controllers: Vec::new(),
            prev_controllers: Vec::new(),
            input_changed: false,
        }
    }

    pub(super) fn handle_event(&mut self, event: &Event) {
        match event {
            Event::ControllerDeviceAdded { which , .. } => { 
                if self.controller_subsystem.is_game_controller(*which) {
                    match self.controller_subsystem.open(*which) {
                        Ok(gc) => {
                            let id = gc.instance_id();
                            self.sdl_controllers.insert(id, gc);
                            println!("controller added: {}", id);
                            self.controllers.push(Controller::new(id));
                        },
                        Err(e) => eprintln!("error opening controller {:?}", e),
                    }
                } else {
                    eprintln!("warning: added device was not a game controller, unsupported");
                }
            },
            
            Event::ControllerDeviceRemoved { which , .. } => {
                println!("controller removed: {}", which);
                self.sdl_controllers.remove(which);
                self.controllers.retain(|c| c.id != *which);
            },
            
            Event::ControllerAxisMotion { /*which, axis, value,*/ .. } => {
                //println!("id: {} axis: {:?} value: {}", which, axis, value);
                self.input_changed = true;
            }
            
            Event::ControllerButtonDown { which, button, .. } => {
                //println!("down:  id: {} button: {:?}", which, button);
                self.input_changed = true;
                for c in self.controllers.iter_mut() {
                    if c.id == *which {
                        c.button[*button as usize] = true;
                    }
                }
            }
            
            Event::ControllerButtonUp { which, button, .. } => {
                //println!("up:  id: {} button: {:?}", which, button);
                self.input_changed = true;
                for c in self.controllers.iter_mut() {
                    if c.id == *which {
                        c.button[*button as usize] = false;
                    }
                }
            }
            _ => (),
        }
    }

    pub(super) fn set_previous_controller(&mut self) {
        if self.input_changed {
            self.input_changed = false;
            self.prev_controllers = self.controllers.clone();
        }
    }

    pub(super) fn update_controller_state(&mut self) {
        for c in self.controllers.iter_mut() {
            c.update(&self.sdl_controllers[&c.id]);
        }
    }

    fn _change_controller_mapping_text(&mut self, id: &u32, button_name: &str, button_code: &str) {
        let mut  mapping = self.sdl_controllers[id].mapping();
        let i = mapping.find(button_name).unwrap();
        mapping = mapping[0..i].to_string()
            + button_name + ":"
            + button_code + ","
            + mapping[i..].split_once(",").unwrap().1;
        self.controller_subsystem.add_mapping(&mapping).unwrap();
    }

    /// returns the number of currently connected controllers
    pub fn count(&self) -> usize {
        self.controllers.len()
    }

    /// returns true if the button is being held down
    ///
    /// will return false if the controller index is out of range
    pub fn hold(&self, index: usize, button: Button) -> bool {
        if index >= self.controllers.len() { return false; }
        self.controllers[index].button[button as usize]
    }

    /// returns true if the controller button as just been pressed
    ///
    /// will return false if the controller index is out of range
    pub fn press(&self, index: usize, button: Button) -> bool {
        if index >= self.controllers.len() { return false; }
        if index >= self.prev_controllers.len() {
            return self.controllers[index].button[button as usize]
        }
        self.controllers[index].button[button as usize] &&
            !self.prev_controllers[index].button[button as usize]
    }

    /// get a 2d vector representing the direction of the joystick.
    /// Each direction ranges from `0.0` to `1.0`.
    ///
    /// Retuns `0.0` if the controller isn't connected.
    ///
    /// Use [Side] left and right to get the state of the two joysticks.
    ///
    /// The `self.axis_deadzone` member of `Control` is automatically applied to the joystick values.
    pub fn joy(&self, index: usize, joy: Side) -> Vec2 {
        if index >= self.controllers.len() { return Vec2::new(0.0, 0.0); }
        match joy {
            Side::Left => vec_deadzone(
                self.controllers[index].left_joy, Vec2::new(self.axis_deadzone, self.axis_deadzone)
            ),
            Side::Right => vec_deadzone(
                self.controllers[index].right_joy, Vec2::new(self.axis_deadzone, self.axis_deadzone)
            ),
        }
    }

    /// Get a float representing the amount of travel the analogue trigger has gone through.
    ///
    /// Values range from `0.0` to `1.0`.
    ///
    /// Returns `0.0` if the controller isn't connected.
    ///
    /// The `side` parameter will give you the left or right analogue trigger.
    ///
    /// The `self.axis_deadzone` member of `Control` is automatically applied to the trigger values.
    pub fn trigger(&self, index: usize, side: Side) -> f64 {
        if index >= self.controllers.len() { return 0.0; }
        match side {
            Side::Left => deadzone(self.controllers[index].left_trigger, self.axis_deadzone),
            Side::Right => deadzone(self.controllers[index].right_trigger, self.axis_deadzone),
        }
    }
    
    /// Set the rumble values of the controller.
    ///
    /// If the controller does not support rumble, this does nothing.
    ///
    /// If you set the duration to `u32::MAX` the duration will be very short,
    /// so use some value smaller than that if you want it to rumble for a long time.
    pub fn rumble(&mut self, index: usize, low_motor: u16,
                                 high_motor: u16, duration: u32) {
        if index >= self.controllers.len() { return; }
        match self.sdl_controllers.get_mut(&self.controllers[index].id) {
            None => (),
            Some(c) => { match c.set_rumble(low_motor, high_motor, duration) {
                _ => () // There will be an error if controller doesn't support rumble,
                // We will ignore this, as rumble isn't usually essential to a game.
            };},
        };
    }
}

/// Apply deadzone for a given value.
fn deadzone(val:f64, dz:f64) -> f64 {
    if val.abs() > dz { val } else { 0.0 }
}

// Apply deadzone for a given vector.
fn vec_deadzone(v:Vec2, dz:Vec2) -> Vec2 {
    let mut  n_v = v;
    n_v.x = deadzone(v.x, dz.x);
    n_v.y = deadzone(v.y, dz.y);
    n_v
}

/// To get the size of our array of booleans for button states.
const CONTROLLER_BTN_MAX : usize = Button::Touchpad as usize + 1;

#[derive(Clone, Copy)]
pub(super) struct Controller {
    id: u32,
    pub left_joy: Vec2,
    pub right_joy: Vec2,
    pub left_trigger: f64,
    pub right_trigger: f64,
    pub button: [bool; CONTROLLER_BTN_MAX],
}

impl Controller {
    pub fn new(id: u32) -> Controller {
        Controller {
            id,
            left_joy: Vec2::new(0.0, 0.0),
            right_joy: Vec2::new(0.0, 0.0),
            left_trigger: 0.0,
            right_trigger: 0.0,
            button: [false; CONTROLLER_BTN_MAX],
        }
    }

    fn update(&mut self, sdl_c: &sdlController) {
        self.update_axis(sdl_c);
    }

    fn update_axis(&mut self, sdl_c: &sdlController) {
        self.left_joy = Vec2::new(
                i16_to_percent(sdl_c.axis(Axis::LeftX)),
                i16_to_percent(sdl_c.axis(Axis::LeftY))
            );
        self.right_joy = Vec2::new(
            i16_to_percent(sdl_c.axis(Axis::RightX)),
            i16_to_percent(sdl_c.axis(Axis::RightY))
        );
        self.right_trigger = i16_to_percent(sdl_c.axis(Axis::TriggerRight));
        self.left_trigger = i16_to_percent(sdl_c.axis(Axis::TriggerRight));
    }
}

fn i16_to_percent(val: i16) -> f64 {
    val as f64 / i16::MAX as f64
}

