use gbl::io::color::ColorName;

pub trait ColorExt<'a> {
    fn fg(self, fg: ColorName) -> ColoredStr<'a>;
    // TODO:
    // fn background(&self, bg: Color) -> ColoredStr;
}

impl<'a> ColorExt<'a> for &'a str {
    fn fg(self, fg: ColorName) -> ColoredStr<'a> {
        ColoredStr { fg, s: self }
    }
}

impl<'a> ColorExt<'a> for ColoredStr<'a> {
    fn fg(self, fg: ColorName) -> ColoredStr<'a> {
        ColoredStr { fg, ..self }
    }
}

pub struct ColoredStr<'a> {
    pub fg: ColorName,
    // TODO: pub bg: Color,
    pub s: &'a str,
}

impl core::fmt::Display for ColoredStr<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Calling `write` directly causes the escape sequence to be printed out in chunks,
        // which breaks the framebuffer's ANSI parsing... Thus, we allocate a String containing
        // the full ANSI sequence, then print it out.
        let full = alloc::format!("\x1B[3{}m{}\x1B[3{}m", self.fg as u8, self.s, ColorName::DefaultForeground as u8);
        write!(f, "{full}")
    }
}

