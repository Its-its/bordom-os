#![no_std]

extern crate alloc;

pub mod io;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::_print($crate::io::LogType::Output, format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ()            => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! input {
    ($($arg:tt)*) => ($crate::io::_print($crate::io::LogType::UserInput, format_args!($($arg)*)));
}