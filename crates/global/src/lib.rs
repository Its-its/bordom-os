#![no_std]

extern crate alloc;

pub mod io;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print(format_args!("{}{}", $crate::io::OUTPUT_CODE, format_args!($($arg)*))));
}

#[macro_export]
macro_rules! println {
    ()            => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => ($crate::io::_print(format_args!("{}{}", $crate::io::USER_INPUT_CODE, format_args!($($arg)*))));
}