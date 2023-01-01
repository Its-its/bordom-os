use core::{pin::Pin, task::{Context, Poll}};

use alloc::string::String;
use crossbeam_queue::ArrayQueue;
use futures_util::{Stream, task::AtomicWaker, StreamExt};
use gbl::io::LogType;
use lazy_static::lazy_static;

static WAKER: AtomicWaker = AtomicWaker::new();

lazy_static! {
    static ref OUTPUT_QUEUE: ArrayQueue<String> = ArrayQueue::new(200);
}


pub async fn handle_output() {
    let mut stream = OutputStream::new();

    // Will never return None
    while let Some(output) = stream.next().await {
        crate::display::framebuffer::_print(LogType::Output, format_args!("{output}"));
    }
}


pub(crate) fn add_output(args: String) {
    if OUTPUT_QUEUE.force_push(args).is_some() {
        println!("WARNING: scancode queue full; dropping output");
    } else {
        WAKER.wake();
    }
}


pub struct OutputStream {
    _private: (),
}

impl OutputStream {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Initiates the field.
        let _ = OUTPUT_QUEUE.len();
        OutputStream { _private: () }
    }
}

impl Stream for OutputStream {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if let Some(code) = OUTPUT_QUEUE.pop() {
            return Poll::Ready(Some(code));
        }

        WAKER.register(cx.waker());

        match OUTPUT_QUEUE.pop() {
            Some(scancode) => {
                WAKER.take();

                Poll::Ready(Some(scancode))
            }

            None => Poll::Pending,
        }
    }
}