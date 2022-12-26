use alloc::{vec::Vec, format};
use tracing::error;

#[derive(Debug)]
pub struct FontGlyph {
    pub charlie: char,
    pub width: u16,
    pub height: u16,
    pub off_x: isize,
    pub off_y: isize,
    pub display: &'static [bool]
}

impl FontGlyph {
    pub const fn new(
        charlie: char,
        (width, height): (u16, u16),
        (off_x, off_y): (isize, isize),
        display: &'static [bool]
    ) -> Self {
        Self {
            charlie,
            width,
            height,
            off_x,
            off_y,
            display,
        }
    }
}

pub const FONT_WIDTH: u16 = 7;
pub const FONT_HEIGHT: u16 = 15;

pub const FONT_SCALE: u16 = 1;

pub const FONTS: &[FontGlyph] = &[
    FontGlyph::new(
        '\u{0}', (7, 13), (0, 1),
        &[
            true, false, true, false, true, false, true,
            false, false, false, false, false, false, false,
            true, false, false, false, false, false, true,
            false, false, false, false, false, false, false,
            true, false, false, false, false, false, true,
            false, false, false, false, false, false, false,
            true, false, false, false, false, false, true,
            false, false, false, false, false, false, false,
            true, false, false, false, false, false, true,
            false, false, false, false, false, false, false,
            true, false, false, false, false, false, true,
            false, false, false, false, false, false, false,
            true, false, true, false, true, false, true
        ]
    ),
    FontGlyph::new(
        '\u{1}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{2}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{3}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{4}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{5}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{6}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{7}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{8}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new('\t', (1, 1), (0, 3), &[ false ]),
    FontGlyph::new(
        '\n', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{b}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{c}', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\r', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{e}', (4, 7), (0, 3),
        &[
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{f}', (4, 7), (3, 3),
        &[
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{10}', (4, 7), (3, 3),
        &[
            true, true, true, true,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false
        ]
    ),
    FontGlyph::new(
        '\u{11}', (4, 7), (0, 3),
        &[
            true, true, true, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true
        ]
    ),
    FontGlyph::new(
        '\u{12}', (7, 1), (0, 3),
        &[ true, true, true, true, true, true, true ]
    ),
    FontGlyph::new(
        '\u{13}', (1, 13), (3, 3),
        &[
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true
        ]
    ),
    FontGlyph::new(
        '\u{14}', (4, 13), (0, 3),
        &[
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            true, true, true, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true,
            false, false, false, true
        ]
    ),
    FontGlyph::new(
        '\u{15}', (7, 7), (0, 3),
        &[
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            true, true, true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{16}', (4, 13), (3, 3),
        &[
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, true, true, true,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false,
            true, false, false, false
        ]
    ),
    FontGlyph::new(
        '\u{17}', (7, 7), (0, 3),
        &[
            true, true, true, true, true, true, true,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false
        ]
    ),
    FontGlyph::new(
        '\u{18}', (7, 13), (0, 3),
        &[
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            true, true, true, true, true, true, true,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false,
            false, false, false, true, false, false, false
        ]
    ),
    FontGlyph::new(
        '\u{19}', (7, 13), (0, 3),
        &[
            true, false, true, false, true, false, true,
            false, true, false, true, false, true, false,
            true, false, true, false, true, false, true,
            false, true, false, true, false, true, false,
            true, false, true, false, true, false, true,
            false, true, false, true, false, true, false,
            true, false, true, false, true, false, true,
            false, true, false, true, false, true, false,
            true, false, true, false, true, false, true,
            false, true, false, true, false, true, false,
            true, false, true, false, true, false, true,
            false, true, false, true, false, true, false,
            true, false, true, false, true, false, true
        ]
    ),
    FontGlyph::new(
        '\u{1a}', (5, 5), (1, 3),
        &[
            false, false, false, true, false,
            true, true, true, true, true,
            false, false, true, false, false,
            true, true, true, true, true,
            false, true, false, false, false
        ]
    ),
    FontGlyph::new(
        '\u{1b}', (5, 7), (1, 0),
        &[
            false, false, false, true, true,
            false, true, true, false, false,
            true, false, false, false, false,
            false, true, true, false, false,
            false, false, false, true, true,
            false, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{1c}', (5, 6), (1, 0),
        &[
            true, true, true, true, true,
            false, true, false, true, false,
            false, true, false, true, false,
            false, true, false, true, false,
            false, true, false, true, false,
            false, true, false, true, false
        ]
    ),
    FontGlyph::new(
        '\u{1d}', (5, 7), (1, 0),
        &[
            true, true, false, false, false,
            false, false, true, true, false,
            false, false, false, false, true,
            false, false, true, true, false,
            true, true, false, false, false,
            false, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{1e}', (5, 8), (1, 0),
        &[
            false, false, true, true, false,
            false, true, false, false, false,
            false, true, false, false, false,
            true, true, true, true, false,
            false, true, false, false, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '\u{1f}', (3, 1), (2, 3),
        &[ true, true, true ]
    ),
    FontGlyph::new(
        ' ', (1, 1), (5, 3),
        &[ false ]
    ),
    FontGlyph::new(
        '!', (1, 8), (3, 3),
        &[
            true,
            true,
            true,
            true,
            true,
            true,
            false,
            true
        ]
    ),
    FontGlyph::new(
        '\"', (3, 3), (2, 9),
        &[
            true, false, true,
            true, false, true,
            true, false, true
        ]
    ),
    FontGlyph::new(
        '#', (5, 8), (1, 3),
        &[
            false, true, false, true, false,
            false, true, false, true, false,
            true, true, true, true, true,
            false, true, false, true, false,
            false, true, false, true, false,
            true, true, true, true, true,
            false, true, false, true, false,
            false, true, false, true, false
        ]
    ),
    FontGlyph::new(
        '$', (5, 10), (1, 3),
        &[
            false, false, true, false, false,
            false, true, true, true, false,
            true, false, true, false, true,
            true, false, true, false, false,
            false, true, true, true, false,
            false, false, true, false, true,
            false, false, true, false, true,
            true, false, true, false, true,
            false, true, true, true, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        '%', (5, 9), (1, 3),
        &[
            false, true, false, false, false,
            true, false, true, false, false,
            false, true, false, false, true,
            false, false, false, true, false,
            false, false, true, false, false,
            false, true, false, false, false,
            true, false, false, true, false,
            false, false, true, false, true,
            false, false, false, true, false
        ]
    ),
    FontGlyph::new(
        '&', (5, 9), (1, 3),
        &[
            false, false, true, false, false,
            false, true, false, true, false,
            false, true, false, true, false,
            false, false, true, false, false,
            false, true, true, false, true,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, true, false,
            false, true, true, false, true
        ]
    ),
    FontGlyph::new(
        '\'', (1, 3), (3, 9),
        &[
            true,
            true,
            true
        ]
    ),
    FontGlyph::new(
        '(', (3, 11), (2, 3),
        &[
            false, false, true,
            false, true, false,
            false, true, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            false, true, false,
            false, true, false,
            false, false, true
        ]
    ),
    FontGlyph::new(
        ')', (3, 11), (2, 3),
        &[
            true, false, false,
            false, true, false,
            false, true, false,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, true, false,
            false, true, false,
            true, false, false
        ]
    ),
    FontGlyph::new(
        '*', (5, 5), (1, 8),
        &[
            false, true, false, true, false,
            false, false, true, false, false,
            true, true, true, true, true,
            false, false, true, false, false,
            false, true, false, true, false
        ]
    ),
    FontGlyph::new(
        '+', (5, 5), (1, 4),
        &[
            false, false, true, false, false,
            false, false, true, false, false,
            true, true, true, true, true,
            false, false, true, false, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        ',', (2, 4), (2, 1),
        &[
            true, true,
            true, true,
            false, true,
            true, false
        ]
    ),
    FontGlyph::new(
        '-', (5, 1), (1, 6),
        &[ true, true, true, true, true ]
    ),
    FontGlyph::new(
        '.', (2, 2), (2, 3),
        &[
            true, true,
            true, true
        ]
    ),
    FontGlyph::new(
        '/', (5, 10), (1, 1),
        &[
            false, false, false, false, true,
            false, false, false, false, true,
            false, false, false, true, false,
            false, false, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false
        ]
    ),
    FontGlyph::new(
        '0', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        '1', (5, 8), (1, 3),
        &[
            false, false, true, false, false,
            false, true, true, false, false,
            true, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '2', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            false, false, false, false, true,
            false, false, false, true, false,
            false, false, true, false, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '3', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            false, false, false, false, true,
            false, false, true, true, false,
            false, false, false, false, true,
            false, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        '4', (6, 8), (1, 3),
        &[
            false, false, false, false, true, false,
            false, false, false, true, true, false,
            false, false, true, false, true, false,
            false, true, false, false, true, false,
            true, false, false, false, true, false,
            true, true, true, true, true, true,
            false, false, false, false, true, false,
            false, false, false, false, true, false
        ]
    ),
    FontGlyph::new(
        '5', (5, 8), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false,
            false, false, false, false, true,
            false, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        '6', (5, 8), (1, 3),
        &[
            false, false, true, true, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        '7', (5, 8), (1, 3),
        &[
            true, true, true, true, true,
            false, false, false, false, true,
            false, false, false, true, false,
            false, false, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, true, false, false, false,
            false, true, false, false, false
        ]
    ),
    FontGlyph::new(
        '8', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        '9', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, true,
            false, false, false, false, true,
            false, false, false, true, false,
            false, true, true, false, false
        ]
    ),
    FontGlyph::new(
        ':', (2, 6), (2, 3),
        &[
            true, true,
            true, true,
            false, false,
            false, false,
            true, true,
            true, true
        ]
    ),
    FontGlyph::new(
        ';', (2, 8), (2, 3),
        &[
            true, true,
            true, true,
            false, false,
            false, false,
            true, true,
            true, true,
            false, true,
            true, false
        ]
    ),
    FontGlyph::new(
        '<', (4, 7), (2, 3),
        &[
            false, false, false, true,
            false, false, true, false,
            false, true, false, false,
            true, false, false, false,
            false, true, false, false,
            false, false, true, false,
            false, false, false, true
        ]
    ),
    FontGlyph::new(
        '=', (5, 3), (1, 5),
        &[
            true, true, true, true, true,
            false, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '>', (4, 7), (1, 3),
        &[
            true, false, false, false,
            false, true, false, false,
            false, false, true, false,
            false, false, false, true,
            false, false, true, false,
            false, true, false, false,
            true, false, false, false
        ]
    ),
    FontGlyph::new(
        '?', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            false, false, false, false, true,
            false, false, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, false, false, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        '@', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, true, true, true,
            true, false, true, false, true,
            true, false, true, true, true,
            true, false, false, false, false,
            false, true, true, true, true
        ]
    ),
    FontGlyph::new(
        'A', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'B', (5, 8), (1, 3),
        &[
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'C', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'D', (5, 8), (1, 3),
        &[
            true, true, true, false, false,
            true, false, false, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, true, false,
            true, true, true, false, false
        ]
    ),
    FontGlyph::new(
        'E', (5, 8), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        'F', (5, 8), (1, 3),
        &[
            true, true, true, true, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false
        ]
    ),
    FontGlyph::new(
        'G', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'H', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'I', (3, 8), (2, 3),
        &[
            true, true, true,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            true, true, true
        ]
    ),
    FontGlyph::new(
        'J', (5, 8), (1, 3),
        &[
            false, false, true, true, true,
            false, false, false, false, true,
            false, false, false, false, true,
            false, false, false, false, true,
            false, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'K', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, true, false,
            true, false, true, false, false,
            true, true, true, false, false,
            true, false, false, true, false,
            true, false, false, true, false,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'L', (5, 8), (1, 3),
        &[
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        'M', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, true, false, true, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'N', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, true, false, false, true,
            true, true, false, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, false, true, true,
            true, false, false, true, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'O', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'P', (5, 8), (1, 3),
        &[
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false
        ]
    ),
    FontGlyph::new(
        'Q', (5, 9), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, true, false,
            false, true, true, false, true,
            false, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'R', (5, 8), (1, 3),
        &[
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, false,
            true, false, false, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'S', (5, 8), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, false,
            false, true, true, true, false,
            false, false, false, false, true,
            false, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'T', (5, 8), (1, 3),
        &[
            true, true, true, true, true,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        'U', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'V', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, false, true, false,
            false, true, false, true, false,
            false, true, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        'W', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            false, true, true, true, false,
            false, true, false, true, false,
            false, true, false, true, false
        ]
    ),
    FontGlyph::new(
        'X', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, true, false, true, false,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'Y', (5, 8), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        'Z', (5, 8), (1, 3),
        &[
            true, true, true, true, true,
            false, false, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '[', (3, 11), (2, 2),
        &[
            true, true, true,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, false, false,
            true, true, true
        ]
    ),
    FontGlyph::new(
        '\\', (5, 10), (1, 1),
        &[
            true, false, false, false, false,
            true, false, false, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, false, true, false,
            false, false, false, true, false,
            false, false, false, false, true,
            false, false, false, false, true
        ]
    ),
    FontGlyph::new(
        ']', (3, 11), (2, 2),
        &[
            true, true, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            true, true, true
        ]
    ),
    FontGlyph::new(
        '^', (5, 3), (1, 8),
        &[
            false, false, true, false, false,
            false, true, false, true, false,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        '_', (5, 1), (1, 2),
        &[ true, true, true, true, true ]
    ),
    FontGlyph::new(
        '`', (2, 2), (2, 10),
        &[
            true, false,
            false, true
        ]
    ),
    FontGlyph::new(
        'a', (5, 6), (1, 3),
        &[
            false, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, true, true,
            false, true, true, false, true
        ]
    ),
    FontGlyph::new(
        'b', (5, 9), (1, 3),
        &[
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'c', (5, 6), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'd', (5, 9), (1, 3),
        &[
            false, false, false, false, true,
            false, false, false, false, true,
            false, false, false, false, true,
            false, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, true
        ]
    ),
    FontGlyph::new(
        'e', (5, 6), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, true, true, true, true,
            true, false, false, false, false,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'f', (5, 9), (1, 3),
        &[
            false, false, true, true, true,
            false, true, false, false, false,
            false, true, false, false, false,
            true, true, true, true, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, true, false, false, false
        ]
    ),
    FontGlyph::new(
        'g', (5, 9), (1, 0),
        &[
            false, true, true, true, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, true,
            false, false, false, false, true,
            false, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'h', (5, 9), (1, 3),
        &[
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'i', (4, 8), (2, 3),
        &[
            false, true, false, false,
            false, false, false, false,
            true, true, false, false,
            false, true, false, false,
            false, true, false, false,
            false, true, false, false,
            false, true, false, false,
            false, false, true, true
        ]
    ),
    FontGlyph::new(
        'j', (3, 10), (2, 3),
        &[
            false, false, true,
            false, false, false,
            false, true, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            false, false, true,
            true, false, true,
            false, true, false
        ]
    ),
    FontGlyph::new(
        'k', (5, 9), (1, 3),
        &[
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, true,
            true, false, false, true, false,
            true, false, true, false, false,
            true, true, true, false, false,
            true, false, false, true, false,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'l', (3, 9), (2, 3),
        &[
            true, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, false,
            false, true, true
        ]
    ),
    FontGlyph::new(
        'm', (5, 6), (1, 3),
        &[
            true, true, false, true, false,
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            true, false, true, false, true
        ]
    ),
    FontGlyph::new(
        'n', (5, 6), (1, 3),
        &[
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'o', (5, 6), (1, 3),
        &[
            false, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'p', (5, 9), (1, 0),
        &[
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, true, true, true, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false
        ]
    ),
    FontGlyph::new(
        'q', (6, 9), (1, 0),
        &[
            false, true, true, true, true, false,
            true, false, false, false, true, false,
            true, false, false, false, true, false,
            true, false, false, false, true, false,
            true, false, false, false, true, false,
            false, true, true, true, true, false,
            false, false, false, false, true, false,
            false, false, false, false, true, false,
            false, false, false, false, true, true
        ]
    ),
    FontGlyph::new(
        'r', (5, 6), (1, 3),
        &[
            true, true, true, true, false,
            true, false, false, false, true,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false,
            true, false, false, false, false
        ]
    ),
    FontGlyph::new(
        's', (5, 6), (1, 3),
        &[
            false, true, true, true, true,
            true, false, false, false, false,
            false, true, true, true, false,
            false, false, false, false, true,
            false, false, false, false, true,
            true, true, true, true, false
        ]
    ),
    FontGlyph::new(
        't', (5, 8), (1, 3),
        &[
            false, true, false, false, false,
            false, true, false, false, false,
            true, true, true, true, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, true, false, false, false,
            false, false, true, true, true
        ]
    ),
    FontGlyph::new(
        'u', (5, 6), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, true
        ]
    ),
    FontGlyph::new(
        'v', (5, 6), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, false, true, false,
            false, true, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false
        ]
    ),
    FontGlyph::new(
        'w', (5, 6), (1, 3),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, true, false, true,
            true, false, true, false, true,
            false, true, false, true, false,
            false, true, false, true, false
        ]
    ),
    FontGlyph::new(
        'x', (5, 6), (1, 3),
        &[
            true, false, false, false, true,
            false, true, false, true, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, true, false, true, false,
            true, false, false, false, true
        ]
    ),
    FontGlyph::new(
        'y', (5, 9), (1, 0),
        &[
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            true, false, false, false, true,
            false, true, true, true, true,
            false, false, false, false, true,
            false, false, false, false, true,
            false, true, true, true, false
        ]
    ),
    FontGlyph::new(
        'z', (5, 6), (1, 3),
        &[
            true, true, true, true, true,
            false, false, false, true, false,
            false, false, true, false, false,
            false, true, false, false, false,
            true, false, false, false, false,
            true, true, true, true, true
        ]
    ),
    FontGlyph::new(
        '{', (5, 11), (1, 3),
        &[
            false, false, false, true, true,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            true, true, false, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, false, true, true
        ]
    ),
    FontGlyph::new(
        '|', (1, 13), (2, 1),
        &[
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true,
            true
        ]
    ),
    FontGlyph::new(
        '}', (5, 11), (1, 3),
        &[
            true, true, false, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, false, true, true,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            false, false, true, false, false,
            true, true, false, false, false
        ]
    ),
    FontGlyph::new(
        '~', (5, 3), (1, 9),
        &[
            false, true, false, false, true,
            true, false, true, false, true,
            true, false, false, true, false
        ]
    ),
];

pub fn validate_fonts() {
    // (0, 0) is at the bottom left of the glyph,
    // while `glyph.raster` starts at the top left

    for glyph in FONTS {
        let mut errors = Vec::new();

        assert_eq!(
            glyph.width * glyph.height,
            glyph.display.len() as u16,
            "Glyph {:?} WxH != display.len", glyph.charlie,
        );

        if glyph.width > FONT_WIDTH {
            errors.push(format!("width({} > {})", glyph.width, FONT_WIDTH));
        }

        if glyph.height > FONT_HEIGHT {
            errors.push(format!("height({} > {})", glyph.height, FONT_HEIGHT));
        }

        if glyph.off_x < 0 {
            errors.push(format!("off_x({} < 0)", glyph.off_x));
        }

        if glyph.off_y < 0 {
            errors.push(format!("off_y({} < 0)", glyph.off_y));
        }

        if glyph.off_x + glyph.width as isize > FONT_WIDTH as isize {
            errors.push(format!("off_x + width({} > {})", glyph.off_x + glyph.width as isize, FONT_WIDTH));
        }

        if glyph.off_y + glyph.height as isize > FONT_HEIGHT as isize {
            errors.push(format!("off_x + height({} > {})", glyph.off_y + glyph.height as isize, FONT_HEIGHT));
        }

        if !errors.is_empty() {
            error!("Glyph {:?} {}", glyph.charlie, errors.join(", "));
        }

    }
}