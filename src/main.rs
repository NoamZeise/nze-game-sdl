use sdl_helper::{Map, Camera, Colour, Render, DrawingArea, TextObject, error::Error, GameObject, input::Key, key};
use geometry::*;

use std::path::Path;

pub fn main() -> Result<(), Error> {
    
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Game Template",
        geometry::Rect::new(0.0, 0.0, 240.0, 160.0),
        geometry::Vec2::new(240.0 * 4.0, 160.0 * 4.0)
    ).unwrap();
    let mut render = Render::new(drawing_area, &context)?;
   
    let mono_font = render.font_manager.load(Path::new("textures/fonts/FiraCode-Light.ttf"))?;

    let text = render.font_manager.load_text(&mono_font, "Hello Sdl2", Colour::white())?;
    let text = TextObject::new(text,
                               sdl_helper::get_text_rect_from_height(
                                   Vec2::new(text.width as f64, text.height as f64),
                                   Vec2::new(0.0, 0.0), 10.0
                               ),
                               Vec2::new(1.0, 1.0), Colour::white()
    );

    let map = Map::new("test-resources/test.tmx", &mut render.texture_manager, &mut render.font_manager)?;

    let mut is_gaia = true;
    let mut ephemeral_obj =
        GameObject::new_from_tex(render.texture_manager.load(Path::new("textures/gaia.png"))?);

    loop {
        update(&mut render, &mut cam)?;

        if render.controls.should_close {
            break;
        }
        
        if key!(render.controls,pressed[Key::A]) {
            render.texture_manager.unload_from_gameobject(ephemeral_obj);
            if is_gaia {
                ephemeral_obj = GameObject::new_from_tex(
                    render.texture_manager.load(Path::new("textures/error.png"))?);
            } else {
                ephemeral_obj = GameObject::new_from_tex(
                    render.texture_manager.load(Path::new("textures/gaia.png"))?);
            }
            is_gaia = !is_gaia;
        }
        
        render.start_draw();
        
        map.draw(&mut cam);
        //cam.draw_disposable_text(&mono_font, "Hello SDL!".to_string(), 40, Vec2::new(10.0, 40.0), Colour::white(), Vec2::new(1.0, 1.0));
        cam.draw_text(&text);
        cam.draw(&ephemeral_obj);
        
        render.end_draw(&mut cam)?;
    }

    Ok(())
}


fn update(render: &mut Render, cam: &mut Camera) -> Result<(), Error> {
    render.event_loop();
    let input = render.controls.input;
    let prev_frame = render.controls.frame_elapsed;
    let mut pos = cam.get_offset();
    const SPEED : f64 = 500.0;
    if key!(input.down[Key::Left]) {
        pos.x -= SPEED * prev_frame;
    }
    if key!(input.down[Key::Right]) {
        pos.x += SPEED * prev_frame;
    }
    if key!(input.down[Key::Up]) {
        pos.y -= SPEED * prev_frame;
    }
    if key!(input.down[Key::Down]) {
        pos.y += SPEED * prev_frame;
    }
    cam.set_offset(pos);
    
    if key!(input.down[Key::Equals]) {
        let mut cs = cam.get_window_size();
        if cs.x < cam.get_view_size().x {
            cs.x *= 2.0;
            cs.y *= 2.0;
        } else {
            cs.x += cam.get_view_size().x/2.0;
            cs.y += cam.get_view_size().y/2.0;
        }
        render.set_win_size(cam, cs)?;
    }
    if key!(input.down[Key::Minus]) {
        let mut cs = cam.get_window_size();
        if cs.x <= cam.get_view_size().x {
            cs.x /= 2.0;
            cs.y /= 2.0;
        } else {
            cs.x -= cam.get_view_size().x/2.0;
            cs.y -= cam.get_view_size().y/2.0;
        }
        render.set_win_size(cam, cs)?;
    }

    if key!(input.down[Key::D]) {
        let mut cs = cam.get_window_size();
        cs.x += SPEED * prev_frame;
        render.set_win_size(cam, cs)?;
    }
    if key!(input.down[Key::A]) {
        let mut cs = cam.get_window_size();
        cs.x -= SPEED * prev_frame;
        render.set_win_size(cam, cs)?;
    }
    if key!(input.down[Key::W]) {
        let mut cs = cam.get_window_size();
        cs.y += SPEED * prev_frame;
        render.set_win_size(cam, cs)?;
    }
    if key!(input.down[Key::S]) {
        let mut cs = cam.get_window_size();
        cs.y -= SPEED * prev_frame;
        render.set_win_size(cam, cs)?;
    }
    if key!(input.down[Key::Escape]) {
        render.controls.should_close = true;
    }
    Ok(())
}
