use alloc::boxed::Box;
use core::fmt::Arguments;

pub mod ansi;
pub mod color;

pub use color::{Color, ColorName};

#[allow(clippy::type_complexity)]
static mut GLOBAL_DISPATCH: Option<Box<dyn Fn(LogType, Arguments)>> = None;

pub fn set_global_dispatcher(func: impl Fn(LogType, Arguments) + 'static) {
    unsafe {
        GLOBAL_DISPATCH = Some(Box::new(func));
    }
}

#[doc(hidden)]
pub fn _print(type_of: LogType, args: Arguments) {
    unsafe {
        if let Some(disp) = GLOBAL_DISPATCH.as_ref() {
            disp(type_of, args);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogType {
    Output,
    UserInput,
}