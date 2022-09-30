use geometry::*;
use crate::{TextureDraw, GameObject};

pub struct Camera {
    rect: Rect,
    window_size: Vec2,
}

impl Camera {
    pub fn new(rect: Rect, window_size: Vec2) -> Camera {
        Camera { rect, window_size}
    }
    
    pub fn to_camera_space(&self, game_obj: &GameObject) -> TextureDraw {
        TextureDraw::new(
            game_obj.texture,
            Rect::new(
                game_obj.rect.x - (self.rect.x * game_obj.parallax.x),
                game_obj.rect.y - (self.rect.y * game_obj.parallax.y),
                game_obj.rect.w,
                game_obj.rect.h
            ),
            game_obj.tex_rect,
        )
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
}
