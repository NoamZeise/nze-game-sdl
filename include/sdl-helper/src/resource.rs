//! Resources represent textures loaded into the active sdl2 context, where they can be used
//! by texture and font manager to get the actual resources to draw to the canvas,


/// A 2D texture loaded into an sdl context
#[derive(Clone, Copy)]
pub struct Texture {
    pub(crate) id:     usize,
    pub width:  u32,
    pub height: u32
}

//A font loaded with sdl tff 
#[derive(Clone, Copy)]
pub struct Font {
    pub(crate) id : usize,
}

/// can be returned by [FontManager], stores a reference to a texture held by font_manager
#[derive(Clone, Copy)]
pub struct Text {
    pub(crate) id: usize,
    pub width: u32,
    pub height: u32,
}
