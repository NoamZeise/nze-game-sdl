//! Used to get input from the Keyboard and Mouse
//!
//! processes sdl2 events and update structs that can be used to control the game.
//! `Control` is updated by render in `event_loop`

use sdl2::EventPump;
use sdl2::GameControllerSubsystem;
use std::time::Instant;

mod keyboard;
mod mouse;
pub mod controller;
pub mod macros;
pub use keyboard::KeyboardAndMouse;
pub use keyboard::Key;
pub use mouse::Mouse;
use controller::Controller;
use controller::ControllerHandler;

/// Holds info on input state and frame elapsed time
/// held and updated by `Render` at the start of each frame
pub struct Controls {
    /// current frame input state
    pub input: KeyboardAndMouse,
    /// previous frame input state
    pub prev_input: KeyboardAndMouse,
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
            input: KeyboardAndMouse::new(),
            prev_input: KeyboardAndMouse::new(),
            controller_handler: ControllerHandler::new(gcs),
            frame_elapsed: 0.0,
            prev_time: Instant::now(),
            controllers: Vec::new(),
            prev_controllers: Vec::new(),
            should_close: false,
        }
    }

    pub(crate) fn update(&mut self, event_pump: &mut EventPump) {
        self.prev_input = self.input;
        self.prev_controllers = self.controllers.clone();
        self.input.mouse.wheel = 0;
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
            self.input.handle_event(&e);
            self.controller_handler.handle_event(&e, &mut self.controllers);
        }
        self.controller_handler.update_controller_state(&mut self.controllers);
        self.frame_elapsed = self.prev_time.elapsed().as_secs_f64();
        self.prev_time = Instant::now();
    }
}
