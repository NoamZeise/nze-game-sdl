/// This example shows how to use the multi button
/// input system. This lets you register multiple
/// input methods to a single virtual button.

use std::path::Path;
use nze_game_sdl::{
    GameObject,
    Render, DrawingArea,
    Error,
    input::Controls,
    input::keyboard::Key,
    input::controller,
    input::multi::{MultiInput, MuliBtnRef, MultiButton, Joy},
    geometry::{Rect, Vec2},
};

/// Hold the inputs that our game accepts
struct ButtonMap {
    pub up: MuliBtnRef,
    pub down: MuliBtnRef,
    pub left: MuliBtnRef,
    pub right: MuliBtnRef,
}

impl ButtonMap {
    /// register the buttons with multiinput
    /// then register the possible ways of activating
    /// the buttons
    pub fn new(mi: &mut MultiInput) -> ButtonMap {
        let mut up = MultiButton::new(0.0);
        let mut down = MultiButton::new(0.0);
        let mut left = MultiButton::new(0.0);
        let mut right = MultiButton::new(0.0);
        up.register_key(Key::W);
        up.register_controller_btn(0, controller::Button::DPadUp);
        up.register_joy(0, Joy::new(Vec2::new(0.0, -1.0), controller::Side::Left));
        down.register_key(Key::S);
        down.register_controller_btn(0, controller::Button::DPadDown);
        down.register_joy(0, Joy::new(Vec2::new(0.0, 1.0), controller::Side::Left));
        left.register_key(Key::A);
        left.register_controller_btn(0, controller::Button::DPadLeft);
        left.register_joy(0, Joy::new(Vec2::new(-1.0, 0.0), controller::Side::Left));
        right.register_key(Key::D);
        right.register_controller_btn(0, controller::Button::DPadRight);
        right.register_joy(0, Joy::new(Vec2::new(1.0, 0.0), controller::Side::Left));
        ButtonMap {
            up: mi.add_btn(up),
            down: mi.add_btn(down),
            left: mi.add_btn(left),
            right: mi.add_btn(right),
        }
    }
}

pub fn main() -> Result<(), Error> {
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Multibutton Example",             // window name
        Rect::new(0.0, 0.0, 400.0, 400.0), // window camera
        Vec2::new(400.0, 400.0)            // window size
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;

    let mut input = MultiInput::new();
    let btns = ButtonMap::new(&mut input);
    
    let mut obj = GameObject::new_from_tex(
        render.texture_manager.load(Path::new("resources/textures/gaia.png"))?
    );
    
    obj.rect.x = cam.get_view_size().x / 2.0 - obj.rect.w / 2.0;
    obj.rect.y = cam.get_view_size().y / 2.0 - obj.rect.h / 2.0;

    const SPEED: f64 = 125.0;
    
    while !controls.should_close {
        controls.update(&cam);
        input.update(&controls);
        
        if input.hold(&btns.right) {
            obj.rect.x += SPEED * controls.frame_elapsed;
        }

        if input.hold(&btns.left) {
            obj.rect.x -= SPEED * controls.frame_elapsed;
        }

        if input.hold(&btns.up) {
            obj.rect.y -= SPEED * controls.frame_elapsed;
        }
        
        if input.hold(&btns.down) {
            obj.rect.y += SPEED * controls.frame_elapsed;
        }

        if controls.kb.hold(Key::Escape) {
            controls.should_close = true;
        }
        
        render.start_draw();
        cam.draw(&obj);
        render.end_draw(&mut cam)?;
    }
    Ok(())
}

