//! A library to abstract away the details of the sdl2 library for creating games easier
//!
//! setup and game loop outline
//! ```
//! let (mut cam, drawing_area, context) = DrawingArea::new(
//!    "Name of Game",                              // window name
//!    geometry::Rect::new(0.0, 0.0, 400.0, 400.0), // window camera
//!    geometry::Vec2::new(400.0, 400.0)            // window size
//! )?;
//! let mut render = Render::new(drawing_area, &context)?;
//! let mut controls = Controls::new(&context)?;
//!
//! let mut obj = GameObject::new_from_tex(
//!     render.texture_manager.load(Path::new("textures/test.png"))?
//! );
//!
//! while !controls.should_close {
//!    controls.update(&cam);
//!
//!    if controls.kbm.down(Key::D) {
//!        obj.rect.x += 10.0 * controls.frame_elapsed;
//!    }
//!
//!    render.start_draw();
//!    cam.draw(&obj);
//!    render.end_draw(&mut cam)?;
//! }
//! ```

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
