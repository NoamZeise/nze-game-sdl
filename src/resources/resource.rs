//! Resources represent textures loaded into the active sdl2 context, where they can be used
//! by `TextureManager` and `FontManager` to get the actual resources to draw to the canvas,


/// A handle for a 2D texture loadememory and owned by `TextureManager`
#[derive(Clone, Copy)]
pub struct Texture {
    pub(crate) id:     usize,
    pub width:  u32,
    pub height: u32
}

/// A handle for a font loaded to memory and owned by `FontManager`
#[derive(Clone, Copy)]
pub struct Font {
    pub(crate) id : usize,
}

/// can be returned by `FontManager`, stores a reference to a texture owned by `FontManager `
#[derive(Clone, Copy)]
pub struct Text {
    pub(crate) id: usize,
    pub width: u32,
    pub height: u32,
}

/// can be returned by 'AudioManager', links to a sound effect held by the manager
#[derive(Clone, Copy)]
pub struct SoundEffect {
    pub(crate) id: usize,
}

/// can be returned by 'AudioManager', links to a music held by the manager
#[derive(Clone, Copy)]
pub struct Music {
    pub(crate) id: usize,
}
