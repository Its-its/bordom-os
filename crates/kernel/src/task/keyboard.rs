use core::{pin::Pin, task::{Context, Poll}};

use crossbeam_queue::ArrayQueue;
use futures_util::{Stream, task::AtomicWaker, StreamExt};
use keyboard::{ExtendedKeyCode, KeyCode, KeyEvent};
use lazy_static::lazy_static;

use crate::{println, input};

static WAKER: AtomicWaker = AtomicWaker::new();

lazy_static! {
    static ref SCANCODE_QUEUE: ArrayQueue<u8> = ArrayQueue::new(100);
}


pub async fn handle_key_presses() {
    let mut scancodes = ScancodeStream::new();

    // Will never return None
    while let Some(scancode) = scancodes.next().await {
        if let Some(KeyEvent::Down(key)) = keyboard::handle_next_scan_code(scancode) {
            match key.code {
                KeyCode::Unknown(v) => println!("[{v}]"),
                KeyCode::Extended(ExtendedKeyCode::Unknown(v)) => println!("[e{v}]"),

                KeyCode::Extended(ExtendedKeyCode::CursorUp) => input!("\x1B[1A"),
                KeyCode::Extended(ExtendedKeyCode::CursorDown) => input!("\x1B[1B"),
                KeyCode::Extended(ExtendedKeyCode::CursorRight) => input!("\x1B[1C"),
                KeyCode::Extended(ExtendedKeyCode::CursorLeft) => input!("\x1B[1D"),

                _ => input!("{}", key.char),
            }
        }
    }
}


pub(crate) fn add_scancode(scancode: u8) {
    if SCANCODE_QUEUE.push(scancode).is_err() {
        println!("WARNING: scancode queue full; dropping keyboard input");
    } else {
        WAKER.wake();
    }
}


pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Initiates the field.
        let _ = SCANCODE_QUEUE.len();
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        if let Some(code) = SCANCODE_QUEUE.pop() {
            return Poll::Ready(Some(code));
        }

        WAKER.register(cx.waker());

        match SCANCODE_QUEUE.pop() {
            Some(scancode) => {
                WAKER.take();

                Poll::Ready(Some(scancode))
            }

            None => Poll::Pending,
        }
    }
}