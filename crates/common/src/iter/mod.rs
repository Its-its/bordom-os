use core::iter::Peekable;

use alloc::vec::Vec;



pub struct SaveStateIterContainer<'a, V: Copy, I: Iterator<Item = V>> {
    cache: Vec<V>,
    items: &'a mut Peekable<I>,
}

impl<'a, V: Copy, I: Iterator<Item = V>> SaveStateIterContainer<'a, V, I> {
    pub fn new(items: &'a mut Peekable<I>) -> Self {
        Self {
            cache: Vec::new(),
            items,
        }
    }

    pub fn iter<'b: 'a>(&'b mut self) -> SaveStateInnerIter<'b, V, I> {
        SaveStateInnerIter {
            current_index: 0,
            saved_index: false,
            container: self,
        }
    }
}


pub struct SaveStateInnerIter<'a, V: Copy, I: Iterator<Item = V>> {
    container: &'a mut SaveStateIterContainer<'a, V, I>,

    current_index: usize,
    saved_index: bool,
}

impl<'a, V: Copy, I: Iterator<Item = V>> SaveStateInnerIter<'a, V, I> {
    pub fn save_and_finish(mut self) {
        self.saved_index = true;
    }

    pub fn save_state(&self) -> SaveState {
        SaveState(self.current_index)
    }

    pub fn load_state(&mut self, value: SaveState) {
        self.current_index = value.0;
    }

    pub fn peek(&mut self) -> Option<&V> {
        if self.current_index < self.container.cache.len() {
            self.container.cache.get(self.current_index)
        } else{
            self.container.items.peek()
        }
    }
}

impl<'a, V: Copy, I: Iterator<Item = V>> Iterator for SaveStateInnerIter<'a, V, I> {
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;

        if self.current_index - 1 < self.container.cache.len() {
            self.container.cache.get(self.current_index - 1).copied()
        } else{
            self.container.cache.push(self.container.items.next()?);
            self.container.cache.last().copied()
        }
    }
}

impl<'a, V: Copy, I: Iterator<Item = V>> Drop for SaveStateInnerIter<'a, V, I> {
    fn drop(&mut self) {
        if self.saved_index {
            self.container.cache.drain(..self.current_index);
        }
    }
}


pub struct SaveState(usize);