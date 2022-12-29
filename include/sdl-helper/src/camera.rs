use geometry::*;
use crate::{texture_manager::TextureDraw, font_manager::{DisposableTextDraw, TextDraw}, types::GameObject, Colour, resource, types::TextObject};
use std::vec::Drain;

/// holds buffered draw commands that render will consume at the end of each frame
///
/// Note: need to pass this to 'Render' at the end of each frame in order to see the draws on the screen
///
/// Note: set parallax to 0 if you want the object to be unaffected by the moving camera
pub struct Camera {
    rect: Rect,
    window_size: Vec2,
    size_ratio: Vec2,
    draws : Vec<TextureDraw>,
    rect_draws: Vec<(Rect, Colour)>,
    temp_text_draws: Vec<DisposableTextDraw>,
    perm_text_draws: Vec<TextDraw>,
}

impl Camera {
    pub fn new(rect: Rect, window_size: Vec2) -> Camera {
        let mut cam = Camera {
            rect,
            window_size,
            draws: Vec::new(),
            temp_text_draws: Vec::new(),
            perm_text_draws: Vec::new(),
            rect_draws: Vec::new(),
            size_ratio: Vec2::new(0.0, 0.0),
        };
        cam.update_size_ratio();
        cam
    }
    
    pub(crate) fn drain_draws(&mut self) -> Drain<TextureDraw> { 
        self.draws.drain(..)
    }

    pub(crate) fn drain_temp_text_draws(&mut self) -> Drain<DisposableTextDraw> { 
        self.temp_text_draws.drain(..)
    }

    pub(crate) fn drain_text_draws(&mut self) -> Drain<TextDraw> {
        self.perm_text_draws.drain(..)
    }

    pub(crate) fn drain_rect_draws(&mut self) -> Drain<(Rect, Colour)> {
        self.rect_draws.drain(..)
    }

    /// Draws a [GameObject] adjusted for the camera's position and scale
    pub fn draw(&mut self, game_obj: &GameObject) {
        self.draws.push(
            TextureDraw::new(
                game_obj.get_texture(),
                self.rect_to_cam_space(game_obj.rect, game_obj.parallax),
                game_obj.tex_rect,
                game_obj.colour,
            )
        );
    }

    /// Draws text adjusted for the camera's position and scale
    pub fn draw_disposable_text(&mut self, font: &resource::Font, text: String, height: u32, pos: Vec2, colour: Colour, parallax: Vec2) {
        let rect = self.rect_to_cam_space(Rect::new(pos.x, pos.y, height as f64, height as f64), parallax);
        self.temp_text_draws.push(DisposableTextDraw {
            font: *font,
            text,
            height: rect.h as u32,
            pos: rect.top_left(),
            colour,
        })
    }

    pub fn draw_text(&mut self, text_obj: &TextObject) {
        let rect = self.rect_to_cam_space(text_obj.rect, text_obj.parallax);
        self.perm_text_draws.push(TextDraw {
            text: text_obj.texture,
            rect,
            colour: text_obj.colour,
        })
    }

    pub fn draw_rect(&mut self, rect: Rect, colour: Colour) {
        self.rect_draws.push((rect, colour));
    }

    pub fn get_offset(&self) -> Vec2 {
        return Vec2::new(self.rect.x, self.rect.y);
    }

    pub fn set_offset(&mut self, offset: Vec2) {
        self.rect.x = offset.x;
        self.rect.y = offset.y;
    }

    pub fn get_window_size(&self) -> Vec2 {
        self.window_size
    }

    pub fn set_window_size(&mut self, size: Vec2) {
        self.window_size = size;
        self.update_size_ratio();
    }

    pub fn get_view_size(&self) -> Vec2 {
        Vec2::new(self.rect.w, self.rect.h)
    }
    pub fn set_view_size(&mut self, view: Vec2) {
        self.rect.w = view.x;
        self.rect.h = view.y;
        self.update_size_ratio();
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.rect.w / self.rect.h
    }

    pub fn window_to_cam_vec2(&self, pos: Vec2) -> Vec2 {
        (self.size_ratio * pos) + self.get_offset()
    }

    fn update_size_ratio(&mut self) {
        self.size_ratio = Vec2::new(
                self.rect.w / self.window_size.x,
                self.rect.h / self.window_size.y
        );
    }

    fn rect_to_cam_space(&self, rect: Rect, parallax: Vec2) -> Rect {
        Rect::new(
            ((rect.x) - (self.rect.x * parallax.x)) / self.size_ratio.x,
            ((rect.y) - (self.rect.y * parallax.y)) / self.size_ratio.y,
            rect.w / self.size_ratio.x,
            rect.h / self.size_ratio.y,
        )
    }
}
