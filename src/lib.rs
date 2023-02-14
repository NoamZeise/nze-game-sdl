//! A library to abstract away the details of the sdl2 library for creating games easier

pub mod input;
mod error;
mod resources;
mod render;
pub mod map;
mod camera;
mod rect_conversion;
mod error_macros;
mod context;

pub use error::Error;
pub use context::{ContextSdl, DrawingArea};
pub use render::Render;
pub use resources::resource;
pub use resources::audio;
/// resource managers which are created and held by other types
pub mod manager {
    pub use super::resources::texture_manager::TextureManager;
    pub use super::resources::font_manager::FontManager;
    pub use super::audio::{MusicManager, SfxManager};
}
pub use resources::types::{Colour, GameObject, TextObject};
pub use camera::Camera;
pub use geometry;
