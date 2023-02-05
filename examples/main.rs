use sdl_helper::{map::Map, Camera, Colour, Render, audio::AudioManager, DrawingArea, Error, GameObject};
use sdl_helper::input::{keyboard::Key, controller, Controls};
use sdl_helper::geometry::*;

use std::path::Path;

pub fn main() -> Result<( ), Error> {
    
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Game Template", //window name
        Rect::new(0.0, 0.0, 240.0, 160.0),
        Vec2::new(240.0 * 4.0, 160.0 * 4.0)
    )?;
    let mut render = Render::new(drawing_area, &context)?;
    let mut controls = Controls::new(&context)?;
   
    let mono_font = render.font_manager.load_font(Path::new("resources/textures/fonts/FiraCode-Light.ttf"))?;
    
    let map = Map::new(
        Path::new("resources/map/test.tmx"), &mut render.texture_manager,
        Path::new("resources/textures/fonts"), &mut render.font_manager)?;

    let mut audio = AudioManager::new()?;

    let music = audio.music.load(Path::new("resources/audio/test.wav"))?;
    audio.music.play(music, -1)?;

    let sfx = audio.sfx.load(Path::new("resources/audio/test.mp3"))?;
    audio.sfx.set_volume(sfx, 0.4)?;
    
    //checking resource loading/unloading
    let mut is_gaia = true;
    let mut ephemeral_obj = GameObject::new_from_tex(render.texture_manager.load(Path::new("resources/textures/gaia.png"))?);

    let mut text = render
        .font_manager
        .load_text_obj(
            &mono_font,
            "The Planet Earth",
            Colour::new(100, 200, 70, 255),
            Vec2::new(0.0, 0.0), 10.0,
            Vec2::new(0.0, 0.0)
        )?;

    while !controls.should_close {
        cam_controls(&mut render, &mut controls, &mut cam)?;
        controls.update(&cam);
        
        // load/unload resources
        if controls.kbm.press(Key::L) || controls.controller.press(0, controller::Button::A) {
            render.texture_manager.unload_from_gameobject(ephemeral_obj);
            render.font_manager.unload_text_obj(text);
            if is_gaia {
                ephemeral_obj = GameObject::new_from_tex(
                    render.texture_manager.load(Path::new("resources/textures/error.png"))?);
                text = render.font_manager.load_text_obj(&mono_font, "Error Text", Colour::new(200, 100, 70, 255),
                                                         Vec2::new(100.0, 0.0), 10.0, Vec2::new(0.0, 0.0))?;
                text.parallax = Vec2::new(1.0, 1.0);
            } else {
                ephemeral_obj = GameObject::new_from_tex(
                    render.texture_manager.load(Path::new("resources/textures/gaia.png"))?);
                text = render.font_manager.load_text_obj(&mono_font, "The Planet Earth", Colour::new(100, 200, 70, 255),
                                             Vec2::new(0.0, 0.0), 10.0, Vec2::new(0.0, 0.0))?;
            }
            is_gaia = !is_gaia;
        }

        if controls.kbm.press(Key::P) {
            audio.sfx.play(sfx)?;
        }
        
        if controls.controller.press(0, controller::Button::DPadUp) {
            controls.controller.rumble(0, 10000, 20000, 1000);
        }
        
        render.start_draw();
        
        map.draw(&mut cam);
        cam.draw_disposable_text(&mono_font, format!("Wheel: {}", controls.kbm.mouse_wheel()), 40, controls.kbm.mouse_pos(), Colour::white(), Vec2::new(1.0, 1.0));
        cam.draw_text(&text);
        cam.draw(&ephemeral_obj);
        
        render.end_draw(&mut cam)?;
    }

    Ok(())
}


fn cam_controls(render: &mut Render, controls: &mut Controls, cam: &mut Camera) -> Result<(), Error> {
    let prev_frame = controls.frame_elapsed;
    let mut pos = cam.get_offset();
    const SPEED : f64 = 500.0;
    if controls.kbm.down(Key::Left) {
        pos.x -= SPEED * prev_frame;
    }
    if controls.kbm.down(Key::Right) {
        pos.x += SPEED * prev_frame;
    }
    if controls.kbm.down(Key::Up) {
        pos.y -= SPEED * prev_frame;
    }
    if controls.kbm.down(Key::Down) {
        pos.y += SPEED * prev_frame;
    }

    pos.x += SPEED * controls.kbm.mouse_wheel() as f64 * prev_frame;

    let v =  controls.controller.joy(0, controller::Side::Left) * SPEED * prev_frame;
    pos = pos + v;
    
    cam.set_offset(pos);
    let mut win_size_update = false;
    let mut cs = cam.get_window_size();
    
    if controls.kbm.down(Key::Equals) {
        if cs.x < cam.get_view_size().x {
            cs.x *= 2.0;
            cs.y *= 2.0;
        } else {
            cs.x += cam.get_view_size().x/2.0;
            cs.y += cam.get_view_size().y/2.0;
        }
        win_size_update = true;
    }
    if controls.kbm.down(Key::Minus) {
        if cs.x <= cam.get_view_size().x {
            cs.x /= 2.0;
            cs.y /= 2.0;
        } else {
            cs.x -= cam.get_view_size().x/2.0;
            cs.y -= cam.get_view_size().y/2.0;
        }
        win_size_update = true;
    }

    if controls.kbm.down(Key::D) {
        cs.x += SPEED * prev_frame;
        win_size_update = true;
    }
    if controls.kbm.down(Key::A) {
        
        cs.x -= SPEED * prev_frame;
        win_size_update = true;
    }
    if controls.kbm.down(Key::W) {
        
        cs.y += SPEED * prev_frame;
        win_size_update = true;
    }
    if controls.kbm.down(Key::S) {
        
        cs.y -= SPEED * prev_frame;
        win_size_update = true;
    }
    
    if win_size_update {
        render.set_win_size(cam, cs, false)?;
    }

    if controls.kbm.down(Key::Escape) {
        controls.should_close = true;
    }
    
    Ok(())
}
