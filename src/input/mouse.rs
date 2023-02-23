//! used to query information about the state of the mouse input.
//! Query using `m` member of [crate::input::Controls]

use sdl2::event::Event;

/// represents the buttons on a mouse that sdl tracks the state of
pub type Button = sdl2::mouse::MouseButton;

use crate::geometry::Vec2;

use crate::Camera;

/// Holds mouse input info
#[derive(Copy, Clone)]
struct MouseStateHolder {
    pub x : i32,
    pub y : i32,
    pub pos: Vec2,
    pub cam_offset: Vec2,
    pub wheel: i32,
    pub left_click : bool,
    pub middle_click: bool,
    pub right_click : bool,
}

impl MouseStateHolder {
    fn new() -> Self {
        MouseStateHolder {
            x: 0,
            y: 0,
            pos: Vec2::zero(),
            cam_offset: Vec2::zero(),
            wheel: 0,
            left_click : false,
            middle_click : false,
            right_click : false,
        }
    }

    fn handle_event(&mut self, event: &Event) {
        let mut btn_down = false;
        let btn = match event {
            Event::MouseWheel { y, ..} => {
                self.wheel = *y;
                None
            },
            Event::MouseMotion { x, y, .. } => {
                self.x = *x;
                self.y = *y;
                None
            },
            Event::MouseButtonDown { mouse_btn, ..} => {
                btn_down = true;
                Some(mouse_btn)
            },
            Event::MouseButtonUp { mouse_btn, .. } => {
                btn_down = false;
                Some(mouse_btn)
            }
            _ => None,
        };
        match btn {
            Some(btn) => match btn {
                Button::Left => self.left_click = btn_down,
                Button::Middle => self.middle_click = btn_down,
                Button::Right => self.right_click = btn_down,
                _ => (),
            }
            None => (),
        }
    }

    fn query_mouse_btn(&self, mouse_btn: &Button) -> bool {
        match mouse_btn {
            Button::Left => self.left_click,
            Button::Middle => self.middle_click,
            Button::Right => self.right_click,
            _  => false,
        }
    }
}

/// Holds the mouse state and the previous state
///
/// can query if certain buttons are held or pressed,
/// updated by `Control`
pub struct Mouse {
    input: MouseStateHolder,
    prev_input: MouseStateHolder,
}

impl Mouse {
    pub(super) fn new() -> Mouse {
        Mouse {
            input: MouseStateHolder::new(),
            prev_input: MouseStateHolder::new(),
        }
    }

    pub(super) fn update(&mut self) {
        self.prev_input = self.input;
        self.input.wheel = 0;
    }

    pub(super) fn handle_event(&mut self, e: &Event) {
        self.input.handle_event(e);
    }

    pub(super) fn correct_pos_with_cam(&mut self, cam: &Camera) {
        self.input.cam_offset = cam.get_offset();
        self.input.pos = cam.window_to_cam_vec2(
            Vec2::new(self.input.x as f64, self.input.y as f64),
            Vec2::zero()
        );
    }

    /// Get the current direction of the mouse scroll wheel
    ///
    /// - `0`  if not scrolling
    /// - `1`  if scrolling up
    /// - `-1` if scrolling down
    pub fn wheel(&self) -> i32 {
        self.input.wheel
    }

    /// The position of the mouse in the screen corrected by the camera's scale.
    /// Not corrected by the camera's position, use `pos_corrected` for that.
    ///
    /// The camera offset/scale correction is done during `Render.event_loop`
    pub fn pos(&self) -> Vec2 {
        self.input.pos + self.input.cam_offset
    }

    /// The position of the mouse in the screen corrected by the camera's scale and
    /// by the camera's position based on the parallax value.
    ///
    /// The parallax matches the effect of the parallax field of `GameObject`.
    ///
    /// The camera offset/scale correction is done during `Render.event_loop`
    pub fn pos_corrected(&self, parallax: Vec2) -> Vec2 {
        self.input.pos + self.input.cam_offset * parallax
    }

    /// returns true if the mouse button is currently being held down
    pub fn hold(&self, mouse_btn: Button) -> bool {
        self.input.query_mouse_btn(&mouse_btn)
    }

    /// returns true if the mouse button was just pressed
    pub fn press(&self, mouse_btn: Button) -> bool {
        self.input.query_mouse_btn(&mouse_btn) && !self.prev_input.query_mouse_btn(&mouse_btn)
    }   
}
