use sdl2::event::Event;
use sdl2::mouse::MouseButton;

use geometry::Vec2;

/// Holds mouse input info
#[derive(Copy, Clone)]
pub(crate) struct Mouse {
    pub x : i32,
    pub y : i32,
    pub pos: Vec2,
    pub wheel: i32,
    pub left_click : bool,
    pub middle_click: bool,
    pub right_click : bool,
}


impl Mouse {
    pub(super) fn new() -> Self {
        Mouse {
            x: 0,
            y: 0,
            pos: Vec2::new(0.0, 0.0),
            wheel: 0,
            left_click : false,
            middle_click : false,
            right_click : false,
        }
    }

    pub(super) fn handle_mouse(&mut self, event: &Event) {
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
                MouseButton::Left => self.left_click = btn_down,
                MouseButton::Middle => self.middle_click = btn_down,
                MouseButton::Right => self.right_click = btn_down,
                _ => (),
            }
            None => (),
        }
    }
}
