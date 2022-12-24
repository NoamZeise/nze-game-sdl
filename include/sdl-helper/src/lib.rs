//! A library to abstract away the details of the sdl2 library for creating games easier
use sdl2::pixels::Color;
use sdl2::{Sdl, VideoSubsystem, EventPump, image};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

pub mod input;
mod map;
mod camera;
mod texture_manager;
mod font_manager;
pub mod resource;
mod types;
mod rect_conversion;

pub use texture_manager::TextureManager;
pub use camera::Camera;
pub use font_manager::{FontManager, TextDraw};
pub use types::{Colour, GameObject};
pub use map::Map;

use geometry::*;


/// This holds ownership of many sdl types that are required for being able to use it,
/// but the context will not be changed after creation.
/// It is created by [DrawingArea]
pub struct ContextSdl {
    sdl_context : Sdl,
    _video_subsystem: VideoSubsystem,
    _image_context: image::Sdl2ImageContext,
    ttf_context: sdl2::ttf::Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
}

impl ContextSdl {
    fn new(cam: &Camera) -> Result<(Canvas<Window>, ContextSdl), String> {
        let sdl_context = sdl2::init()?;
        let _video_subsystem = sdl_context.video()?;
        let _image_context = image::init(image::InitFlag::PNG)?;
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
        let window = _video_subsystem
        .window(
            "SDL2-Rust",
            cam.get_window_size().x as u32, cam.get_window_size().y as u32)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
        let texture_creator = canvas.texture_creator();

        Ok((canvas, ContextSdl { sdl_context, _video_subsystem, _image_context, ttf_context, texture_creator}))
    }

}

/// Holds ownership of an sdl Canvas, this should be passed to [Render]
pub struct DrawingArea {
    canvas: Canvas<Window>
}

impl DrawingArea {
    ///returns the [ContextSdl] of this instance of sdl2, as well as a [DrawingArea]
    pub fn new(cam: &Camera) -> Result<(DrawingArea,ContextSdl), String> {
        let (mut canvas, holder) = ContextSdl::new(&cam)?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        Ok((DrawingArea { canvas }, holder))
    }
}

/// Holds ownership of a [DrawingArea] and the resource managers.
/// Also contains functions for doing the main update,draw loop
pub struct Render<'sdl> {
    pub texture_manager: TextureManager<'sdl, WindowContext>,
    pub font_manager: FontManager<'sdl, WindowContext>,
    event_pump: EventPump,
    drawing_area: DrawingArea,
}


impl<'sdl> Render<'sdl> {
    pub fn new(drawing_area: DrawingArea, context: &ContextSdl) -> Result<Render, String> {
        Ok(Render {
            texture_manager: TextureManager::new(&context.texture_creator),
            font_manager: FontManager::new(&context.ttf_context, &context.texture_creator)?,
            event_pump: context.sdl_context.event_pump()?,
            drawing_area,
        })
    }
    /// Clears the [DrawingArea] for it to be filled with this frame's drawing instructions
    pub fn start_draw(&mut self) {
        self.drawing_area.canvas.set_draw_color(Color::BLACK);
        self.drawing_area.canvas.clear();
    }

    ///drain the draws from [Camera] and draw to [DrawingArea]
    pub fn end_draw(&mut self, cam: &mut Camera) -> Result<(), String>{
        
        for d in cam.drain_draws() {
            self.texture_manager.draw(&mut self.drawing_area.canvas, d)?;
        }
        for d in cam.drain_tex_draws() {
            self.font_manager.draw_disposable(&mut self.drawing_area.canvas, d)?;
        }
        for d in cam.drain_rect_draws() {
            self.texture_manager.draw_rect(&mut self.drawing_area.canvas, &d.0, d.1)?;
        }
      
        self.drawing_area.canvas.present();
        Ok(())
    }

    /// Update the input struct using the sdl events that occured between the previous call
    pub fn event_loop(&mut self, input: &mut input::Keyboard) {
        for event in self.event_pump.poll_iter() {
            input.handle_event(&event);
        }
    }

    /// Update the game window to the new size, and change the [Camera] to the new resolution
    pub fn set_win_size(&mut self, cam: &mut Camera, cs: Vec2) -> Result<(), String> {
        match self.drawing_area.canvas.window_mut().set_size(cs.x as u32, cs.y as u32) {
            Err(_) => { return Err(String::from("failed to resize window"));},
            _ => ()
        }
        cam.set_window_size(cs);
        self.drawing_area.canvas.window_mut().set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered
        );
        Ok(())
    }
}