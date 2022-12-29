//! processes sdl2 events and update structs that can be used to control the game

use sdl2::event::Event;
use sdl2::EventPump;
pub use sdl2::keyboard::Scancode as Key;
use sdl2::mouse::MouseButton;

use std::time::Instant;

/// macros for getting key bools 
///
/// # down
///
/// **true if key is down this frame**
///
/// use down with the control struct held by render
///```
/// key!(render.controls, down[Key::A]) // True if key A is held down
///```
/// down shorthand
///```
///let input = render.controls.input; //store keyboard struct in a var called input
/// key!(input.down[Key::A]) // True if key A is held down
///```
///
/// # pressed
///
/// **true if key is down this frame but was up last frame**
///
/// using pressed with controls stored in render
///```
/// key!(render.controls,pressed[Key:A])
///```
///
#[macro_export]
macro_rules! key {
    ($controls:expr, down[$key:expr]) => {
	$controls.input.keys[$key as usize]
    };
    ($input:ident.down[$key:expr]) => {
        $input.keys[$key as usize]
    };
    ($controls:expr,pressed[$key:expr]) => {
       $controls.input.keys[$key as usize] && !$controls.prev_input.keys[$key as usize]
    };
}

/// Holds info on input state and frame elapsed time
/// held and updated by 'Render' at the start of each frame
#[derive(Copy, Clone)]
pub struct Controls {
    pub input: KBMControls,
    /// previous frame's input
    pub prev_input: KBMControls,
    /// time in seconds between last frame and this one
    pub frame_elapsed: f64,
    /// This value must be used by the user, it is set to true by the window controls,
    /// this can also be used in your game loop to exit some other way, 
    /// but must be checked in the game loop and broken by the user. 
    pub should_close: bool,
    prev_time: Instant,
}

impl Controls {
    pub(crate) fn new() -> Controls {
        Controls {
            input: KBMControls::new(),
            prev_input: KBMControls::new(),
            frame_elapsed: 0.0,
            prev_time: Instant::now(),
            should_close: false,
        }
    }

    pub(crate) fn update(&mut self, event_pump: &mut EventPump) {
        self.prev_input = self.input;
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
        }
        self.frame_elapsed = self.prev_time.elapsed().as_secs_f64();
        self.prev_time = Instant::now();
    }
}

/// Holds mouse input info
#[derive(Copy, Clone)]
pub struct Mouse {
    pub x : i32,
    pub y : i32,
    pub left_click : bool,
    pub right_click : bool,
}

impl Mouse {
    fn new() -> Self {
        Mouse {
            x: 0,
            y: 0,
            left_click : false,
            right_click : false,
        }
    }
}

/// Holds character typed that frame, and the state of some useful buttons for controls, as well as the mouse
#[derive(Copy, Clone)]
pub struct KBMControls {
    /// an array indexed by [Key] enums as usize
    pub keys : [bool; Key::Num as usize],
    pub mouse : Mouse,
    character : Option<char>,
}

impl KBMControls {
    pub(crate) fn new() -> Self {
        KBMControls {
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
            self.handle_mouse(event);
        }
    }

    /// Get the last character typed by the keyboard in text input mode, or 'None' if nothing was typed
    ///
    /// Getting a character causes the current character to be set to None,
    ///
    /// To enable typing: use 'SdlContext.set_text_input' function.
    pub fn get_typed_character(&mut self) -> Option<char> {
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

    fn handle_mouse(&mut self, event : &Event) {
        let mut btn_down = false;
        let btn = match event {
            Event::MouseMotion { x, y, .. } => {
                self.mouse.x = *x;
                self.mouse.y = *y;
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
                MouseButton::Left => self.mouse.left_click = btn_down,
                MouseButton::Right => self.mouse.right_click = btn_down,
                _ => (),
            }
            None => (),
        }
    }
}
