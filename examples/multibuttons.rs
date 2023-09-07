/// This example shows how to use the multi button
/// input system. This lets you register multiple
/// input methods to a single virtual button.
///
/// The example shows a 5 button menu at the top, where buttons
/// can be selected by using the left/right keys, or the right
/// joystick of a controller going left/right.
/// A planet can be moved around using WASD, D-pad or the right joy stick
/// of a controller.

use std::path::Path;
use nze_game_sdl::{
    GameObject,
    Render, DrawingArea,
    Error,
    input::Controls,
    input::keyboard::Key,
    input::controller,
    input::multi::{MultiInput, MuliBtnRef, MultiButton, Joy},
    geometry::{Rect, Vec2}, Colour,
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

    let mut planet_input = MultiInput::new();
    let btns = ButtonMap::new(&mut planet_input);
    
    let mut planet = GameObject::new_from_tex(
        render.texture_manager.load(Path::new("resources/textures/gaia.png"))?
    );
    planet.rect.x = cam.get_view_size().x / 2.0 - planet.rect.w / 2.0;
    planet.rect.y = cam.get_view_size().y / 2.0 - planet.rect.h / 2.0;


    let mut menu_input = MultiInput::new();
    const BTN_DELAY: f64 = 0.4;
    let mut btn_left = MultiButton::new(BTN_DELAY);
    btn_left.register_key(Key::Left);
    btn_left.register_joy(0, Joy::new(Vec2::new(-1.0, 0.0), controller::Side::Right));
    let btn_left = menu_input.add_btn(btn_left);

    let mut btn_right = MultiButton::new(BTN_DELAY);
    btn_right.register_key(Key::Right);
    btn_right.register_joy(0, Joy::new(Vec2::new(1.0, 0.0), controller::Side::Right));
    let btn_right = menu_input.add_btn(btn_right);
    
    let mut btn_obj = GameObject::new_from_tex(
        render.texture_manager.load(Path::new("resources/textures/button.png"))?);
    const BUTTON_AMOUNT: usize = 5;
    let mut buttons : Vec<GameObject> = (0..BUTTON_AMOUNT).map(|x| {
        btn_obj.rect = Rect::new( 35.0 + x as f64 * 70.0,
                                  20.0, 50.0, 25.0);
        btn_obj.clone()
    }).collect();

    let mut selected_button = 0;

    const SPEED: f64 = 125.0;
    
    while !controls.should_close {
        controls.update(&cam);
        planet_input.update(&controls);
        menu_input.update(&controls);
        
        if planet_input.hold(&btns.right) {
            planet.rect.x += SPEED * controls.frame_elapsed;
        }

        if planet_input.hold(&btns.left) {
            planet.rect.x -= SPEED * controls.frame_elapsed;
        }

        if planet_input.hold(&btns.up) {
            planet.rect.y -= SPEED * controls.frame_elapsed;
        }
        
        if planet_input.hold(&btns.down) {
            planet.rect.y += SPEED * controls.frame_elapsed;
        }

        if controls.kb.hold(Key::Escape) {
            controls.should_close = true;
        }

        if menu_input.press(&btn_left) {
            if selected_button == 0 && BUTTON_AMOUNT > 0 {
                selected_button = BUTTON_AMOUNT - 1;
            } else {
                selected_button -= 1;
            }
        }
        
        if menu_input.press(&btn_right) {
            selected_button = (selected_button + 1) % BUTTON_AMOUNT;
        }
        
        render.start_draw();
        cam.draw(&planet);
        for (i, b) in buttons.iter_mut().enumerate() {
            b.colour = if i == selected_button {
                Colour::new(150, 150, 150, 255)
            } else {
                Colour::white()
            };
            cam.draw(b);
        }
        render.end_draw(&mut cam)?;
    }
    Ok(())
}

