use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::image;

use geometry::Vec2;
use sdl_test::{TextureManager, FontManager, GameObject, map};
use sdl_test::input::Typing;

use std::time::Instant;
use std::path::Path;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(image::InitFlag::PNG);
    let window = video_subsystem
        .window("SDL2-Rust", 640, 480)
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
    let mut test_text = GameObject::new_from_tex(texture_manager.load(Path::new("textures/gaia.png"))?);
    test_text.draw_rect.x = 350.0;
    test_text.draw_rect.y = 170.0;
    //test_text.tex_rect.x += 50.0;
    //test_text.tex_rect.w *= 0.5;
 //   test_text.tex_recttest_text.tex.width / 2;
    let mut map = map::Map::new("test-resources/test.tmx", &mut texture_manager).unwrap();
    
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
                _ => {}
            }
            typing.handle_event(&event);
        }
        canvas.set_draw_color(Color::RGB(10, 10, 10));
        canvas.clear();
        
        font_manager.draw(&mut canvas, &mono_font, "SDL2-Game", 50, Vec2::new(50.0, 100.0), Color::RGBA(255, 255, 255, 255))?;
        texture_manager.draw(&mut canvas, &test_text)?;
        map.draw(&mut canvas, &texture_manager)?;
        
        canvas.present();

        prev_frame = start_time.elapsed().as_secs_f64();
    }

    Ok(())
}
