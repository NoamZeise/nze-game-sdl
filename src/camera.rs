use geometry::*;
use crate::resources::{texture_manager::TextureDraw, font_manager::{DisposableTextDraw, TextDraw}, types::GameObject, resource, types::TextObject};
use crate::Colour;
use std::vec::Drain;

/// Used for drawing to the canvas.
///
/// The window size and screen size and position are used to adjust the size of the rects sent as 
/// draw commands to the sdl `Canvas`
///
/// This holds buffered draw commands that render will consume at the end of each frame, so that the commands
/// need to be resubmitted each frame they are to be drawn.
///
/// # Notes:
/// - passed to `Render` at the end of each frame in order to submit the draw commands
/// - the rects are scaled according to the `view size` which represents the target screen resolution
///   and the `window size`, which represents the resolution of the game's window
/// - the rects are moved according to camera's `offset`
/// - The `parallax` of the draws will affect how much the camera's `offset` changes the object's position,
/// set `parallax` to 0 if you want the object to be unaffected by the moving camera
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
    pub(crate) fn new(rect: Rect, window_size: Vec2) -> Camera {
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

    /// Draws a [GameObject] adjusted for the camera's `view`
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

    /// Draws a disposable text texture adjusted for the camera's `view`
    pub fn draw_disposable_text(&mut self, font: &resource::Font, text: String, height: u32, pos: Vec2, colour: Colour, parallax: Vec2) {
        let rect = self.rect_to_cam_space(Rect::new(pos.x, pos.y, height as f64, height as f64), parallax);
        self.temp_text_draws.push(DisposableTextDraw {
            font: *font,
            text,
            height: rect.h as u32,
            pos: rect.top_left(),
            colour,
            rect,
        })
    }

    /// Draws a [TextObject] adjusted by camera's `view` 
    pub fn draw_text(&mut self, text_obj: &TextObject) {
        let rect = self.rect_to_cam_space(text_obj.rect, text_obj.parallax);
        self.perm_text_draws.push(TextDraw {
            text: text_obj.texture,
            rect,
            colour: text_obj.colour,
        })
    }

    /// Draw a [Rect] with a [Colour] adjusted by the camera's `view` 
    pub fn draw_rect(&mut self, rect: Rect, colour: Colour, parallax: Vec2) {
        self.rect_draws.push((self.rect_to_cam_space(rect, parallax), colour));
    }

    /// Get the current view offset
    pub fn get_offset(&self) -> Vec2 {
        return Vec2::new(self.rect.x, self.rect.y);
    }

    /// Set the camera's view offset
    ///
    /// The view offset will move all draws by the offset multiplied by the object's `parallax`
    pub fn set_offset(&mut self, offset: Vec2) {
        self.rect.x = offset.x;
        self.rect.y = offset.y;
    }

    /// Get the current window size
    ///
    /// The window size is the resolution of the window drawn by the OS.
    /// This must be set by using `render.set_win_size`.
    pub fn get_window_size(&self) -> Vec2 {
        self.window_size
    }

    pub(crate) fn set_window_size(&mut self, size: Vec2) {
        self.window_size = size;
        self.update_size_ratio();
    }

    /// Get the camera's view size
    pub fn get_view_size(&self) -> Vec2 {
        Vec2::new(self.rect.w, self.rect.h)
    }

    /// Set the camera's view size
    ///
    /// The view size is the resolution of your game, which may be different from your window resolution
    pub fn set_view_size(&mut self, view: Vec2) {
        self.rect.w = view.x;
        self.rect.h = view.y;
        self.update_size_ratio();
    }

    /// Get the current aspect ratio (the view width divided by the view height)
    pub fn aspect_ratio(&self) -> f64 {
        self.rect.w / self.rect.h
    }

    // Transform a pos from window space to cam space
    pub(crate) fn window_to_cam_vec2(&self, pos: Vec2) -> Vec2 {
        (self.size_ratio * pos) + self.get_offset()
    }

    fn update_size_ratio(&mut self) {
        self.size_ratio = Vec2::new(
                self.rect.w / self.window_size.x,
                self.rect.h / self.window_size.y
        );
    }

    pub(crate) fn rect_to_cam_space(&self, rect: Rect, parallax: Vec2) -> Rect {
        Rect::new(
            ((rect.x) - (self.rect.x * parallax.x)) / self.size_ratio.x,
            ((rect.y) - (self.rect.y * parallax.y)) / self.size_ratio.y,
            rect.w / self.size_ratio.x,
            rect.h / self.size_ratio.y,
        )
    }
}
