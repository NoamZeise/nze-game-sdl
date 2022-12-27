use sdl_helper::{Map, Camera, Colour, Render, DrawingArea, TextObject};
use geometry::*;

use std::path::Path;

pub fn main() -> Result<(), String> {
    
    let (mut cam, drawing_area, context) = DrawingArea::new(
        "Game Template",
        geometry::Rect::new(0.0, 0.0, 240.0, 160.0),
        geometry::Vec2::new(240.0, 160.0)
    )?;
    let mut render = Render::new(drawing_area, &context)?;
   
    let mono_font = render.font_manager.load_font(Path::new("textures/fonts/FiraCode-Light.ttf"))?;

    let text = render.font_manager.get_text(&mono_font, "Hello Sdl2", Colour::white())?;
    let text = TextObject::new(text, sdl_helper::get_text_rect_from_height(Vec2::new(text.width as f64, text.height as f64), Vec2::new(0.0, 0.0), 10.0), Vec2::new(1.0, 1.0), Colour::white());

    let map = Map::new("test-resources/test.tmx", &mut render.texture_manager, &mut render.font_manager)?;

    loop {
        update(&mut render, &mut cam)?;
        
        render.start_draw();
        
        map.draw(&mut cam);
        //cam.draw_disposable_text(&mono_font, "Hello SDL!".to_string(), 40, Vec2::new(10.0, 40.0), Colour::white(), Vec2::new(1.0, 1.0));

        cam.draw_text(&text);
        
        render.end_draw(&mut cam)?;

        if render.controls.should_close {
            break;
        }
    }

    Ok(())
}


fn update(render: &mut Render, cam: &mut Camera) -> Result<(), String> {
    render.event_loop();
    let input = render.controls.input;
    let prev_frame = render.controls.frame_elapsed;
    let mut pos = cam.get_offset();
    const SPEED : f64 = 500.0;
    if input.left {
        pos.x -= SPEED * prev_frame;
    }
    if input.right {
        pos.x += SPEED * prev_frame;
    }
    if input.up {
        pos.y -= SPEED * prev_frame;
    }
    if input.down {
        pos.y += SPEED * prev_frame;
    }
    cam.set_offset(pos);
    
    if input.plus {
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
    if input.minus {
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
    if input.esc {
        render.controls.should_close = true;
    }
    Ok(())
}
