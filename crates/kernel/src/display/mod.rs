use alloc::{collections::VecDeque, vec::Vec};
use bootloader_api::{info::FrameBuffer};
use common::{Dimensions, user::ConsoleCursor};
use gbl::io::{LogType, ColorName, Color};

use crate::font::{FONT_HEIGHT, FONT_WIDTH, FONT_SCALE, FONTS};

pub mod framebuffer;
pub mod vga;


const FONT_HEIGHT_SCALED: usize = FONT_HEIGHT as usize * FONT_SCALE as usize;
const FONT_WIDTH_SCALED: usize = FONT_WIDTH as usize * FONT_SCALE as usize;


pub fn init(fb: Option<&'static mut FrameBuffer>) {
    let green = ColorName::Green.ansi();
    let clear = ColorName::DefaultForeground.ansi();

    // Framebuffer Output
    if let Some(fb) = fb {
        let fb_info = fb.info();

        framebuffer::init(fb.buffer_mut(), fb_info);

        println!("INIT: Framebuffer... [{green}OK{clear}]");
        println!("  Dimensions: w {}, h {}", fb_info.width, fb_info.height);
        println!("  Pixel Format: {:?}", fb_info.pixel_format);
        println!("  Pixel Size: {}", fb_info.bytes_per_pixel);
        println!("  Line Stride: {}", fb_info.stride);
    }
}


// TODO: Should this be in the kernel?
pub struct ConsoleContainer {
    dimensions: Dimensions<usize>,

    pub text_style: TextStyle,
    bytes_per_pixel: usize,

    // TODO: Private
    pub(in self) cached_lines: VecDeque<Vec<CacheType>>,

    // TODO: Expand upon. For example allow whole container input.
    input_height: u16,
    pub log_type: LogType,

    cursor: ConsoleCursor,
}

impl ConsoleContainer {
    pub fn new(dimensions: Dimensions<usize>, bytes_per_pixel: usize) -> Self {
        Self {
            dimensions,

            text_style: TextStyle::default(),
            bytes_per_pixel,

            cached_lines: VecDeque::with_capacity(500),

            input_height: 1,
            log_type: LogType::Output,

            cursor: ConsoleCursor::default(),
        }
    }

    // Cursor Specific

    pub fn move_cursor(&mut self, horiz: i32, vert: i32) {
        self.cursor.move_me(horiz, vert);

        if self.cursor.pos().x() >= self.pixel_width() {
            self.cursor.set_x(self.pixel_width() - 1);
        }

        if self.cursor.pos().y() >= self.pixel_height() {
            self.cursor.set_y(self.pixel_height() - 1);
        }
    }

    pub fn insert_input(&mut self, value: char) {
        self.cursor.insert_input(value, self.screen_dimensions());
    }

    pub fn take_input(&mut self) -> Vec<char> {
        self.cursor.take_input()
    }

    pub fn clear_cursor_line(&mut self, buffer: &mut [u8]) {
        let mut pos = self.cursor.pos();

        for i in 0..pos.x() + 1 {
            pos.set_x(i);
            self.clear_cell(pos.inner(), buffer);
        }
    }

    pub fn handle_backspace(&mut self, buffer: &mut [u8]) {
        let last_pos = self.cursor.pos().inner();

        match self.log_type {
            LogType::Output => {
                self.cached_lines.back_mut().unwrap().pop();
            }

            LogType::UserInput => {
                self.cursor.backspace();
            }
        }

        self.clear_cell(last_pos, buffer);
    }

    pub fn clear_current_cell(&mut self, buffer: &mut [u8]) {
        self.clear_cell(self.cursor.pos().inner(), buffer);
    }

    pub fn draw_glyph_in_current_cell(&mut self, value: char, buffer: &mut [u8]) {
        self.draw_glyph_in_cell(self.cursor.pos().inner(), value, buffer);
    }

    //

    pub fn tick(&mut self, buffer: &mut [u8]) {
        self.cursor.update();

        if self.cursor.is_displayed() {
            let curr = self.text_style.foreground;
            self.text_style.foreground = ColorName::Green.color();
            self.draw_glyph_in_cell(self.cursor.pos().inner(), '_', buffer);
            self.text_style.foreground = curr;
        } else {
            self.clear_cell(self.cursor.pos().inner(), buffer);
        }
    }

    pub fn draw_and_store_glyph_at_end_of_output(&mut self, value: char, buffer: &mut [u8]) {
        let last_cached_row = self.cached_lines.back_mut().unwrap();

        last_cached_row.push(CacheType::Char(value));

        let last_line_size = last_cached_row.iter().filter(|c| c.is_char()).count().saturating_sub(1);
        let buff_len = (self.cached_lines.len() - 1).min(self.pixel_height() as usize - 2);

        self.draw_glyph_in_cell((last_line_size as u16, buff_len as u16), value, buffer);
    }

    pub fn clear(&mut self, buffer: &mut [u8]) {
        self.cached_lines.clear();
        self.cached_lines.push_back(Vec::new());

        let bg = ColorName::DefaultBackground.color().to_framebuffer_pixel();

        // This is faster than using for i in 0..num_subpixels,
        // since for loops use the `Iterator` trait under the hood,
        // which uses Clone, rather than Copy.
        //
        // This could likely be optimized even further with some
        // cursed pointer stuff, but it is fast enough as is.
        let mut i = 0;
        let num_subpixels = buffer.len();
        while i < num_subpixels {
            buffer[i] = bg[i % 3];

            i += 1;
        }
    }

    fn move_buffer_up(&mut self, buffer: &mut [u8]) {
        let bg_color = ColorName::DefaultBackground.color().to_framebuffer_pixel();

        let buffer_line = self.dim_pixel_width();

        let font_buffer_line_size = FONT_HEIGHT_SCALED * buffer_line;

        let pix_height = self.pixel_height() as usize;

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
                        buffer[index] = bg_color[index % 3];
                    } else {
                        buffer[
                            px
                                // X
                                + (x % prev_line_width)
                                // Y
                                + buffer_line * (x / prev_line_width)
                        ] =
                        buffer[
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
                    buffer[
                        px
                            // X
                            + (x % curr_line_width)
                            // Y
                            + buffer_line * (x / curr_line_width)
                    ] =
                    buffer[
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
        let mut i = buffer.len() - font_buffer_line_size * 2;
        let num_subpixels = buffer.len();
        while i < num_subpixels {
            buffer[i] = bg_color[i % 3];

            i += 1;
        }
    }

    fn process_buffer_check(&mut self, buffer: &mut [u8]) {
        if self.cached_lines.len() == self.cached_lines.capacity() {
            self.cached_lines.pop_front();
        }

        if self.cached_lines.len() as u16 + self.input_height >= self.pixel_height() {
            self.move_buffer_up(buffer);
        //     self.cursor_pos.set_x(0);
        // } else {
        //     self.cursor_pos.set_x(0);
        //     self.cursor_pos.inc_y(1);
        }

        self.cached_lines.push_back(Vec::new());
    }

    pub fn clear_cell(&mut self, (sx, sy): (u16, u16), buffer: &mut [u8]) {
        let cell_x = sx * FONT_SCALE * FONT_WIDTH;
        let cell_y = sy * FONT_SCALE * FONT_HEIGHT;

        for y in 0..FONT_HEIGHT + 1 {
            for x in 0..FONT_WIDTH {
                let x = x * FONT_SCALE;
                let y = y * FONT_SCALE;

                self.draw_scaled_pixel(self.text_style.background, FONT_SCALE, (cell_x + x, cell_y + y), buffer);
            }
        }
    }

    pub fn draw_glyph_in_cell(&mut self, (sx, sy): (u16, u16), char: char, buffer: &mut [u8]) {
        let glyph = &FONTS[char as usize];

        // (0, 0) is at the bottom left of the glyph,
        // while `glyph.raster` starts at the top left,
        // so the glyph has to be offset accordingly
        let cell_offset_y = FONT_SCALE * (FONT_HEIGHT.max(glyph.height) - glyph.height);

        let cell_x = sx * FONT_SCALE * FONT_WIDTH;
        let cell_y = sy * FONT_SCALE * FONT_HEIGHT;

        for y in 0..glyph.height {
            for x in 0..glyph.width {
                let index = y * glyph.width + x;
                let fg_pixel = glyph.display[index as usize];

                let x = x * FONT_SCALE;
                let y = y * FONT_SCALE;

                let draw_x = (cell_x + x) as isize + (FONT_SCALE as isize * glyph.off_x);
                let draw_y = (cell_y + y + cell_offset_y) as isize - (FONT_SCALE as isize * glyph.off_y);

                let color = if fg_pixel { self.text_style.foreground } else { self.text_style.background };

                self.draw_scaled_pixel(color, FONT_SCALE, (draw_x as u16, draw_y as u16), buffer);
            }
        }
    }

    pub fn draw_pixel(&mut self, color: Color, (x, y): (u16, u16), buffer: &mut [u8]) {
        let pixel_index = (y as usize * self.dim_pixel_width()) + (x as usize * self.bytes_per_pixel);

        // Prevent Going out of buffer bounds.
        // TODO: Improve on. We shouldn't cut off pixels.
        if buffer.len() <= pixel_index + self.bytes_per_pixel {
            return;
        }

        buffer[pixel_index    ] = color.b;
        buffer[pixel_index + 1] = color.g;
        buffer[pixel_index + 2] = color.r;

        if self.bytes_per_pixel > 3 {
            for i in 3..self.bytes_per_pixel {
                buffer[pixel_index + i] = 0xFF;
            }
        }
    }

    fn draw_scaled_pixel(&mut self, color: Color, scale: u16, (x, y): (u16, u16), buffer: &mut [u8]) {
        for sy in 0..scale {
            for sx in 0..scale {
                self.draw_pixel(color, (x + sx, y + sy), buffer);
            }
        }
    }


    fn dim_pixel_width(&self) -> usize {
        self.dimensions.width() * self.bytes_per_pixel
    }

    fn pixel_width(&self) -> u16 {
        (self.dimensions.width() / (FONT_WIDTH * FONT_SCALE) as usize) as u16
    }

    fn pixel_height(&self) -> u16 {
        (self.dimensions.height() / (FONT_HEIGHT * FONT_SCALE) as usize) as u16
    }

    fn screen_dimensions(&self) -> Dimensions<u16> {
        Dimensions::from((
            self.pixel_width(),
            self.pixel_height(),
        ))
    }
}


pub struct TextStyle {
    foreground: Color,
    background: Color,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            foreground: ColorName::DefaultForeground.color(),
            background: ColorName::DefaultBackground.color(),
        }
    }
}

#[derive(Debug)]
pub(in self) enum CacheType {
    Char(char),
    Ansi(Vec<char>),
}

impl CacheType {
    pub fn is_char(&self) -> bool {
        matches!(self, Self::Char(_))
    }
}
