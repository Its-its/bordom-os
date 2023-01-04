use core::str::FromStr;

use super::ansi::{COLOR_THEME, ANSI_ESCAPES};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b }
    }

    pub fn to_framebuffer_pixel(self) -> [u8; 3] {
        [self.b, self.g, self.r]
    }
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(hex) = s.strip_prefix('#') {
            if hex.len() == 6 {
                if !hex.chars().any(|c| !c.is_ascii_hexdigit()) {
                    let r = &hex[0..2];
                    let g = &hex[2..4];
                    let b = &hex[4..6];

                    let r = u8::from_str_radix(r, 16).unwrap();
                    let g = u8::from_str_radix(g, 16).unwrap();
                    let b = u8::from_str_radix(b, 16).unwrap();

                    Ok(Color { r, g, b })
                } else {
                    Err("Color hex code must be valid hexadecimal")
                }
            } else {
                Err("Color hex code must be 6 characters long")
            }
        } else {
            Err("Color hex code must start with '#'")
        }
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorName {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    DefaultBackground,
    DefaultForeground,
}

impl ColorName {
    pub fn color(self) -> Color {
        COLOR_THEME[self as usize]
    }

    pub fn ansi(self) -> &'static str {
        ANSI_ESCAPES[self as usize]
    }

    pub fn from_u8(value: u8) -> Self {
        use ColorName::*;

        match value {
            0 => Black,
            1 => Red,
            2 => Green,
            3 => Yellow,
            4 => Blue,
            5 => Magenta,
            6 => Cyan,
            7 => White,

            8 => DefaultBackground,
            9 => DefaultForeground,

            v => unimplemented!("{v}")
        }
    }
}