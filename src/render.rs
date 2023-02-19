use sdl2::pixels::Color;
use sdl2::video::{WindowContext, FullscreenType};

use crate::camera::Draw;
use crate::{Camera, DrawingArea, Error, ContextSdl, helper_err};
use crate::manager::{FontManager, TextureManager};
use geometry::Vec2;

/// Holds ownership of a [DrawingArea] and texture and font managers, created using a [ContextSdl]
pub struct Render<'sdl> {
    pub texture_manager: TextureManager<'sdl, WindowContext>,
    pub font_manager: FontManager<'sdl, WindowContext>,
    drawing_area: DrawingArea,
}


impl<'sdl> Render<'sdl> {
    pub fn new(drawing_area: DrawingArea, context: &ContextSdl) -> Result<Render, Error> {
        Ok(Render {
            texture_manager: TextureManager::new(&context.texture_creator),
            font_manager: FontManager::new(&context.ttf_context, &context.texture_creator),
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
            match d {
                Draw::Texture(t) => self.texture_manager.draw(
                    &mut self.drawing_area.canvas, t)?,
                Draw::Rect(r, c) => self.texture_manager.draw_rect(
                    &mut self.drawing_area.canvas, &r, c)?,
                Draw::Text(t) => self.font_manager.draw_text_draw(
                    &mut self.drawing_area.canvas, t)?,
                Draw::DisposableText(t) => self.font_manager.draw_disposable(
                    &mut self.drawing_area.canvas, t)?,
            }
        }
        self.drawing_area.canvas.present();
        Ok(())
    }

    /// Update the game window to the new size, and change the [Camera] to the new resolution
    ///
    /// Changes the resolution of the Sdl Canvas and centeres the window
    pub fn set_win_size(&mut self, cam: &mut Camera, cs: Vec2) -> Result<(), Error> {
        cam.set_window_size(cs);
        helper_err!(self.drawing_area
            .canvas.window_mut()
            .set_size(cs.x as u32, cs.y as u32), Sdl2ChangeState)?;
        crate::context::set_canvas_logical_size(cam, &mut self.drawing_area.canvas)?;
        Ok(())
    }

    /// Set whether the window shoudl take up the full screen or not
    ///
    /// Fullscreen is really windowed borderless, taking up the whole monitor
    pub fn set_fullscreen(&mut self, cam: &mut Camera, fullscreen: bool) -> Result<(), Error> {
        helper_err!(
            self.drawing_area.canvas.window_mut().set_fullscreen(
                if fullscreen { FullscreenType::Desktop } else { FullscreenType::Off}
            ),
            Sdl2ChangeState)?;
        crate::context::set_canvas_logical_size(cam, &mut self.drawing_area.canvas)?;
        Ok(())
    }

    pub fn get_fullscreen(&self) -> bool {
        self.drawing_area.canvas.window().fullscreen_state() == FullscreenType::Desktop
    }

    /// Toggle from the current fullscreen state, to the opposite, and returns the new value.
    pub fn toggle_fullscreen(&mut self, cam: &mut Camera) -> Result<bool, Error> {
        Ok(if self.get_fullscreen() { self.set_fullscreen(cam, false)?; false } else {
            self.set_fullscreen(cam, true)?; true
        })
    }
}
