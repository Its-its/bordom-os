use core::fmt::Arguments;

use alloc::boxed::Box;

pub const OUTPUT_CODE: char = '\x7E';
pub const USER_INPUT_CODE: char = '\x7F';

#[allow(clippy::type_complexity)]
static mut GLOBAL_DISPATCH: Option<Box<dyn Fn(Arguments)>> = None;

pub fn set_global_dispatcher(func: impl Fn(Arguments) + 'static) {
    unsafe {
        GLOBAL_DISPATCH = Some(Box::new(func));
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    unsafe {
        if let Some(disp) = GLOBAL_DISPATCH.as_ref() {
            disp(args);
        }
    }
}