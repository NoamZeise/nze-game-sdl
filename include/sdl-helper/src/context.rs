use sdl2::{Sdl, VideoSubsystem, AudioSubsystem, image};
use sdl2::video::{Window, WindowContext};
use sdl2::render::{Canvas, TextureCreator};

use geometry::*;
use crate::{Error, init_err, Camera};

/// This holds ownership of many sdl types that are required for being able to use it,
/// but the context will not be changed after creation.
/// It is created by [DrawingArea]
pub struct ContextSdl {
    pub(crate) sdl_context : Sdl,
    _video_subsystem: VideoSubsystem,
    _audio_subsystem: AudioSubsystem, 
    _image_context: image::Sdl2ImageContext,
    pub(crate) ttf_context: sdl2::ttf::Sdl2TtfContext,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
}

impl ContextSdl {
    fn new(cam: &Camera, window_name: &str) -> Result<(Canvas<Window>, ContextSdl), Error> {
        let sdl_context = init_err!(sdl2::init())?;
        let _video_subsystem = init_err!(sdl_context.video())?;
        let _image_context = init_err!(image::init(image::InitFlag::PNG))?;
        let ttf_context = init_err!(sdl2::ttf::init())?;
        let window = init_err!(
            _video_subsystem
                .window(window_name, cam.get_window_size().x as u32, cam.get_window_size().y as u32)
                .opengl()
                .build()
        )?;
        let canvas = init_err!(
            window
                .into_canvas()
                .present_vsync()
                .build()
        )?;
        let texture_creator = canvas.texture_creator();

        let _audio_subsystem = init_err!(sdl_context.audio())?;
        
        Ok((canvas, ContextSdl { sdl_context, _video_subsystem, _audio_subsystem, _image_context, ttf_context, texture_creator}))
    }

    /// enable text input so that the `Control.input` get character function will return typed characters
    pub fn set_text_input(&mut self, is_enabled: bool) {
        if is_enabled {
            self._video_subsystem.text_input().start();
        } else {
            self._video_subsystem.text_input().stop();
        }
    }

    /// returns whether typing to keyboard for text input is enabled
    pub fn is_text_input_enabled(&self) -> bool {
        self._video_subsystem.text_input().is_active()
    }
}

/// Holds ownership of an sdl Canvas, this should be passed to `Render`
pub struct DrawingArea {
    pub(crate) canvas: Canvas<Window>
}

impl DrawingArea {
    ///returns the [ContextSdl] of this instance of sdl2, as well as a [DrawingArea]
    ///
    ///- `cam_rect` the `x`,`y` part is the camera's offset the `w`,`h` is the target resolution of the drawing area
    ///- `window_size` the resolution of the window, does not need to match `cam_rect`
    pub fn new(window_name: &str, cam_rect: Rect, window_size: Vec2) -> Result<(Camera, DrawingArea,ContextSdl), Error> {
        let cam = Camera::new(cam_rect, window_size);
        let (mut canvas, holder) = ContextSdl::new(&cam, window_name)?;
        println!("SDL2 context loaded...");
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        Ok((cam, DrawingArea { canvas }, holder))
    }
}
