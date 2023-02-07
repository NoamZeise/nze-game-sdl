//! Used to get input from the Keyboard and Mouse
//!
//! processes sdl2 events and update structs that can be used to control the game.
//! `Control` is updated by render in `event_loop`
use geometry::Vec2;
use sdl2::EventPump;
use std::time::Instant;

mod mouse;
pub mod controller;
pub mod keyboard;
use controller::ControllerHandler;
use keyboard::KBM;

use crate::Camera;
use crate::ContextSdl;
use crate::Error;
use crate::init_err;

/// Holds info on input state and frame elapsed time
///
/// `update` must be called each frame to have proper input information 
pub struct Controls {
    /// current keyboard and mouse state
    pub kbm: KBM,
    /// current controller state
    pub controller: ControllerHandler,
    /// time in seconds between last frame and this one
    pub frame_elapsed: f64,
    /// This value must be checked by the user, e.g. break the game loop when this is true
    ///
    /// This is set to true by the window controls when a window close signal is recieved.
    ///
    /// This can also be set to true in your game loop to exit due to some other condition. 
    pub should_close: bool,
    event_pump: EventPump,
    prev_time: Instant,
}

impl Controls {
    pub fn new(context: &ContextSdl) -> Result<Controls, Error> {
        Ok(Controls {
            event_pump: init_err!(context.sdl_context.event_pump())?,
            kbm: KBM::new(),
            controller: ControllerHandler::new(
                init_err!(context.sdl_context.game_controller())?
            ),
            frame_elapsed: 0.0,
            should_close: false,
            prev_time: Instant::now(),
        })
    }

    fn update_input_state(&mut self) {
        self.kbm.update();
        self.controller.set_previous_controller();
        for e in self.event_pump.poll_iter() {
            let win_ev = match &e {
                sdl2::event::Event::Window {
                    win_event:  w,
                    ..
                } => {
                    Some(w)
                },
                _ => None,
            };
            match win_ev {
                Some(w) => match w {sdl2::event::WindowEvent::Close => { self.should_close = true; }, _ => ()},
                _ => ()
            }
            self.kbm.handle_event(&e);
            self.controller.handle_event(&e);
        }
        self.controller.update_controller_state();
        self.frame_elapsed = self.prev_time.elapsed().as_secs_f64();
        self.prev_time = Instant::now();
    }

    /// Update  using the sdl events that occured between the previous call.
    /// This should be called at the start of update.
    ///
    /// The mouse pos is adjusted using the camera.
    pub fn update(&mut self, cam: &Camera) {
        self.update_input_state();
        self.kbm.input.mouse.cam_offset = cam.get_offset();
        self.kbm.input.mouse.pos = cam.window_to_cam_vec2(
            Vec2::new(self.kbm.input.mouse.x as f64, self.kbm.input.mouse.y as f64),
            Vec2::zero()
        );
    }
}

