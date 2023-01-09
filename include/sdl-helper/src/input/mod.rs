//! Used to get input from the Keyboard and Mouse
//!
//! processes sdl2 events and update structs that can be used to control the game.
//! `Control` is updated by render in `event_loop`
use sdl2::EventPump;
use sdl2::GameControllerSubsystem;
use std::time::Instant;

mod mouse;
pub mod controller;
pub mod keyboard;
pub mod keyboard_mouse;
use controller::Controller;
use controller::ControllerHandler;
use keyboard::KeyboardAndMouse;

use self::keyboard_mouse::KBM;

/// Holds info on input state and frame elapsed time
/// held and updated by `Render` at the start of each frame
pub struct Controls {
    /// current keyboard and mouse state
    pub kbm: KBM,
    /// time in seconds between last frame and this one
    pub frame_elapsed: f64,
    /// This value must be checekd by the user, e.g. break the game loop when this is true
    ///
    /// This is set to true by the window controls when a window close signal is recieved.
    ///
    /// This can also be set to true in your game loop to exit due to some other condition. 
    pub should_close: bool,

    pub controllers : Vec<Controller>,
    pub prev_controllers: Vec<Controller>,
    controller_handler: ControllerHandler,
    prev_time: Instant,
}

impl Controls {
    pub(crate) fn new(gcs: GameControllerSubsystem) -> Controls {
        Controls {
            kbm: KBM::new(),
            controller_handler: ControllerHandler::new(gcs),
            frame_elapsed: 0.0,
            prev_time: Instant::now(),
            controllers: Vec::new(),
            prev_controllers: Vec::new(),
            should_close: false,
        }
    }

    pub(crate) fn update(&mut self, event_pump: &mut EventPump) {
        self.prev_controllers = self.controllers.clone();
        self.kbm.update();
        for e in event_pump.poll_iter() {
            let win_ev = match &e {
                sdl2::event::Event::Window {
                    win_event:  w,
                    ..
                } => {
                    Some(w)
                }
                _ => None,
            };
            match win_ev {
                Some(w) => match w {sdl2::event::WindowEvent::Close => { self.should_close = true; }, _ => ()},
                _ => ()
            }
            self.kbm.handle_event(&e);
            self.controller_handler.handle_event(&e, &mut self.controllers);
        }
        self.controller_handler.update_controller_state(&mut self.controllers);
        self.frame_elapsed = self.prev_time.elapsed().as_secs_f64();
        self.prev_time = Instant::now();
    }

    /// returns the number of currently connected controllers
    pub fn controller_count(&self) -> usize {
        self.controllers.len()
    }

    /// returns true if the button is being held down
    ///
    /// will return false if the controller index is out of range
    pub fn controller_held(&self, index: usize, button: controller::Button) -> bool {
        if index >= self.controllers.len() { return false; }
        self.controllers[index].button[button as usize]
    }

    /// returns true if the controller button as just been pressed
    ///
    /// will return false if the controller index is out of range
    pub fn controller_pressed(&self, index: usize, button: controller::Button) -> bool {
        if index >= self.controllers.len() { return false; }
        if index >= self.prev_controllers.len() { return self.controllers[index].button[button as usize] }
        self.controllers[index].button[button as usize] &&
        !self.prev_controllers[index].button[button as usize]
    }
}
