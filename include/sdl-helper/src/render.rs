use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::video::WindowContext;

use crate::{Camera, TextureManager, FontManager, input::Controls, DrawingArea, Error, ContextSdl, init_err};
use geometry::Vec2;

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
    pub fn event_loop(&mut self, cam: &mut Camera) {
        self.controls.update(&mut self.event_pump);
        self.controls.input.mouse.pos = cam.window_to_cam_vec2(self.controls.input.mouse.pos);
    }

    /// Update the game window to the new size, and change the [Camera] to the new resolution
    ///
    /// Chnages the resolution of the Sdl Canvas and centeres the window
    pub fn set_win_size(&mut self, cam: &mut Camera, cs: Vec2, keep_view_ratio: bool) -> Result<(), Error> {
        // TODO keep view ratio true option
        cam.set_window_size(cs);
        match self.drawing_area.canvas.window_mut().set_size(cs.x as u32, cs.y as u32) {
            Err(_) => { return Err(Error::Sdl2ChangeState(String::from("failed to resize window")));},
            _ => ()
        }
        self.drawing_area.canvas.window_mut().set_position(
            sdl2::video::WindowPos::Centered,
            sdl2::video::WindowPos::Centered
        );
        Ok(())
    }
}
