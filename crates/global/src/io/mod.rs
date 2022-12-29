use core::fmt::Arguments;

use alloc::boxed::Box;

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

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Output,
    UserInput,
}