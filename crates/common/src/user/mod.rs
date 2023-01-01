use alloc::vec::Vec;

use crate::Position;



pub struct ConsoleCursor {
    pos: Position<u16>,

    displayed: bool,

    input: Vec<char>,
}