use core::fmt::Write;

use alloc::{vec::Vec, string::String, vec, format};
use bootloader_api::info::FrameBufferInfo;
use common::Dimensions;
use gbl::io::LogType;
use spin::{Mutex, Once};

use crate::{color::ColorName, allocator::get_allocated};

use super::{ConsoleContainer, CacheType};

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


pub struct FrameBufferWriter {
    buffer: &'static mut [u8],
    #[allow(dead_code)]
    info: FrameBufferInfo,

    console: ConsoleContainer,
}

impl FrameBufferWriter {
    fn new(buffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut fb = FrameBufferWriter {
            console: ConsoleContainer::new(Dimensions::from((info.width, info.height)), info.bytes_per_pixel),
            buffer,
            info,
        };

        fb.console.clear(fb.buffer);
        // TODO: console.set_input_type FullCanvas, Row,
        fb.console.move_cursor(0, 999);

        fb
    }

    pub fn tick(&mut self) {
        self.console.tick(self.buffer);
    }

    // TODO: Check string for new line? Move up first then render. Would fix self.buffer overflow
    fn process_string(&mut self, s: &str) {
        let mut chars = s.chars();

        while let Some(char) = chars.next() {
            // TODO: Where to store these string processors?

            if char == '\n' {
                match self.console.log_type {
                    LogType::Output => {
                        self.console.process_buffer_check(self.buffer);

                        crate::serial_println!("Allocated: {}/1024", get_allocated() / 1024);
                    }

                    LogType::UserInput => {
                        self.console.clear_cursor_line(self.buffer);

                        self.console.log_type = LogType::Output;

                        let input = self.console.take_input();
                        self.write_fmt(format_args!("{}\n", input.into_iter().collect::<String>())).unwrap();

                        self.console.log_type = LogType::UserInput;
                    }
                }

                continue;
            }

            // Backspace
            if char == '\x08' {
                self.console.handle_backspace(self.buffer);

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

                                self.console.cached_lines.back_mut().unwrap().push(CacheType::Ansi(vec!['\x1B', '[', mode, color, 'm']));

                                if mode != '3' && mode != '4' { continue }

                                let color_index = color as u8 - b'0';
                                let color = ColorName::from_u8(color_index).color();

                                match mode {
                                    '3' => self.console.text_style.foreground = color,
                                    '4' => self.console.text_style.background = color,

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

                                self.console.move_cursor(amount, 0);
                            }

                            // Cursor Back
                            Some('D') if !items.is_empty() => {
                                let amount = items.into_iter().fold(
                                    0,
                                    |a, c| a * 10 + c.to_digit(10).unwrap()
                                ) as i32;

                                self.console.move_cursor(-amount, 0);
                            }

                            _ => ()
                        }
                    }

                    _ => ()
                }

                continue;
            }

            match self.console.log_type {
                LogType::Output => {
                    self.console.draw_and_store_glyph_at_end_of_output(char, self.buffer);
                }

                LogType::UserInput => {
                    self.console.clear_current_cell(self.buffer);
                    self.console.draw_glyph_in_current_cell(char, self.buffer);

                    self.console.insert_input(char);
                }
            }
        }
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
            writer.console.log_type = type_of;

            writer.write_fmt(args).expect("Failed to write to framebuffer");
        } else if cfg!(debug_assertions) {
            crate::serial_println!("WARN: Framebuffer has not been initialized");
        }
    })
}