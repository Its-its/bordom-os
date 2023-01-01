use core::mem;

use alloc::vec::Vec;

use crate::{Position, Dimensions};


#[derive(Default)]
pub struct ConsoleCursor {
    pos: Position<u16>,

    displayed: bool,

    input: Vec<char>,
}

impl ConsoleCursor {
    /// Toggles display. Returns the new value.
    pub fn toggle_displayed(&mut self) -> bool {
        self.displayed = !self.displayed;
        self.displayed
    }

    pub fn pos(&self) -> Position<u16> {
        self.pos
    }

    pub fn set_y(&mut self, value: u16) {
        self.pos.set_y(value);
    }

    pub fn backspace(&mut self) {
        // TODO: Check where we are in the input.
        self.input.pop();

        if self.pos.x() != 0 {
            self.pos.dec_x(1);
        }
    }

    pub fn take_input(&mut self) -> Vec<char> {
        self.pos.set_x(0);

        mem::take(&mut self.input)
    }

    pub fn insert_input(&mut self, value: char, display_dimensions: Dimensions<u16>) {
        self.input.push(value);

        if self.pos.x() + 1 >= display_dimensions.width() {
            // TODO: handle
        } else {
            self.pos.inc_x(1);
        }
    }

    pub fn move_me(&mut self, horiz: i32, vert: i32) {
        let horiz_abs = horiz.unsigned_abs() as u16;
        let vert_abs = vert.unsigned_abs() as u16;

        match horiz {
            0 => (),

            1..=i32::MAX => self.pos.inc_x(horiz_abs),

            i32::MIN ..= -1 if self.pos.x() < horiz_abs => self.pos.dec_x(horiz_abs),

            _ => ()
        }

        match vert {
            0 => (),

            1..=i32::MAX => self.pos.inc_y(vert_abs),

            i32::MIN ..= -1 if self.pos.y() < vert_abs => self.pos.dec_y(vert_abs),

            _ => ()
        }
    }
}