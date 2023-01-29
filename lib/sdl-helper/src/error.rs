
/// Used by the library for reporting errors, so that the calling program can respond appropriately
#[derive(Debug)]
pub enum Error {
    /// Some part of sdl failed to initilaize, this error is usually unrecoverable and indicates an issue with the environment or with missing library files
    Sdl2InitFailure(String),
    /// Error from changing the state of sdl
    Sdl2ChangeState(String),
    /// This error indicates a requested resource failed to load, ie a texture or a font from 'TextureManager::load' or 'FontManager::load'
    LoadFile(String),
    /// Occurs if there was a problem drawing to the sdl2 'Canvas'
    Draw(String),
    /// Occurs if there was a problem creating a texture from a font and a string
    TextRender(String),
    /// Occurs if a resource that has been freed is used
    MissingResource(String),
    /// Occurs if an audio resource fails to play
    AudioPlay(String),
}
