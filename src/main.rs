use sdl_helper::{Map, Camera, Colour, Render, DrawingArea, TextObject, Error, GameObject, input::Key, key};
use geometry::*;

use std::path::Path;

pub fn main() -> Result<(), Error> {
    
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Game Template", //window name
        geometry::Rect::new(0.0, 0.0, 240.0, 160.0),
        geometry::Vec2::new(240.0 * 4.0, 160.0 * 4.0)
    ).unwrap();
    let mut render = Render::new(drawing_area, &context)?;
   
    let mono_font = render.font_manager.load(Path::new("textures/fonts/FiraCode-Light.ttf"))?;

    let text = render.font_manager.load_text(&mono_font, "Prerendered Text", Colour::white())?;
    let text = TextObject::new(text,
                               sdl_helper::get_text_rect_from_height(
                                   Vec2::new(text.width as f64, text.height as f64),
                                   Vec2::new(0.0, 0.0), 40.0
                               ),
                               Vec2::new(1.0, 1.0), Colour::white()
    );

    let map = Map::new("test-resources/test.tmx", &mut render.texture_manager, &mut render.font_manager)?;

    //checking resource loading/unloading
    let mut is_gaia = true;
    let mut ephemeral_obj =
        GameObject::new_from_tex(render.texture_manager.load(Path::new("textures/gaia.png"))?);

    loop {
        update(&mut render, &mut cam)?;

        if render.controls.should_close {
            break;
        }
        
        if key!(render.controls,pressed[Key::L]) {
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
        cam.draw_disposable_text(&mono_font, format!("Wheel: {}", render.controls.input.mouse.wheel), 40, render.controls.input.mouse.pos, Colour::white(), Vec2::new(1.0, 1.0));
        cam.draw_text(&text);
        cam.draw(&ephemeral_obj);
        
        render.end_draw(&mut cam)?;
    }

    Ok(())
}


fn update(render: &mut Render, cam: &mut Camera) -> Result<(), Error> {
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

    pos.x += SPEED * input.mouse.wheel as f64 * prev_frame;
    
    cam.set_offset(pos);
    let mut win_size_update = false;
    let mut cs = cam.get_window_size();
    
    if key!(input.down[Key::Equals]) {
        if cs.x < cam.get_view_size().x {
            cs.x *= 2.0;
            cs.y *= 2.0;
        } else {
            cs.x += cam.get_view_size().x/2.0;
            cs.y += cam.get_view_size().y/2.0;
        }
        win_size_update = true;
    }
    if key!(input.down[Key::Minus]) {
        if cs.x <= cam.get_view_size().x {
            cs.x /= 2.0;
            cs.y /= 2.0;
        } else {
            cs.x -= cam.get_view_size().x/2.0;
            cs.y -= cam.get_view_size().y/2.0;
        }
        win_size_update = true;
    }

    if key!(input.down[Key::D]) {
        cs.x += SPEED * prev_frame;
        win_size_update = true;
    }
    if key!(input.down[Key::A]) {
        
        cs.x -= SPEED * prev_frame;
        win_size_update = true;
    }
    if key!(input.down[Key::W]) {
        
        cs.y += SPEED * prev_frame;
        win_size_update = true;
    }
    if key!(input.down[Key::S]) {
        
        cs.y -= SPEED * prev_frame;
        win_size_update = true;
    }
    
    if win_size_update {
        render.set_win_size(cam, cs, true)?;
    }

    if key!(input.down[Key::Escape]) {
        render.controls.should_close = true;
    }

    render.event_loop(cam);
    
    Ok(())
}
