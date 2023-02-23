//! for keyboard input state. query by using the `kb` member of [crate::input::Controls]

/// An enum of keyboard buttons used to repressent the keys on a keyboard
///
/// See `key!` macro in crate root for shorthand use of the key array
pub type Key = sdl2::keyboard::Scancode;

use sdl2::event::Event;

/// Holds the keyboard state for a frame of event updates
#[derive(Copy, Clone)]
struct KeyboardStateHolder {
    /// an array indexed by [Key] enums as usize
    pub keys : [bool; Key::Num as usize],
    character : Option<char>,
}

impl KeyboardStateHolder {
    fn new() -> Self {
        KeyboardStateHolder {
            keys      : [false; Key::Num as usize],
            character : None
        }
    }

    fn handle_event(&mut self, event: &Event) {
        if event.is_keyboard() {
            self.handle_keyboard(event);
        } else if event.is_text() {
            self.handle_text(event);
        }
    }

    fn get_typed_character(&mut self) -> Option<char> {
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

/// A type held by `Control` that can be queried for the state of the keyboard
pub struct Keyboard {
    input: KeyboardStateHolder,
    prev_input: KeyboardStateHolder,
}

impl Keyboard {
    pub(super) fn new() -> Keyboard {
        Keyboard {
            input: KeyboardStateHolder::new(),
            prev_input: KeyboardStateHolder::new(),
        }
    }
    pub(super) fn update(&mut self) {
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
}

