use super::KeyboardAndMouse;
use super::keyboard;
use geometry::Vec2;
use sdl2::event::Event;



pub struct KBM {
    pub(crate) input: KeyboardAndMouse,
    prev_input: KeyboardAndMouse,
}

impl KBM {
    pub(super) fn new() -> KBM {
        KBM {
            input: KeyboardAndMouse::new(),
            prev_input: KeyboardAndMouse::new(),
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
    pub fn down(&self, key: keyboard::Key) -> bool {
        self.input.keys[key as usize]
    }

    /// returns true if the key was just pressed
    pub fn press(&self, key: keyboard::Key) -> bool {
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

    /// returns true if the button is currently being held down
    pub fn mouse_btn_hold(&self, mouse_btn: keyboard::MouseButton) -> bool {
        match mouse_btn {
            keyboard::MouseButton::Left => self.input.mouse.left_click,
            keyboard::MouseButton::Middle => self.input.mouse.middle_click,
            keyboard::MouseButton::Right => self.input.mouse.right_click,
        }
    }

    /// returns true if the button was just pressed
    pub fn mouse_btn_pressed(&self, mouse_btn: keyboard::MouseButton) -> bool {
        match mouse_btn {
            keyboard::MouseButton::Left => self.input.mouse.left_click && !self.prev_input.mouse.left_click,
            keyboard::MouseButton::Middle => self.input.mouse.middle_click && !self.prev_input.mouse.middle_click,
            keyboard::MouseButton::Right => self.input.mouse.right_click && !self.prev_input.mouse.right_click,
        }
    }   
}
