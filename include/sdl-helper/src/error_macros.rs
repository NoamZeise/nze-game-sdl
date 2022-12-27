#[macro_export]
macro_rules! init_err {
     ($expression:expr)  => {
	 $expression.map_err(|e| {Error::Sdl2InitFailure(e.to_string())})
    };
}
