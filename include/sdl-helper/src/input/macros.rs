
/// macros for getting key bool values with less repetition
///
/// # down
///
/// **true if key is down this frame**
///
/// use down with the control struct held by render
///```
/// key!(render.controls, down[Key::A]) // True if key A is held down
///```
/// down shorthand
///```
///let input = render.controls.input; //store keyboard struct in a var called input
/// key!(input.down[Key::A]) // True if key A is held down
///```
///
/// # pressed
///
/// **true if key is down this frame but was up last frame**
///
/// using pressed with controls stored in render
///```
/// key!(render.controls,pressed[Key:A])
///```
///
#[macro_export]
macro_rules! key {
    ($controls:expr, down[$key:expr]) => {
	$controls.input.keys[$key as usize]
    };
    ($input:ident.down[$key:expr]) => {
        $input.keys[$key as usize]
    };
    ($controls:expr,pressed[$key:expr]) => {
       $controls.input.keys[$key as usize] && !$controls.prev_input.keys[$key as usize]
    };
}
