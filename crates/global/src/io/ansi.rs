// https://en.wikipedia.org/wiki/ASCII
// https://en.wikipedia.org/wiki/ANSI_escape_code


// BiteMe / ByteMe
// #[byte_match(\x1B)]
// #[byte_match([\x1B, \x1B])]
// #[byte_store()]

use core::iter::Peekable;

use common::iter::{SaveStateInnerIter, SaveStateIterContainer};

use super::{Color, ColorName};


pub(crate) const ANSI_ESCAPES: &[&str] = &[
    "\x1B[30m", // Black
    "\x1B[31m", // Red
    "\x1B[32m", // Green
    "\x1B[33m", // Yellow
    "\x1B[34m", // Blue
    "\x1B[35m", // Magenta
    "\x1B[36m", // Cyan
    "\x1B[37m", // White

    // Not official ANSI codes
    "\x1B[38m", // Background
    "\x1B[39m", // Foreground
];

// Gruvbox material medium dark
pub(crate) const COLOR_THEME: &[Color] = &[
    Color::new(0x66, 0x5c, 0x54), // Black
    Color::new(0xEA, 0x69, 0x62), // Red
    Color::new(0xA9, 0xB6, 0x65), // Green
    Color::new(0xD8, 0xA6, 0x57), // Yellow
    Color::new(0x7D, 0xAE, 0xA3), // Blue
    Color::new(0xD3, 0x86, 0x9B), // Magenta
    Color::new(0x89, 0xB4, 0x82), // Cyan
    Color::new(0xDD, 0xC7, 0xA1), // White

    Color::new(0x29, 0x28, 0x28), // Background
    Color::new(0xEB, 0xDB, 0xD2), // Foreground
];

/// First
pub fn try_parse_ansi<I: Iterator<Item = char>>(value: &mut Peekable<I>) -> Option<Option<Ansi>> {
    let mut container = SaveStateIterContainer::new(value);

    let mut inner_iter = container.iter();

    if Ansi::does_iter_contain(&mut inner_iter) {
        Some(Ansi::parse(inner_iter))
    } else {
        None
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ansi {
    /// BEL
    Bell,
    /// BS
    Backspace,
    /// HT
    Tab,
    /// LF
    LineFeed,
    /// FF
    FormFeed,
    /// CR
    CarriageReturn,
    /// ESC
    Escape(AnsiEscape),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsiEscape {
    // 0x40-0x5F

    /// SS2
    SingleShiftTwo,
    /// SS3
    SingleShiftThree,
    /// DCS
    DeviceControlString,
    /// CSI
    ControlSequenceIntroducer(AnsiEscapeCSI),
    /// ST
    StringTerminator,
    /// OSC
    OperatingSystemCommand,
    /// SOS
    StartOfString,
    /// PM
    PrivacyMessage,
    /// APC
    ApplicationProgramCommand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsiEscapeCSI {
    CursorUp(usize),
    CursorDown(usize),
    CursorForward(usize),
    CursorBack(usize),
    CursorNextLine(i32),
    CursorPreviousLine(i32),
    CursorHorizontalAbsolute(i32),
    CursorPosition { rows: i32, columns: i32 },
    EraseInDisplay(EraseBy),
    EraseInLine(EraseBy),
    ScrollUp(Option<u32>),
    ScrollDown(Option<u32>),
    HorizontalVerticalPosition { rows: i32, columns: i32 },
    SelectGraphicRendition(GraphicsAttributes),
    // AUXPortOn
    // AUXPortOff
    // DeviceStatusReport

    SaveCurrentCursorPosition,
    RestoreSavedCursorPosition
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EraseBy {
    #[default]
    ClearToEnd,
    ClearToStart,
    ClearAll,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GraphicsAttributes {
    // Reset or normal
    // Bold or increased intensity
    // Faint, decreased intensity, or dim
    // Italic
    // Underline
    // Slow blink
    // Rapid blink
    // Reverse video or invert
    // Conceal or hide
    // Crossed-out, or strike
    // Primary (default) font
    // Alternative font
    // Fraktur (Gothic)
    // Doubly underlined; or: not bold
    // Normal intensity
    // Neither italic, nor blackletter
    // Not underlined
    // Not blinking
    // Proportional spacing
    // Not reversed
    // Reveal
    // Not crossed out
    pub foreground_color: Option<ColorName>,
    // Set foreground color
    // Default foreground color
    pub background_color: Option<ColorName>,
    // Set background color
    // Default background color
    // Disable proportional spacing
    // Framed
    // Encircled
    // Overlined
    // Neither framed nor encircled
    // Not overlined
    // Set underline color
    // Default underline color
    // Ideogram underline or right side line
    // Ideogram double underline, or double line on the right side
    // Ideogram overline or left side line
    // Ideogram double overline, or double line on the left side
    // Ideogram stress marking
    // No ideogram attributes
    // Superscript
    // Subscript
    // Neither superscript nor subscript
    // Set bright foreground color
    // Set bright background color
}

impl Ansi {
    fn does_iter_contain<I: Iterator<Item = char>>(iter: &mut SaveStateInnerIter<char, I>) -> bool {
        if let Some(item) = iter.peek() {
            ['\x1B'].contains(item)
        } else {
            false
        }
    }

    fn parse<I: Iterator<Item = char>>(mut iter: SaveStateInnerIter<char, I>) -> Option<Self> {
        #[allow(clippy::single_match)]
        match iter.next()? {
            '\x1B' => Some(Self::Escape(AnsiEscape::parse(iter)?)),

            _ => None,
        }
    }
}

impl AnsiEscape {
    fn parse<I: Iterator<Item = char>>(mut iter: SaveStateInnerIter<char, I>) -> Option<Self> {
        #[allow(clippy::single_match)]
        match iter.next()? {
            '[' => Some(Self::ControlSequenceIntroducer(AnsiEscapeCSI::parse(iter)?)),

            _ => None
        }
    }
}

impl AnsiEscapeCSI {
    fn parse<I: Iterator<Item = char>>(mut iter: SaveStateInnerIter<char, I>) -> Option<Self> {
        let state = iter.save_state();

        let end_letter = {
            let mut found = None;

            for item in &mut iter {
                if ('a'..='z').contains(&item) || ('A'..='Z').contains(&item) || item == ';' {
                    found = Some(item);
                    break;
                }
            }

            found?
        };

        iter.load_state(state);

        match end_letter {
            'A' => {
                let amount = get_int(&mut iter).unwrap_or(1);

                let _ = iter.next()?;

                Some(Self::CursorUp(amount))
            }

            'B' => {
                let amount = get_int(&mut iter).unwrap_or(1);

                let _ = iter.next()?;

                Some(Self::CursorDown(amount))
            }

            'C' => {
                let amount = get_int(&mut iter).unwrap_or(1);

                let _ = iter.next()?;

                Some(Self::CursorForward(amount))
            }

            'D' => {
                let amount = get_int(&mut iter).unwrap_or(1);

                let _ = iter.next()?;

                Some(Self::CursorBack(amount))
            }

            // n E    Cursor Next Line
            // n F    Cursor Previous Line
            // n G    Cursor Horizontal Absolute
            // n;m H  Cursor Position
            // n J    Erase in Display
            // n K    Erase in Line
            // n S    Scroll Up
            // n T    Scroll Down
            // n;m f  Horizontal Vertical Position

            'm' => {
                let mut attr = GraphicsAttributes::default();

                // TODO: Handle ;
                loop {
                    let num = get_int(&mut iter).unwrap_or_default();

                    match num {
                        // 0 	    Reset or normal
                        // 1 	    Bold or increased intensity
                        // 2 	    Faint, decreased intensity, or dim
                        // 3 	    Italic
                        // 4 	    Underline
                        // 5 	    Slow blink
                        // 6 	    Rapid blink
                        // 7 	    Reverse video or invert
                        // 8 	    Conceal or hide
                        // 9 	    Crossed-out, or strike
                        // 10 	    Primary (default) font
                        // 11–19 	Alternative font
                        // 20 	    Fraktur (Gothic)
                        // 21 	    Doubly underlined; or: not bold
                        // 22 	    Normal intensity
                        // 23 	    Neither italic, nor blackletter
                        // 24 	    Not underlined
                        // 25 	    Not blinking
                        // 26 	    Proportional spacing
                        // 27 	    Not reversed
                        // 28 	    Reveal
                        // 29 	    Not crossed out
                        30..=37 => attr.foreground_color = Some(ColorName::from_u8((num - 30) as u8)),
                        // 38 	    Set foreground color
                        39 => attr.foreground_color = Some(ColorName::DefaultForeground),
                        40..=47 => attr.background_color = Some(ColorName::from_u8((num - 40) as u8)),
                        // 48 	    Set background color
                        49 => attr.background_color = Some(ColorName::DefaultBackground),
                        // 50 	    Disable proportional spacing
                        // 51 	    Framed
                        // 52 	    Encircled
                        // 53 	    Overlined
                        // 54 	    Neither framed nor encircled
                        // 55 	    Not overlined
                        // 58 	    Set underline color
                        // 59 	    Default underline color
                        // 60 	    Ideogram underline or right side line
                        // 61 	    Ideogram double underline, or double line on the right side
                        // 62 	    Ideogram overline or left side line
                        // 63 	    Ideogram double overline, or double line on the left side
                        // 64 	    Ideogram stress marking
                        // 65 	    No ideogram attributes
                        // 73 	    Superscript
                        // 74 	    Subscript
                        // 75 	    Neither superscript nor subscript
                        // 90–97 	Set bright foreground color
                        // 100–107 	Set bright background color

                        _ => ()
                    }

                    if iter.next()? == 'm' {
                        break;
                    }
                }

                Some(Self::SelectGraphicRendition(attr))
            }

            // 5i  AUX Port On
            // 4i  AUX Port Off
            // 6n  Device Status Report

            _ => None
        }
    }
}

impl From<AnsiEscapeCSI> for Ansi {
    fn from(value: AnsiEscapeCSI) -> Self {
        Self::Escape(AnsiEscape::ControlSequenceIntroducer(value))
    }
}


fn get_int<I: Iterator<Item = char>>(iter: &mut SaveStateInnerIter<char, I>) -> Option<usize> {
    let mut value = 0;

    while ('0'..='9').contains(iter.peek()?) {
        let c = iter.next()?;
        value = value * 10 + c.to_digit(10).unwrap() as usize;
    }

    Some(value).filter(|&v| v != 0)
}

#[cfg(test)]
mod tests {
    use common::iter::SaveStateIterContainer;

    use super::*;

    fn parse(value: &str) -> Option<Ansi> {
        let mut chars = value.chars().peekable();
        let mut inner = SaveStateIterContainer::new(&mut chars);

        Ansi::parse(inner.iter())
    }

    #[test]
    fn test_name() {
        assert_eq!(parse("\x1B[30m"), Some(AnsiEscapeCSI::SelectGraphicRendition(GraphicsAttributes { foreground_color: Some(ColorName::Black), ..Default::default() }).into()));
    }
}