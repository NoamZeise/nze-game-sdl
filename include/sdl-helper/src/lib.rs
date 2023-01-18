//! A library to abstract away the details of the sdl2 library for creating games easier

pub mod input;
mod error;
mod resources;
mod render;
mod map;
mod camera;
mod rect_conversion;
mod error_macros;
mod context;
pub mod audio;

pub use error::Error;
pub use context::{ContextSdl, DrawingArea};
pub use render::Render;
pub use resources::resource;
pub use resources::texture_manager::TextureManager;
pub use resources::font_manager::FontManager;
pub use resources::types::{Colour, GameObject, TextObject};
pub use camera::Camera;
pub use map::Map;
