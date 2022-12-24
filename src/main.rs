use sdl_helper::{Map, Camera, input::Keyboard, Colour, Render, DrawingArea};
use geometry::*;

use std::time::Instant;
use std::path::Path;

pub fn main() -> Result<(), String> {
    let mut cam = Camera::new(
        geometry::Rect::new(0.0, 0.0, 240.0, 160.0),
        geometry::Vec2::new(240.0, 160.0)
    );

    let (drawing_area, context) = DrawingArea::new(&cam)?;

    let mut render = Render::new(drawing_area, &context)?;
   
    let mono_font = render.font_manager.load_font(Path::new("textures/FiraCode-Light.ttf"))?;

    let map = Map::new("test-resources/test.tmx", &mut render.texture_manager)?;

   
    
    let mut input = Keyboard::new();
    let mut prev_frame : f64 = 0.0;
    loop {
        let start_time = Instant::now();

        render.event_loop(&mut input);
        change_win_controls(&input, &mut render, &mut cam)?;
        
        render.start_draw();
        
        map.draw(&mut cam);

        cam.draw_text(&mono_font, "Hello SDL!".to_string(), 40, Vec2::new(10.0, 40.0), Colour::white(), Vec2::new(1.0, 1.0));
        
        render.end_draw(&mut cam)?;

        if input.esc {
            break;
        }
        
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
        
        prev_frame = start_time.elapsed().as_secs_f64();

        //println!("prev frame: {} fps", 1.0/prev_frame);
    }

    Ok(())
}


fn change_win_controls(input: &Keyboard, render: &mut Render, cam: &mut Camera) -> Result<(), String> {
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
    Ok(())
}
