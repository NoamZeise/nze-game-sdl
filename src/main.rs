use sdl_helper::{Map, Camera, Colour, Render, DrawingArea, Error, GameObject, input::{keyboard::Key, Controls, controller}};
use geometry::*;

use std::path::Path;

pub fn main() -> Result<(), Error> {
    
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Game Template", //window name
        geometry::Rect::new(0.0, 0.0, 240.0, 160.0),
        geometry::Vec2::new(240.0 * 4.0, 160.0 * 4.0)
    ).unwrap();
    let mut render = Render::new(drawing_area, &context)?;
   
    let mono_font = render.font_manager.load_font(Path::new("textures/fonts/FiraCode-Light.ttf"))?;
    
    let map = Map::new("test-resources/test.tmx", &mut render.texture_manager, &mut render.font_manager)?;

    //checking resource loading/unloading
    let mut is_gaia = true;
    let mut ephemeral_obj =
        GameObject::new_from_tex(render.texture_manager.load(Path::new("textures/gaia.png"))?);

    let mut text = render.font_manager.load_text_obj(&mono_font,
                                                     "The Planet Earth",
                                                     Colour::new(100, 200, 70, 255),
                                                     Vec2::new(0.0, 0.0), 10.0,
                                                     Vec2::new(0.0, 0.0)
    )?;

    loop {
        update(&mut render, &mut cam)?;

        if render.controls.should_close {
            break;
        }
        
        if render.controls.kbm.down(Key::L) || render.controls.controller_pressed(0, controller::Button::A) {
            render.texture_manager.unload_from_gameobject(ephemeral_obj);
            render.font_manager.unload_text_obj(text);
            if is_gaia {
                ephemeral_obj = GameObject::new_from_tex(
                    render.texture_manager.load(Path::new("textures/error.png"))?);
                text = render.font_manager.load_text_obj(&mono_font, "Error Text", Colour::new(200, 100, 70, 255),
                                                         Vec2::new(100.0, 0.0), 10.0, Vec2::new(0.0, 0.0))?;
                text.parallax = Vec2::new(1.0, 1.0);
            } else {
                ephemeral_obj = GameObject::new_from_tex(
                    render.texture_manager.load(Path::new("textures/gaia.png"))?);
                text = render.font_manager.load_text_obj(&mono_font, "The Planet Earth", Colour::new(100, 200, 70, 255),
                                             Vec2::new(0.0, 0.0), 10.0, Vec2::new(0.0, 0.0))?;
            }
            is_gaia = !is_gaia;
        }
        
        render.start_draw();
        
        map.draw(&mut cam);
        cam.draw_disposable_text(&mono_font, format!("Wheel: {}", render.controls.kbm.mouse_wheel()), 40, render.controls.kbm.mouse_pos(), Colour::white(), Vec2::new(1.0, 1.0));
        cam.draw_text(&text);
        cam.draw(&ephemeral_obj);
        
        render.end_draw(&mut cam)?;
    }

    Ok(())
}


fn update(render: &mut Render, cam: &mut Camera) -> Result<(), Error> {
    let prev_frame = render.controls.frame_elapsed;
    let mut pos = cam.get_offset();
    const SPEED : f64 = 500.0;
    if render.controls.kbm.down(Key::Left) {
        pos.x -= SPEED * prev_frame;
    }
    if render.controls.kbm.down(Key::Right) {
        pos.x += SPEED * prev_frame;
    }
    if render.controls.kbm.down(Key::Up) {
        pos.y -= SPEED * prev_frame;
    }
    if render.controls.kbm.down(Key::Down) {
        pos.y += SPEED * prev_frame;
    }

    pos.x += SPEED * render.controls.kbm.mouse_wheel() as f64 * prev_frame;

    let v =  controller_vector(&render.controls) * SPEED * prev_frame;
    pos = pos + v;
    
    cam.set_offset(pos);
    let mut win_size_update = false;
    let mut cs = cam.get_window_size();
    
    if render.controls.kbm.down(Key::Equals) {
        if cs.x < cam.get_view_size().x {
            cs.x *= 2.0;
            cs.y *= 2.0;
        } else {
            cs.x += cam.get_view_size().x/2.0;
            cs.y += cam.get_view_size().y/2.0;
        }
        win_size_update = true;
    }
    if render.controls.kbm.down(Key::Minus) {
        if cs.x <= cam.get_view_size().x {
            cs.x /= 2.0;
            cs.y /= 2.0;
        } else {
            cs.x -= cam.get_view_size().x/2.0;
            cs.y -= cam.get_view_size().y/2.0;
        }
        win_size_update = true;
    }

    if render.controls.kbm.down(Key::D) {
        cs.x += SPEED * prev_frame;
        win_size_update = true;
    }
    if render.controls.kbm.down(Key::A) {
        
        cs.x -= SPEED * prev_frame;
        win_size_update = true;
    }
    if render.controls.kbm.down(Key::W) {
        
        cs.y += SPEED * prev_frame;
        win_size_update = true;
    }
    if render.controls.kbm.down(Key::S) {
        
        cs.y -= SPEED * prev_frame;
        win_size_update = true;
    }
    
    if win_size_update {
        render.set_win_size(cam, cs, false)?;
    }

    if render.controls.kbm.down(Key::Escape) {
        render.controls.should_close = true;
    }

    render.event_loop(cam);
    
    Ok(())
}

fn controller_vector(controls: &Controls) -> Vec2 {
    if controls.controllers.len() == 0 { return Vec2::new(0.0, 0.0); }
    let mut v = controls.controllers[0].left_joy;
    const DEADZONE: f64 = 0.1; 
    v.x = if v.x.abs() > DEADZONE { v.x } else { 0.0 };
    v.y = if v.y.abs() > DEADZONE { v.y } else { 0.0 };
    v 
}
