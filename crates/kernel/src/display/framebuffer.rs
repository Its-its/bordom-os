use core::fmt::Write;

use alloc::{string::String, format};
use bootloader_api::info::FrameBufferInfo;
use common::Dimensions;
use gbl::io::{LogType, ansi};
use spin::{Mutex, Once};

use crate::allocator::get_allocated;

use super::ConsoleContainer;

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
        let mut chars = s.chars().peekable();

        while let Some(&char) = chars.peek() {
            // TODO: Where to store these string processors?
            if let Some(ansi) = ansi::try_parse_ansi(&mut chars) {
                if let Some(ansi) = ansi {
                    match ansi {
                        ansi::Ansi::Bell => todo!(),
                        ansi::Ansi::Backspace => todo!(),
                        ansi::Ansi::Tab => todo!(),
                        ansi::Ansi::LineFeed => todo!(),
                        ansi::Ansi::FormFeed => todo!(),
                        ansi::Ansi::CarriageReturn => todo!(),
                        ansi::Ansi::Escape(escape) => match escape {
                            ansi::AnsiEscape::SingleShiftTwo => todo!(),
                            ansi::AnsiEscape::SingleShiftThree => todo!(),
                            ansi::AnsiEscape::DeviceControlString => todo!(),
                            ansi::AnsiEscape::ControlSequenceIntroducer(csi) => match csi {
                                ansi::AnsiEscapeCSI::CursorUp(_) => todo!(),
                                ansi::AnsiEscapeCSI::CursorDown(_) => todo!(),
                                ansi::AnsiEscapeCSI::CursorForward(amount) => self.console.move_cursor(amount as i32, 0),
                                ansi::AnsiEscapeCSI::CursorBack(amount) => self.console.move_cursor(-(amount as i32), 0),
                                ansi::AnsiEscapeCSI::CursorNextLine(_) => todo!(),
                                ansi::AnsiEscapeCSI::CursorPreviousLine(_) => todo!(),
                                ansi::AnsiEscapeCSI::CursorHorizontalAbsolute(_) => todo!(),
                                ansi::AnsiEscapeCSI::CursorPosition { .. } => todo!(),
                                ansi::AnsiEscapeCSI::EraseInDisplay(_) => todo!(),
                                ansi::AnsiEscapeCSI::EraseInLine(_) => todo!(),
                                ansi::AnsiEscapeCSI::ScrollUp(_) => todo!(),
                                ansi::AnsiEscapeCSI::ScrollDown(_) => todo!(),
                                ansi::AnsiEscapeCSI::HorizontalVerticalPosition { .. } => todo!(),
                                ansi::AnsiEscapeCSI::SelectGraphicRendition(sgr) => {
                                    if let Some(color) = sgr.background_color {
                                        self.console.text_style.background = color.color();
                                    }

                                    if let Some(color) = sgr.foreground_color {
                                        self.console.text_style.foreground = color.color();
                                    }
                                }
                                ansi::AnsiEscapeCSI::SaveCurrentCursorPosition => todo!(),
                                ansi::AnsiEscapeCSI::RestoreSavedCursorPosition => todo!(),
                            },
                            ansi::AnsiEscape::StringTerminator => todo!(),
                            ansi::AnsiEscape::OperatingSystemCommand => todo!(),
                            ansi::AnsiEscape::StartOfString => todo!(),
                            ansi::AnsiEscape::PrivacyMessage => todo!(),
                            ansi::AnsiEscape::ApplicationProgramCommand => todo!(),
                        }
                    }

                    continue;
                } else {
                    // crate::serial_println!("Invalid ANSI. Skipping Output.");
                    break;
                }
            }

            let _ = chars.next();

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