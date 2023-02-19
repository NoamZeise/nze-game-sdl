//! represent handles for resources loaded into the active sdl2 context, where they can be used
//! by types in [crate::manager] to get the actual resources to draw to the canvas,


/// A handle for a 2D texture loaded to memory
///
/// created by [crate::manager::TextureManager]
#[derive(Clone, Copy)]
pub struct Texture {
    pub(crate) id:     usize,
    pub width:  u32,
    pub height: u32
}

pub type Text = Texture;

/// A handle for a font loaded to memory and owned by [crate::manager::FontManager]
#[derive(Clone, Copy)]
pub struct Font {
    pub(crate) id : usize,
}

/// can be returned by [crate::manager::SfxManager], links to a sound effect held by the manager
#[derive(Clone, Copy)]
pub struct SoundEffect {
    pub(crate) id: usize,
}

/// can be returned by [crate::manager::MusicManager], links to a music held by the manager
#[derive(Clone, Copy)]
pub struct Music {
    pub(crate) id: usize,
}
