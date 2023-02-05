//! Holds a struct and some enums that can be used to get keboard and mouse input state

use geometry::Vec2;
/// An enum of keyboard buttons used to index the array help by `KBMcontrols`
///
/// See `key!` macro in crate root for shorthand use of the key array
pub use sdl2::keyboard::Scancode as Key;

/// represents three buttons found on a standard mouse
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

use sdl2::event::Event;

use super::mouse::Mouse;

/// Holds the input state for a frame of event updates
///
/// Holds keyboard and mouse state
#[derive(Copy, Clone)]
pub(crate) struct KeyboardAndMouseStateHolder {
    /// an array indexed by [Key] enums as usize
    pub keys : [bool; Key::Num as usize],
    /// represents the mouse input state for this frame
    pub mouse : Mouse,
    character : Option<char>,
}

impl KeyboardAndMouseStateHolder {
    pub(crate) fn new() -> Self {
        KeyboardAndMouseStateHolder {
            keys      : [false; Key::Num as usize],
            mouse     : Mouse::new(),
            character : None
        }
    }

    pub(crate) fn handle_event(&mut self, event: &Event) {
        if event.is_keyboard() {
            self.handle_keyboard(event);
        } else if event.is_text() {
            self.handle_text(event);
        } else if event.is_mouse() {
            self.mouse.handle_mouse(event);
        } 
    }

    pub(crate) fn get_typed_character(&mut self) -> Option<char> {
        match self.character {
            Some(c) => {
                self.character = None;
                Some(c)
            },
            None => None
        }
    }

    fn handle_keyboard(&mut self, event : &Event) {
        let mut key_down = false;
        let key = match event {
            Event::KeyDown {
                scancode: k,
                ..
            } => {
                key_down = true;
                k
            },
            Event::KeyUp {
                scancode: k,
                ..
            } => k,
            _ => &None
        };
        match key {
            Some(k) => self.keys[*k as usize] = key_down,
            _ => {}
        }
    }

    fn handle_text(&mut self, event : &Event) {
        self.character = match event {
            Event::TextInput { text : t, ..} => {
                if t.len() > 0 {
                    Some(t.chars().nth(0).unwrap())
                } else {
                    None
                }
            },
            _ => None,
        }
    }

    pub fn query_mouse_btn(&self, mouse_btn: &MouseButton) -> bool {
        match mouse_btn {
            MouseButton::Left => self.mouse.left_click,
            MouseButton::Middle => self.mouse.middle_click,
           MouseButton::Right => self.mouse.right_click,
        }
    }
}

/// A type held by `Control` that can be queried for the state of the keyboard and mouse
pub struct KBM {
    // the main render struct updates the mouse pos using the camera, so it needs access to this member
    pub(crate) input: KeyboardAndMouseStateHolder,
    prev_input: KeyboardAndMouseStateHolder,
}

impl KBM {
    pub(super) fn new() -> KBM {
        KBM {
            input: KeyboardAndMouseStateHolder::new(),
            prev_input: KeyboardAndMouseStateHolder::new(),
        }
    }
    pub(super) fn update(&mut self) {
        self.input.mouse.wheel = 0;
        self.prev_input = self.input;
    }

    pub(super) fn handle_event(&mut self, e: &Event) {
        self.input.handle_event(e);
    }

    /// returns true if the key is being held down
    pub fn down(&self, key: Key) -> bool {
        self.input.keys[key as usize]
    }

    /// returns true if the key was just pressed
    pub fn press(&self, key: Key) -> bool {
        self.input.keys[key as usize] && !self.prev_input.keys[key as usize]
    }

    /// Get the last character typed by the keyboard in text input mode, or `None` if nothing was typed
    ///
    /// Getting a character causes the current character to be set to `None`,
    ///
    /// To enable typing: use `SdlContext.set_text_input` function.
    pub fn get_typed_character(&mut self) -> Option<char> {
        self.input.get_typed_character()
    }

    /// Get the current direction of the mouse scroll wheel
    ///
    /// - `0`  if not scrolling
    /// - `1`  if scrolling up
    /// - `-1` if scrolling down
    pub fn mouse_wheel(&self) -> i32 {
        self.input.mouse.wheel
    }

    /// The position of the mouse in the screen corrected by the camera's scale, but not the camera's position
    ///
    /// The camera offset correction is done during `Render.event_loop`
    pub fn mouse_pos(&self) -> Vec2 {
        self.input.mouse.pos
    }

    pub fn mouse_pos_cam_off(&self) -> Vec2 {
        self.input.mouse.pos_cam
    }

    /// returns true if the mouse button is currently being held down
    pub fn mouse_hold(&self, mouse_btn: MouseButton) -> bool {
        self.input.query_mouse_btn(&mouse_btn) 
    }

    /// returns true if the mouse button was just pressed
    pub fn mouse_press(&self, mouse_btn: MouseButton) -> bool {
        self.input.query_mouse_btn(&mouse_btn) && !self.prev_input.query_mouse_btn(&mouse_btn)
    }   
}

