
#[derive(std::fmt::Debug)]
pub enum Error {
    Sdl2InitFailure(String),
    UnableToOpenFile(String),
    UnspecifiedError(String),
}
