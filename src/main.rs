
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;
use sdl2::video::Window;
use sdl2::render::Canvas;

use geometry::Vec2;
use sdl_test::{TextureManager, FontManager, map, camera::Camera};
use sdl_test::input::Typing;

use std::time::Instant;
use std::path::Path;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(image::InitFlag::PNG);

    let mut cam = Camera::new(
        geometry::Rect::new(0.0, 0.0, 240.0, 160.0),
        geometry::Vec2::new(240.0, 160.0)
    );
    
    let window = video_subsystem
        .window(
            "SDL2-Rust",
            cam.get_window_size().x as u32,
            cam.get_window_size().y as u32
        )
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture_manager = TextureManager::new(&texture_creator);
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font_manager = FontManager::new(&ttf_context, &texture_creator)?;

    let mono_font = font_manager.load_font(Path::new("textures/FiraCode-Light.ttf"))?;

    let mut map = map::Map::new("test-resources/test.tmx", &mut texture_manager).unwrap();
    
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    

    let mut event_pump = sdl_context.event_pump()?;
    let mut typing = Typing::new();
    let mut prev_frame : f64 = 0.0;
    'running: loop {
        let start_time = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {  keycode: Some(Keycode::Escape), ..} => break 'running,
                _ => { }
            }
            typing.handle_event(&event);
            handle_event(&event, &mut canvas, &mut cam)?;
        }
        
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        map.draw(&mut cam);
        
        for d in cam.drain_draws() {
            texture_manager.draw(&mut canvas, d)?;
        }
      
        canvas.present();
        
        let mut pos = cam.get_offset();
        const SPEED : f64 = 500.0;
        if typing.left {
            pos.x -= SPEED * prev_frame;
        }
        if typing.right {
            pos.x += SPEED * prev_frame;
        }
        if typing.up {
            pos.y -= SPEED * prev_frame;
        }
        if typing.down {
            pos.y += SPEED * prev_frame;
        }
        cam.set_offset(pos);
        
        prev_frame = start_time.elapsed().as_secs_f64();

        //println!("prev frame: {} fps", 1.0/prev_frame);
    }

    Ok(())
}


fn handle_event(event: &Event, canvas: &mut Canvas<Window>, cam: &mut Camera) -> Result<(), String> {
    match event {
        Event::KeyDown {
            keycode: Some(Keycode::Equals),
            ..
        } => {
            let mut cs = cam.get_window_size();
            if cs.x < cam.get_view_size().x {
                cs.x *= 2.0;
                cs.y *= 2.0;
            } else {
                cs.x += cam.get_view_size().x/2.0;
                cs.y += cam.get_view_size().y/2.0;
            }
            set_win_size(canvas, cam, cs)?;
        },
        Event::KeyDown {
            keycode: Some(Keycode::Minus),
            ..
        } => {
            let mut cs = cam.get_window_size();
            if cs.x <= cam.get_view_size().x {
                cs.x /= 2.0;
                cs.y /= 2.0;
            } else {
                cs.x -= cam.get_view_size().x/2.0;
                cs.y -= cam.get_view_size().y/2.0;
            }
            set_win_size(canvas, cam, cs)?;
        },
        _ => {}
    }
    Ok(())
}

fn set_win_size(canvas: &mut Canvas<Window>, cam: &mut Camera, cs: Vec2) -> Result<(), String> {
    match canvas.window_mut().set_size(cs.x as u32, cs.y as u32) {
        Err(_) => { return Err(String::from("failed to resize window"));},
        _ => ()
    }
    cam.set_window_size(cs);
    canvas.window_mut().set_position(
        sdl2::video::WindowPos::Centered,
        sdl2::video::WindowPos::Centered
    );
    Ok(())
}
