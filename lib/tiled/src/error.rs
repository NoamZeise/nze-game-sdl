//! Holds an enum of error types that can occur from loading tilemaps

#[derive(Debug)]
pub enum TiledError {
    /// This error occurs if the file cannot be opened, or if it cannot be parsed as plaintext
    FileReadError(String, String),
    /// This error occurs if data in tiled cannot be parsed into the appropriate type, may indicate a bug in the llibrary
    ParseError(String),
    /// Similar to 'ParseError' but occurs when the parsed data is stored in bytes
    ParseBytesError(),
    /// Can happen if a type is selected for a property but it isn't supported by this library yet, or if an option is selcted that this library doesn't recognize (maybe because Tiled has been updated)
    UnsupportedType(),
    /// Occurs when a poly shape does not have enough points to match the amount indicated
    MissingPoint(),
}
