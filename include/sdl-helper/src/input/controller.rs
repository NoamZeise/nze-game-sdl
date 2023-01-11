//! Holds some enums that can be used to query controller state quing `Control`

use std::collections::HashMap;

use geometry::Vec2;
use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController as sdlController;
use sdl2::controller::Axis;

/// represents each of the buttons on a standart xInput controller
pub use sdl2::controller::Button as Button;

/// represents each side of the controls, eg left joystick/trigger and right joystick/trigger
pub enum Side {
    Left,
    Right,
}

pub(crate) struct ControllerHandler {
    controller_subsystem: GameControllerSubsystem,
    controllers: HashMap<u32, sdlController>,
}

impl ControllerHandler {
    pub(super) fn new(controller_subsystem: GameControllerSubsystem) -> ControllerHandler {
        ControllerHandler {  controller_subsystem, controllers: HashMap::new()}
    }

    pub(super) fn handle_event(&mut self, event: &Event, controllers: &mut Vec<Controller>) {
        //println!("num joysticks: {}", self.controller_subsystem.num_joysticks().unwrap());
        match event {
            Event::ControllerDeviceAdded { which , .. } => { 
                if self.controller_subsystem.is_game_controller(*which) {
                    match self.controller_subsystem.open(*which) {
                        Ok(gc) => {
                            let id = gc.instance_id();
                            self.controllers.insert(id, gc);
                            println!("controller added: {}", id);
                            controllers.push(Controller::new(id));
                        },
                        Err(e) => println!("error opening controller {:?}", e),
                    }
                }
            },
            Event::ControllerDeviceRemoved { which , .. } => {
                println!("controller removed: {}", which);
                self.controllers.remove(which);
                controllers.retain(|c| c.id != *which);
            },
            
            Event::ControllerAxisMotion { which, axis, value, .. } => {
                println!("id: {} axis: {:?} value: {}", which, axis, value);
            }
            Event::ControllerButtonDown { which, button, .. } =>
            {
                println!("down:  id: {} button: {:?}", which, button);
                for c in controllers.iter_mut() {
                    if c.id == *which {
                        c.button[*button as usize] = true;
                    }
                }
            }
            Event::ControllerButtonUp { which, button, .. } => {
                println!("up:  id: {} button: {:?}", which, button);
                for c in controllers.iter_mut() {
                    if c.id == *which {
                        c.button[*button as usize] = false;
                    }
                }
            }
            _ => (),
        }
    }

    pub(super) fn update_controller_state(&self, controllers: &mut Vec<Controller>) {
        for c in controllers.iter_mut() {
            c.update(&self.controllers[&c.id]);
        }
    }

    fn _change_controller_mapping_text(&mut self, id: &u32, button_name: &str, button_code: &str) {
        let mut  mapping = self.controllers[id].mapping();
        let i = mapping.find(button_name).unwrap();
        mapping = mapping[0..i].to_string() + button_name + ":" + button_code + "," + mapping[i..].split_once(",").unwrap().1;
        self.controller_subsystem.add_mapping(&mapping).unwrap();
    }
}

#[derive(Clone, Copy)]
pub(super) struct Controller {
    id: u32,
    pub left_joy: Vec2,
    pub right_joy: Vec2,
    pub left_trigger: f64,
    pub right_trigger: f64,
    pub button: [bool; Button::DPadRight as usize + 1],
}

impl Controller {
    pub fn new(id: u32) -> Controller {
        Controller {
            id,
            left_joy: Vec2::new(0.0, 0.0),
            right_joy: Vec2::new(0.0, 0.0),
            left_trigger: 0.0,
            right_trigger: 0.0,
            button: [false; Button::DPadRight as usize + 1],
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

