use std::collections::HashMap;

use geometry::Vec2;
use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController as sdlController;

use sdl2::controller::Axis;

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
                println!("axis: {}", self.controllers[which].axis(*axis));
                
            }
            _ => (),
        }
    }

    pub(super) fn update_controller_state(&self, controllers: &mut Vec<Controller>) {
        for c in controllers.iter_mut() {
            c.update(&self.controllers[&c.id]);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Controller {
    id: u32,
    pub joy1: Vec2,
    pub joy2: Vec2,
}

impl Controller {
    pub fn new(id: u32) -> Controller {
        Controller { id, joy1: Vec2::new(0.0, 0.0), joy2: Vec2::new(0.0, 0.0) }
    }

    fn update(&mut self, sdl_c: &sdlController) {
        self.update_axis(sdl_c);
    }

    fn update_axis(&mut self, sdl_c: &sdlController) {
        self.joy1 = Vec2::new(
                sdl_c.axis(Axis::LeftX) as f64 / i16::MAX as f64,
                sdl_c.axis(Axis::LeftY) as f64 / i16::MAX as f64,
            );
        self.joy2 = Vec2::new(
            sdl_c.axis(Axis::RightX) as f64 / i16::MAX as f64,
            sdl_c.axis(Axis::RightY) as f64 / i16::MAX as f64,
        )
    }
}
