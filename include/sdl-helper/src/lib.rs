//! A library to abstract away the details of the sdl2 library for creating games easier
use sdl2::pixels::Color;
use sdl2::{Sdl, VideoSubsystem, EventPump, image};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

pub mod input;
pub mod resource;
pub mod error;
mod map;
mod camera;
mod texture_manager;
mod font_manager;
mod types;
mod rect_conversion;
mod error_macros;

pub use texture_manager::TextureManager;
pub use camera::Camera;
pub use font_manager::{FontManager, get_text_rect_from_height};
pub use types::{Colour, GameObject, TextObject};
pub use map::Map;
use crate::input::Controls;

use geometry::*;
use error::Error;


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
    fn new(cam: &Camera, window_name: &str) -> Result<(Canvas<Window>, ContextSdl), Error> {
        let sdl_context = init_err!(sdl2::init())?;
        let _video_subsystem = init_err!(sdl_context.video())?;
        let _image_context = init_err!(image::init(image::InitFlag::PNG))?;
        let ttf_context = init_err!(sdl2::ttf::init())?;
        let window = init_err!(_video_subsystem
        .window(window_name, cam.get_window_size().x as u32, cam.get_window_size().y as u32)
        .opengl()
        .build())?;

    let canvas = init_err!(window
        .into_canvas()
        .present_vsync()
        .build())?;
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
    ///
    ///- 'cam_rect' the x,y part is the camera's offset the w,h is the target resolution of the drawing area
    ///- 'window_size' the size of the OS window made, does not need to match 'cam_rect'
    pub fn new(window_name: &str, cam_rect: Rect, window_size: Vec2) -> Result<(Camera, DrawingArea,ContextSdl), Error> {
        let cam = Camera::new(cam_rect, window_size);
        let (mut canvas, holder) = ContextSdl::new(&cam, window_name)?;
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        Ok((cam, DrawingArea { canvas }, holder))
    }
}

/// Holds ownership of a [DrawingArea] and the resource managers.
/// Also contains functions for doing the main update,draw loop
pub struct Render<'sdl> {
    pub texture_manager: TextureManager<'sdl, WindowContext>,
    pub font_manager: FontManager<'sdl, WindowContext>,
    pub controls: Controls,
    event_pump: EventPump,
    drawing_area: DrawingArea,
}


impl<'sdl> Render<'sdl> {
    pub fn new(drawing_area: DrawingArea, context: &ContextSdl) -> Result<Render, Error> {
        Ok(Render {
            texture_manager: TextureManager::new(&context.texture_creator),
            font_manager: FontManager::new(&context.ttf_context, &context.texture_creator),
            controls: Controls::new(),
            event_pump: init_err!(context.sdl_context.event_pump())?,
            drawing_area,
        })
    }
    /// Clears the [DrawingArea] for it to be filled with this frame's drawing instructions
    pub fn start_draw(&mut self) {
        self.drawing_area.canvas.set_draw_color(Color::BLACK);
        self.drawing_area.canvas.clear();
    }

    /// Drain the draws from [Camera] and draws to the canvas held by [DrawingArea]
    ///
    /// This is when the sdl drawing commands actually occur
    pub fn end_draw(&mut self, cam: &mut Camera) -> Result<(), Error>{
        
        for d in cam.drain_draws() {
            self.texture_manager.draw(&mut self.drawing_area.canvas, d)?;
        }
        for d in cam.drain_text_draws() {
            self.font_manager.draw_text_draw(&mut self.drawing_area.canvas, d)?;
        }
        for d in cam.drain_temp_text_draws() {
            self.font_manager.draw_disposable(&mut self.drawing_area.canvas, d)?;
        }
        for d in cam.drain_rect_draws() {
            self.texture_manager.draw_rect(&mut self.drawing_area.canvas, &d.0, d.1)?;
        }
        
      
        self.drawing_area.canvas.present();
        Ok(())
    }

    /// Update the controls struct using the sdl events that occured between the previous call.
    /// This should be called at the start of update.
    pub fn event_loop(&mut self) {
        self.controls.update(&mut self.event_pump);
    }

    /// Update the game window to the new size, and change the [Camera] to the new resolution
    ///
    /// Chnages the resolution of the Sdl Canvas and centeres the window
    pub fn set_win_size(&mut self, cam: &mut Camera, cs: Vec2) -> Result<(), Error> {
        match self.drawing_area.canvas.window_mut().set_size(cs.x as u32, cs.y as u32) {
            Err(_) => { return Err(Error::Sdl2ChangeState(String::from("failed to resize window")));},
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
