#[doc(hidden)]
#[macro_export]
macro_rules! init_err {
     ($expression:expr)  => {
	 $expression.map_err(|e| {Error::Sdl2InitFailure(e.to_string())})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! file_err {
     ($expression:expr)  => {
	 $expression.map_err(|e| {Error::LoadFile(e)})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! draw_err {
     ($expression:expr)  => {
	 $expression.map_err(|e| {Error::Draw(e.to_string())})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! font_err {
     ($expression:expr)  => {
	 $expression.map_err(|e| {Error::TextRender(e.to_string())})
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! resource_err {
     ($expression:expr)  => {
	 $expression.map_err(|e| {Error::MissingResource(e.to_string())})
    };
}



