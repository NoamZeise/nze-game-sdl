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
use controller::Controller;
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
    /// time in seconds between last frame and this one
    pub frame_elapsed: f64,
    /// This value must be checekd by the user, e.g. break the game loop when this is true
    ///
    /// This is set to true by the window controls when a window close signal is recieved.
    ///
    /// This can also be set to true in your game loop to exit due to some other condition. 
    pub should_close: bool,
    /// update this value to change the deadzones of the controller axes
    pub axis_deadzone: f64,

    event_pump: EventPump,
    controllers : Vec<Controller>,
    prev_controllers: Vec<Controller>,
    controller_handler: ControllerHandler,
    prev_time: Instant,
}

impl Controls {
    pub fn new(context: &ContextSdl) -> Result<Controls, Error> {
        Ok(Controls {
            event_pump: init_err!(context.sdl_context.event_pump())?,
            kbm: KBM::new(),
            controller_handler: ControllerHandler::new(
                init_err!(context.sdl_context.game_controller())?
            ),
            frame_elapsed: 0.0,
            prev_time: Instant::now(),
            controllers: Vec::new(),
            axis_deadzone: 0.1,
            prev_controllers: Vec::new(),
            should_close: false,
        })
    }

    fn update_input_state(&mut self) {
        self.prev_controllers = self.controllers.clone();
        self.kbm.update();
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
            self.controller_handler.handle_event(&e, &mut self.controllers);
        }
        self.controller_handler.update_controller_state(&mut self.controllers);
        self.frame_elapsed = self.prev_time.elapsed().as_secs_f64();
        self.prev_time = Instant::now();
    }

    /// Update  using the sdl events that occured between the previous call.
    /// This should be called at the start of update.
    ///
    /// The mouse pos is adjusted using the camera.
    pub fn update(&mut self, cam: &Camera) {
        self.update_input_state();
        self.kbm.input.mouse.pos = cam.window_to_cam_vec2(
            Vec2::new(self.kbm.input.mouse.x as f64, self.kbm.input.mouse.y as f64)
        );
    }

    /// returns the number of currently connected controllers
    pub fn controller_count(&self) -> usize {
        self.controllers.len()
    }

    /// returns true if the button is being held down
    ///
    /// will return false if the controller index is out of range
    pub fn controller_hold(&self, index: usize, button: controller::Button) -> bool {
        if index >= self.controllers.len() { return false; }
        self.controllers[index].button[button as usize]
    }

    /// returns true if the controller button as just been pressed
    ///
    /// will return false if the controller index is out of range
    pub fn controller_press(&self, index: usize, button: controller::Button) -> bool {
        if index >= self.controllers.len() { return false; }
        if index >= self.prev_controllers.len() { return self.controllers[index].button[button as usize] }
        self.controllers[index].button[button as usize] &&
        !self.prev_controllers[index].button[button as usize]
    }

    /// get a 2d vector representing the direction of the joystick. each direction ranges from `0.0` to `1.0`
    ///
    /// retuns `0.0` if the controller isn't connected
    ///
    /// use [controller::Side] left and right to get the state of the two joysticks
    ///
    /// the `self.axis_deadzone` member of `Control` is automatically applied to the joystick values
    pub fn controller_joy(&self, index: usize, joy: controller::Side) -> Vec2 {
        if index >= self.controllers.len() { return Vec2::new(0.0, 0.0); }
        match joy {
            controller::Side::Left => vec_deadzone(
                self.controllers[index].left_joy, Vec2::new(self.axis_deadzone, self.axis_deadzone)
            ),
            controller::Side::Right => vec_deadzone(
                self.controllers[index].right_joy, Vec2::new(self.axis_deadzone, self.axis_deadzone)
            ),
        }
    }

    /// get a float representing the amount of travel the analogue trigger has gone through.
    ///
    /// values range from `0.0` to `1.0`
    ///
    /// returns `0.0` if the controller isn't connected
    ///
    /// the `side` parameter will give you the left or right analogue trigger
    ///
    /// the `self.axis_deadzone` member of `Control` is automatically applied to the trigger values
    pub fn controller_trigger(&self, index: usize, side: controller::Side) -> f64 {
        if index >= self.controllers.len() { return 0.0; }
        match side {
            controller::Side::Left => deadzone(self.controllers[index].left_trigger, self.axis_deadzone),
            controller::Side::Right => deadzone(self.controllers[index].right_trigger, self.axis_deadzone),
        }
    }
}

fn deadzone(val:f64,dz:f64) -> f64 {
    if val.abs() > dz { val } else { 0.0 }
}

fn vec_deadzone(v:Vec2, dz:Vec2) -> Vec2 {
    let mut  n_v = v;
    n_v.x = deadzone(v.x, dz.x);
    n_v.y = deadzone(v.y, dz.y);
    n_v
}
