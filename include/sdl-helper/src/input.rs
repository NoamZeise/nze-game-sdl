//! take sdl2 events and update a struct of bools for required controls

use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

use std::time::Instant;


/// Holds info on input state and frame elapsed time
/// held and updated by 'Render' at the start of each frame
#[derive(Copy, Clone)]
pub struct Controls {
    pub input: KBMControls,
    pub prev_input: KBMControls,
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
    pub up        : bool,
    pub down      : bool,
    pub left      : bool,
    pub right     : bool,
    pub a         : bool,
    pub b         : bool,
    pub esc       : bool,
    pub plus      : bool,
    pub minus     : bool,
    pub mouse     : Mouse,
    character     : Option<char>,
}

impl KBMControls {

    pub(crate) fn new() -> Self {
        KBMControls {
            up        : false,
            down      : false,
            left      : false,
            right     : false,
            a         : false,
            b         : false,
            esc       : false,
            plus      : false,
            minus     : false,
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
            Some(k) => {
                match k {
                    Scancode::Up => self.up    = key_down,
                    Scancode::Left => self.left  = key_down,
                    Scancode::Down => self.down  = key_down,
                    Scancode::Right => self.right = key_down,
                    Scancode::Z => self.a = key_down,
                    Scancode::X => self.b = key_down,
                    Scancode::Escape => self.esc = key_down,
                    Scancode::Equals => self.plus = key_down,
                    Scancode::Minus  => self.minus = key_down,
                    _ => {}
                }
            }
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
