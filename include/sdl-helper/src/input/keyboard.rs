/// An enum of keyboard buttons used to index the array help by `KBMcontrols`
///
/// See `key!` macro in crate root for shorthand use of the key array
pub use sdl2::keyboard::Scancode as Key;

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
pub(crate) struct KeyboardAndMouse {
    /// an array indexed by [Key] enums as usize
    pub keys : [bool; Key::Num as usize],
    /// represents the mouse input state for this frame
    pub mouse : Mouse,
    character : Option<char>,
}

impl KeyboardAndMouse {
    pub(crate) fn new() -> Self {
        KeyboardAndMouse {
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
}
