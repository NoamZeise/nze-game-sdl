use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;

use geometry::Vec2;
use sdl_test::{TextureManager, FontManager, GameObject, map, camera::Camera};
use sdl_test::input::Typing;

use std::time::Instant;
use std::path::Path;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(image::InitFlag::PNG);

    let mut cam = Camera::new(
        geometry::Rect::new(0.0, 0.0, 640.0, 480.0),
        geometry::Vec2::new(500.0, 500.0)
    );
    
    let window = video_subsystem
        .window(
            "SDL2-Rust",
            cam.get_window_size().x as u32,
            cam.get_window_size().y as u32
        )
        .resizable()
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

    let mut map = map::Map::new("test-resources/level0.tmx", &mut texture_manager).unwrap();
    
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    

    let mut event_pump = sdl_context.event_pump()?;
    let mut typing = Typing::new();
    let mut prev_frame : f64 = 0.0;
    'running: loop {
        let start_time = Instant::now();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    println!("event: {:?}", event);
                    //resize window changes camera~!
                }
            }
            typing.handle_event(&event);
        }
        
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.clear();
        
        map.draw(&mut canvas, &texture_manager, &cam)?;
        font_manager.draw(&mut canvas, &mono_font, "SDL2-Game", 50, Vec2::new(50.0, 100.0), Color::RGBA(255, 255, 255, 255))?;
        
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

        println!("canvas width: {}", canvas.window().size().0);
        
        prev_frame = start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
