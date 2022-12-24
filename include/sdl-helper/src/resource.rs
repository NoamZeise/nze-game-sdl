//! Structs that the resource managers in 'Render' use as handles to sdl resources

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
