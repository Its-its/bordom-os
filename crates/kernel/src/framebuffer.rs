use core::fmt::Write;

use alloc::{collections::VecDeque, vec::Vec};
use bootloader_api::info::FrameBufferInfo;
use common::Position;
use spin::{Mutex, Once};

use crate::{font, color::{ColorName, Color}};

pub static FB_WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

pub(super) fn init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    FB_WRITER.call_once(|| FrameBufferWriter::new(buffer, info).into());
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

pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
    info: FrameBufferInfo,
    cell: Position<u16>,
    text_style: TextStyle,
    bytes_per_pixel: usize,

    cached_lines: VecDeque<Vec<char>>,
}

impl FrameBufferWriter {
    fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut fb = FrameBufferWriter {
            buffer,
            info,
            cell: Position::default(),
            text_style: TextStyle::default(),
            bytes_per_pixel: info.bytes_per_pixel,
            cached_lines: VecDeque::with_capacity(500),
        };

        fb.clear();

        fb
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

    fn cursor_next_line(&mut self) {
        if self.cached_lines.len() == 500 {
            self.cached_lines.pop_front();
        }

        if self.cell.y() + 1 >= self.info.height as u16 / (font::FONT_SCALE * font::FONT_HEIGHT) {
            self.move_buffer_up();
            self.cell.set_x(0);
        } else {
            self.cell.set_x(0);
            self.cell.inc_y(1);
        }

        self.cached_lines.push_back(Vec::new());
    }

    // TODO: Check string for new line? Move up first then render. Would fix self.buffer overflow
    fn write_string(&mut self, s: &str) {
        let mut chars = s.chars();

        while let Some(char) = chars.next() {
            if char == '\n' {
                self.cursor_next_line();
                continue;
            }

            let last_cached_row = self.cached_lines.back_mut().unwrap();

            if char == '\x08' {
                // TODO: Clear Cell.

                if self.cell.x() != 0 {
                    self.cell.dec_x(1);
                    last_cached_row.pop();
                } else {
                    // TODO
                }

                continue;
            }

            last_cached_row.push(char);

            if char == '\x1B' {
                let _square_bracket = chars.next();
                let color_code = [chars.next(), chars.next()];

                if let [Some(mode), Some(color)] = color_code {
                    if mode != '3' && mode != '4' { continue }

                    let color_index = color as u8 - b'0';
                    let color = ColorName::from_u8(color_index).color();

                    match mode {
                        '3' => self.text_style.foreground = color,
                        '4' => self.text_style.background = color,

                        _ => ()
                    }
                }

                let _m = chars.next();

                continue;
            }

            let glyph = &font::FONT[char as usize];

            // (0, 0) is at the bottom left of the glyph,
            // while `glyph.raster` starts at the top left,
            // so the glyph has to be offset accordingly
            let cell_offset_y = font::FONT_SCALE * (font::FONT_HEIGHT.max(glyph.height) - glyph.height);

            for y in 0..glyph.height {
                for x in 0..glyph.width {
                    let index = y * glyph.width + x;
                    let fg_pixel = glyph.display[index as usize];

                    let x = x * font::FONT_SCALE;
                    let y = y * font::FONT_SCALE;

                    let (sx, sy) = self.cell.inner();

                    let cell_x = sx * font::FONT_SCALE * font::FONT_WIDTH;
                    let cell_y = sy * font::FONT_SCALE * font::FONT_HEIGHT;

                    let draw_x = (cell_x + x) as isize + (font::FONT_SCALE as isize * glyph.off_x);
                    let draw_y = (cell_y + y + cell_offset_y) as isize - (font::FONT_SCALE as isize * glyph.off_y);

                    let color = if fg_pixel { self.text_style.foreground } else { self.text_style.background };

                    self.draw_scaled_pixel(color, font::FONT_SCALE, (draw_x as u16, draw_y as u16));
                }
            }

            if self.cell.x() + 1 >= self.info.width as u16 / (font::FONT_WIDTH * font::FONT_SCALE) {
                self.cursor_next_line();
            } else {
                self.cell.inc_x(1);
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
}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);

        Ok(())
    }
}

pub fn _print(args: core::fmt::Arguments) {
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::framebuffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ()            => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}