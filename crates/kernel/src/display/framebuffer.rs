use core::fmt::Write;

use alloc::{collections::VecDeque, vec::Vec, string::String, vec, format};
use bootloader_api::info::FrameBufferInfo;
use common::{user::ConsoleCursor, Dimensions};
use gbl::io::LogType;
use spin::{Mutex, Once};

use crate::{font, color::{ColorName, Color}};

pub static FB_WRITER: Once<Mutex<FrameBufferWriter>> = Once::new();

pub(super) fn init(buffer: &'static mut [u8], info: FrameBufferInfo) {
    FB_WRITER.call_once(|| FrameBufferWriter::new(buffer, info).into());

    gbl::io::set_global_dispatcher(|type_of: LogType, args: core::fmt::Arguments| {
        if type_of == LogType::Output {
            crate::task::output::add_output(format!("{args}"));
        } else {
            _print(type_of, args);
        }
    });
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

#[derive(Debug)]
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

    input_height: u16,
    log_type: LogType,

    cursor: ConsoleCursor,
}

impl FrameBufferWriter {
    fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut fb = FrameBufferWriter {
            buffer,
            info,
            text_style: TextStyle::default(),
            bytes_per_pixel: info.bytes_per_pixel,

            cached_lines: VecDeque::with_capacity(256),

            input_height: 1,
            log_type: LogType::Output,

            cursor: ConsoleCursor::default(),
        };

        fb.clear();
        fb.cursor.set_y(fb.screen_pixel_height() - 1);

        fb
    }

    pub fn tick(&mut self) {
        if self.cursor.toggle_displayed() {
            let curr = self.text_style.foreground;
            self.text_style.foreground = ColorName::Green.color();
            self.draw_glyph_in_cell(self.cursor.pos().inner(), '_');
            self.text_style.foreground = curr;
        } else {
            self.clear_cell(self.cursor.pos().inner());
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
        const FONT_HEIGHT_SCALED: usize = (font::FONT_HEIGHT * font::FONT_SCALE) as usize;
        const FONT_WIDTH_SCALED: usize = (font::FONT_WIDTH * font::FONT_SCALE) as usize;

        let bg_color = ColorName::Background.color().to_framebuffer_pixel();

        let buffer_line = self.info.width * self.bytes_per_pixel;

        let font_buffer_line_size = FONT_HEIGHT_SCALED * buffer_line;

        //

        let pix_height = self.screen_pixel_height() as usize;

        // TODO: When the buffer moves isn't always correct.
        let line_widths = self.cached_lines.range(self.cached_lines.len().saturating_sub(pix_height - 1)..)
            .map(|v| v.iter().filter(|v| v.is_char()).count())
            .collect::<Vec<_>>();

        for i in 1..line_widths.len() {
            let prev_width = line_widths[i - 1];
            let curr_width = line_widths[i];

            let prev_line_width = prev_width * FONT_WIDTH_SCALED * self.bytes_per_pixel;
            let curr_line_width = curr_width * FONT_WIDTH_SCALED * self.bytes_per_pixel;

            let px = (i - 1) * font_buffer_line_size;
            let cx = i * font_buffer_line_size;

            // TODO: Simplify. No need for two for loops.
            if prev_width != 0 && prev_width > curr_width{
                for x in 0..self.bytes_per_pixel * prev_width * FONT_HEIGHT_SCALED * FONT_WIDTH_SCALED {
                    let prev_pos = x % prev_line_width;

                    if prev_pos > curr_line_width {
                        let index = px
                            + (x % prev_line_width)
                            + buffer_line * (x / prev_line_width);
                        self.buffer[index] = bg_color[index % 3];
                    } else {
                        self.buffer[
                            px
                                // X
                                + (x % prev_line_width)
                                // Y
                                + buffer_line * (x / prev_line_width)
                        ] =
                        self.buffer[
                            cx
                                // X
                                + (x % prev_line_width)
                                // Y
                                + buffer_line * (x / prev_line_width)
                        ];
                    }
                }
            } else {
                for x in 0..self.bytes_per_pixel * curr_width * FONT_HEIGHT_SCALED * FONT_WIDTH_SCALED {
                    self.buffer[
                        px
                            // X
                            + (x % curr_line_width)
                            // Y
                            + buffer_line * (x / curr_line_width)
                    ] =
                    self.buffer[
                        cx
                            // X
                            + (x % curr_line_width)
                            // Y
                            + buffer_line * (x / curr_line_width)
                    ];
                }
            }
        }

        // TODO: Move buffer based on cached_lines.
        // let line_offset = buffer_line * 2;
        // // TODO: Line offset may be incorrect.

        // // Move Buffer Up
        // let mut i = 0;
        // let num_subpixels = self.buffer.len() - (font_buffer_line_size + line_offset);
        // while i < num_subpixels {
        //     self.buffer[i + line_offset] = self.buffer[i + font_buffer_line_size + line_offset];

        //     i += 1;
        // }

        // Clear last line
        // TODO: Remove the previous line clear. We should only be clearing the user input one.
        let mut i = self.buffer.len() - font_buffer_line_size * 2;
        let num_subpixels = self.buffer.len();
        while i < num_subpixels {
            self.buffer[i] = bg_color[i % 3];

            i += 1;
        }
    }

    fn process_buffer_check(&mut self) {
        if self.cached_lines.len() == self.cached_lines.capacity() {
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
            if char == '\n' {
                match self.log_type {
                    LogType::Output => {
                        self.process_buffer_check();
                    }

                    LogType::UserInput => {
                        {
                            let mut pos = self.cursor.pos();

                            for i in 0..pos.x() + 1 {
                                pos.set_x(i);
                                self.clear_cell(pos.inner());
                            }
                        }

                        self.log_type = LogType::Output;

                        let input = self.cursor.take_input();
                        self.write_fmt(format_args!("{}\n", input.into_iter().collect::<String>())).unwrap();

                        self.log_type = LogType::UserInput;
                    }
                }

                continue;
            }

            let last_cached_row = self.cached_lines.back_mut().unwrap();

            // Backspace
            if char == '\x08' {
                let last_pos = self.cursor.pos().inner();

                match self.log_type {
                    LogType::Output => {
                        last_cached_row.pop();
                    }

                    LogType::UserInput => {
                        self.cursor.backspace();
                    }
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
                                ) as i32;

                                self.cursor.move_me(amount, 0);
                            }

                            // Cursor Back
                            Some('D') if !items.is_empty() => {
                                let amount = items.into_iter().fold(
                                    0,
                                    |a, c| a * 10 + c.to_digit(10).unwrap()
                                ) as i32;

                                self.cursor.move_me(-amount, 0);
                            }

                            _ => ()
                        }
                    }

                    _ => ()
                }

                continue;
            }

            match self.log_type {
                LogType::Output => {
                    last_cached_row.push(CacheType::Char(char));

                    let last_line_size = last_cached_row.iter().filter(|c| c.is_char()).count().saturating_sub(1);
                    let buff_len = (self.cached_lines.len() - 1).min(self.screen_pixel_height() as usize - 2);

                    self.draw_glyph_in_cell((last_line_size as u16, buff_len as u16), char);
                }

                LogType::UserInput => {
                    self.clear_cell(self.cursor.pos().inner());
                    self.draw_glyph_in_cell(self.cursor.pos().inner(), char);

                    self.cursor.insert_input(
                        char,
                        self.screen_dimensions()
                    );
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

    fn screen_dimensions(&self) -> Dimensions<u16> {
        Dimensions::from((
            self.screen_pixel_width(),
            self.screen_pixel_height()
        ))
    }
}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.process_string(s);

        Ok(())
    }
}

pub(crate) fn _print(type_of: LogType, args: core::fmt::Arguments) {
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        crate::serial::_print(args);

        if let Some(writer) = FB_WRITER.get() {
            let mut writer = writer.lock();
            writer.log_type = type_of;

            writer.write_fmt(args).expect("Failed to write to framebuffer");
        } else if cfg!(debug_assertions) {
            crate::serial_println!("WARN: Framebuffer has not been initialized");
        }
    })
}