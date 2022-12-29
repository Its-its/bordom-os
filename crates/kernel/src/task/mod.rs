use core::{future::Future, pin::Pin, task::{Context, Poll}, sync::atomic::{AtomicU64, Ordering}};
use alloc::boxed::Box;
use crossbeam_queue::ArrayQueue;
use lazy_static::lazy_static;

pub mod keyboard;
mod executor;

pub use executor::Executor;

// TODO: Thread
// lazy_static! {
//     // Tasks which have yet to have been added to the executor.
//     static ref PENDING_TASKS: ArrayQueue<Task> = ArrayQueue::new(512);
//     // TODO: Why can't I go up to 600
// }


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

// Required since tasks might be executed on different threads.
// TODO: Send causes it to panic
// unsafe impl Send for Task {}
// unsafe impl Sync for Task {}

impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}

pub fn spawn_task(future: impl Future<Output = ()> + 'static) -> bool {
    // PENDING_TASKS.push(Task::new(future)).is_ok()

false}