use std::path::Path;
use nze_game_sdl::{
    GameObject,
    Render, DrawingArea,
    Error,
    input::Controls,
    input::keyboard::Key,
    geometry::{Rect, Vec2},
};

pub fn main() -> Result<(), Error> {
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Name of Game",                              // window name
        Rect::new(0.0, 0.0, 400.0, 400.0), // window camera
        Vec2::new(400.0, 400.0)            // window size
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;
    
    let mut obj = GameObject::new_from_tex(
        render.texture_manager.load(Path::new("resources/textures/gaia.png"))?
    );
    
    obj.rect.x = cam.get_view_size().x / 2.0 - obj.rect.w / 2.0;
    obj.rect.y = cam.get_view_size().y / 2.0 - obj.rect.h / 2.0;

    const SPEED: f64 = 125.0;
    
    while !controls.should_close {
        controls.update(&cam);
        
        if controls.kb.hold(Key::D) {
            obj.rect.x += SPEED * controls.frame_elapsed;
        }

        if controls.kb.hold(Key::A) {
            obj.rect.x -= SPEED * controls.frame_elapsed;
        }

        if controls.kb.hold(Key::W) {
            obj.rect.y -= SPEED * controls.frame_elapsed;
        }
        
        if controls.kb.hold(Key::S) {
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
