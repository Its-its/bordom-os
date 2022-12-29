use core::fmt::Write;

use alloc::{collections::VecDeque, vec::Vec, string::String, vec};
use bootloader_api::info::FrameBufferInfo;
use common::Position;
use gbl::io::{OUTPUT_CODE, USER_INPUT_CODE};
use spin::{Mutex, Once};

use crate::{font, color::{ColorName, Color}};

pub static FB_WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

pub(super) fn init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    FB_WRITER.call_once(|| FrameBufferWriter::new(buffer, info).into());

    gbl::io::set_global_dispatcher(_print);
}

pub struct TextStyle {
    foreground: Color,
    background: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            foreground: ColorName::Foreground.color(),
            background: ColorName::Background.color(),
        }
    }
}

enum CacheType {
    Char(char),
    Ansi(Vec<char>),
}

impl CacheType {
    pub fn is_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }
}

pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
    text_style: TextStyle,
    bytes_per_pixel: usize,

    cached_lines: VecDeque<Vec<CacheType>>,

    cursor_pos: Position<u16>,
    displaying_cursor: bool,

    input_height: u16,
    user_input: Vec<char>,
    is_printing: bool,
}

impl FrameBufferWriter {
    fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut fb = FrameBufferWriter {
            buffer,
            info,
            text_style: TextStyle::default(),
            bytes_per_pixel: info.bytes_per_pixel,

            cached_lines: VecDeque::with_capacity(500),

            cursor_pos: Position::default(),
            displaying_cursor: false,

            input_height: 1,
            user_input: Vec::new(),
            is_printing: true,
        };

        fb.clear();
        fb.cursor_pos.set_y(fb.screen_pixel_height() - 1);

        fb
    }

    pub fn tick(&mut self) {
        self.displaying_cursor = !self.displaying_cursor;

        if self.displaying_cursor {
            let curr = self.text_style.foreground;
            self.text_style.foreground = ColorName::Green.color();
            self.draw_glyph_in_cell(self.cursor_pos.inner(), '_');
            self.text_style.foreground = curr;
        } else {
            self.clear_cell(self.cursor_pos.inner());
        }
    }

    fn clear(&mut self) {
        self.cached_lines.clear();
        self.cached_lines.push_back(Vec::new());

        let bg = ColorName::Background.color().to_framebuffer_pixel();

        // This is faster than using for i in 0..num_subpixels,
        // since for loops use the `Iterator` trait under the hood,
        // which uses Clone, rather than Copy.
        //
        // This could likely be optimized even further with some
        // cursed pointer stuff, but it is fast enough as is.
        let mut i = 0;
        let num_subpixels = self.buffer.len();
        while i < num_subpixels {
            self.buffer[i] = bg[i % 3];

            i += 1;
        }
    }

    fn move_buffer_up(&mut self) {
        // TODO: Move buffer based on cached_lines.
        let font_size = (font::FONT_HEIGHT * font::FONT_SCALE) as usize;

        let buffer_line = self.info.width * self.bytes_per_pixel;

        let font_buffer_line_size = font_size * buffer_line;
        let line_offset = buffer_line * 2;
        // TODO: Line offset may be incorrect.

        // Move Buffer Up
        let mut i = 0;
        let num_subpixels = self.buffer.len() - (font_buffer_line_size + line_offset);
        while i < num_subpixels {
            self.buffer[i + line_offset] = self.buffer[i + font_buffer_line_size + line_offset];

            i += 1;
        }

        // Clear last line
        let bg = ColorName::Background.color().to_framebuffer_pixel();

        let mut i = self.buffer.len() - font_buffer_line_size;
        let num_subpixels = self.buffer.len();
        while i < num_subpixels {
            self.buffer[i] = bg[i % 3];

            i += 1;
        }
    }

    fn process_buffer_check(&mut self) {
        if self.cached_lines.len() == 500 {
            self.cached_lines.pop_front();
        }

        if self.cached_lines.len() as u16 + self.input_height >= self.screen_pixel_height() {
            self.move_buffer_up();
        //     self.cursor_pos.set_x(0);
        // } else {
        //     self.cursor_pos.set_x(0);
        //     self.cursor_pos.inc_y(1);
        }

        self.cached_lines.push_back(Vec::new());
    }

    // TODO: Check string for new line? Move up first then render. Would fix self.buffer overflow
    fn process_string(&mut self, s: &str) {
        let mut chars = s.chars();

        while let Some(char) = chars.next() {
            // Check to see if we're outputting to console or user input
            if char == OUTPUT_CODE {
                self.is_printing = true;
                continue;
            } else if char == USER_INPUT_CODE {
                self.is_printing = false;
                continue;
            }

            if char == '\n' {
                if self.is_printing {
                    self.process_buffer_check();
                } else {
                    {
                        let mut pos = self.cursor_pos;

                        for i in 0..pos.x() + 1 {
                            pos.set_x(i);
                            self.clear_cell(pos.inner());
                        }
                    }

                    self.is_printing = true;

                    self.cursor_pos.set_x(0);

                    let input = core::mem::take(&mut self.user_input);
                    self.write_fmt(format_args!("{}\n", input.into_iter().collect::<String>())).unwrap();

                    self.is_printing = false;
                }

                continue;
            }

            let last_cached_row = self.cached_lines.back_mut().unwrap();

            // Backspace
            if char == '\x08' {
                let last_pos = self.cursor_pos.inner();

                if self.cursor_pos.x() != 0 {
                    self.cursor_pos.dec_x(1);

                    if self.is_printing {
                        last_cached_row.pop();
                    } else {
                        self.user_input.pop();
                    }
                } else {
                    // TODO
                }

                self.clear_cell(last_pos);

                continue;
            }

            // Escape Sequences
            if char == '\x1B' {
                #[allow(clippy::single_match)]
                match chars.next() {
                    Some('[') => {
                        // TODO: Remove alloc?
                        let mut items = Vec::new();

                        for item in &mut chars {
                            items.push(item);

                            if ('a'..='z').contains(&item) || ('A'..='Z').contains(&item) {
                                break;
                            }
                        }

                        let end_letter = items.pop();

                        items.reverse();

                        match end_letter {
                            // Foreground / Background - \x1B[30m
                            Some('m') if items.len() == 2 => {
                                // TODO: Handle 0 - 107 (items.len() 1, 2, and 3)
                                let mode = items.pop().unwrap();
                                let color = items.pop().unwrap();

                                last_cached_row.push(CacheType::Ansi(vec!['\x1B', '[', mode, color, 'm']));

                                if mode != '3' && mode != '4' { continue }

                                let color_index = color as u8 - b'0';
                                let color = ColorName::from_u8(color_index).color();

                                match mode {
                                    '3' => self.text_style.foreground = color,
                                    '4' => self.text_style.background = color,

                                    _ => ()
                                }
                            }

                            // Cursor Up
                            Some('A') if !items.is_empty() => {
                                // TODO
                            }

                            // Cursor Down
                            Some('B') if !items.is_empty() => {
                                // TODO
                            }

                            // Cursor Forward
                            Some('C') if !items.is_empty() => {
                                let amount = items.into_iter().fold(
                                    0,
                                    |a, c| a * 10 + c.to_digit(10).unwrap()
                                ) as u16;

                                self.cursor_pos.inc_x(self.cursor_pos.x().min(amount));
                            }

                            // Cursor Back
                            Some('D') if !items.is_empty() => {
                                let amount = items.into_iter().fold(
                                    0,
                                    |a, c| a * 10 + c.to_digit(10).unwrap()
                                ) as u16;

                                self.cursor_pos.dec_x(amount);
                            }

                            _ => ()
                        }
                    }

                    _ => ()
                }

                continue;
            }

            if self.is_printing {
                last_cached_row.push(CacheType::Char(char));

                let last_line_size = last_cached_row.iter().filter(|c| c.is_char()).count().saturating_sub(1);
                let buff_len = (self.cached_lines.len() - 1).min(self.screen_pixel_height() as usize - 1);

                self.draw_glyph_in_cell((last_line_size as u16, buff_len as u16), char);
            } else {
                self.user_input.push(char);

                self.clear_cell(self.cursor_pos.inner());
                self.draw_glyph_in_cell(self.cursor_pos.inner(), char);

                if self.cursor_pos.x() + 1 >= self.screen_pixel_width() {
                    // self.cursor_next_line();
                    // TODO: handle
                } else {
                    self.cursor_pos.inc_x(1);
                }
            }
        }
    }

    fn clear_cell(&mut self, (sx, sy): (u16, u16)) {
        let cell_x = sx * font::FONT_SCALE * font::FONT_WIDTH;
        let cell_y = sy * font::FONT_SCALE * font::FONT_HEIGHT;

        for y in 0..font::FONT_HEIGHT + 1 {
            for x in 0..font::FONT_WIDTH {
                let x = x * font::FONT_SCALE;
                let y = y * font::FONT_SCALE;

                self.draw_scaled_pixel(self.text_style.background, font::FONT_SCALE, (cell_x + x, cell_y + y));
            }
        }
    }

    fn draw_glyph_in_cell(&mut self, (sx, sy): (u16, u16), char: char) {
        let glyph = &font::FONTS[char as usize];

        // (0, 0) is at the bottom left of the glyph,
        // while `glyph.raster` starts at the top left,
        // so the glyph has to be offset accordingly
        let cell_offset_y = font::FONT_SCALE * (font::FONT_HEIGHT.max(glyph.height) - glyph.height);

        let cell_x = sx * font::FONT_SCALE * font::FONT_WIDTH;
        let cell_y = sy * font::FONT_SCALE * font::FONT_HEIGHT;

        for y in 0..glyph.height {
            for x in 0..glyph.width {
                let index = y * glyph.width + x;
                let fg_pixel = glyph.display[index as usize];

                let x = x * font::FONT_SCALE;
                let y = y * font::FONT_SCALE;

                let draw_x = (cell_x + x) as isize + (font::FONT_SCALE as isize * glyph.off_x);
                let draw_y = (cell_y + y + cell_offset_y) as isize - (font::FONT_SCALE as isize * glyph.off_y);

                let color = if fg_pixel { self.text_style.foreground } else { self.text_style.background };

                self.draw_scaled_pixel(color, font::FONT_SCALE, (draw_x as u16, draw_y as u16));
            }
        }
    }

    fn draw_pixel(&mut self, color: Color, (x, y): (u16, u16)) {
        let pixel_index = (y as usize * self.info.width * self.bytes_per_pixel) + (x as usize * self.bytes_per_pixel);

        // Prevent Going out of buffer bounds.
        // TODO: Improve on. We shouldn't cut off pixels.
        if self.buffer.len() <= pixel_index + self.bytes_per_pixel {
            return;
        }

        self.buffer[pixel_index    ] = color.b;
        self.buffer[pixel_index + 1] = color.g;
        self.buffer[pixel_index + 2] = color.r;

        if self.bytes_per_pixel > 3 {
            for i in 3..self.bytes_per_pixel {
                self.buffer[pixel_index + i] = 0xFF;
            }
        }
    }

    fn draw_scaled_pixel(&mut self, color: Color, scale: u16, (x, y): (u16, u16)) {
        for sy in 0..scale {
            for sx in 0..scale {
                self.draw_pixel(color, (x + sx, y + sy));
            }
        }
    }

    fn screen_pixel_width(&self) -> u16 {
        self.info.width as u16 / (font::FONT_WIDTH * font::FONT_SCALE)
    }

    fn screen_pixel_height(&self) -> u16 {
        self.info.height as u16 / (font::FONT_HEIGHT * font::FONT_SCALE)
    }
}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.process_string(s);

        Ok(())
    }
}

fn _print(args: core::fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        crate::serial::_print(args);

        if let Some(writer) = FB_WRITER.get() {
            writer.lock()
                .write_fmt(args)
                .expect("Failed to write to framebuffer");
        } else if cfg!(debug_assertions) {
            crate::serial_println!("WARN: Framebuffer has not been initialized");
        }
    })
}