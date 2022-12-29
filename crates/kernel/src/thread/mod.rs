use core::{sync::atomic::{AtomicU64, Ordering}, mem};

use alloc::boxed::Box;


pub mod local;


pub fn spawn<F>(function: F) -> ThreadRoll
where
    F: FnOnce(),
    F: Send + 'static,
{
    // https://github.com/rust-lang/rust/blob/master/library/std/src/thread/mod.rs
    // To prevent leaks we use a wrapper that drops its contents.
    #[repr(transparent)]
    struct MaybeDangling<T>(mem::MaybeUninit<T>);

    impl<T> MaybeDangling<T> {
        fn new(x: T) -> Self {
            MaybeDangling(mem::MaybeUninit::new(x))
        }

        fn into_inner(self) -> T {
            // SAFETY: we are always initiailized.
            let ret = unsafe { self.0.assume_init_read() };
            // Make sure we don't drop.
            mem::forget(self);
            ret
        }
    }

    impl<T> Drop for MaybeDangling<T> {
        fn drop(&mut self) {
            // SAFETY: we are always initiailized.
            unsafe { self.0.assume_init_drop() };
        }
    }

    let f = MaybeDangling::new(function);

    let main = move || {
        let f = f.into_inner();
    };

    ThreadRoll {
        native: Box::new(main),
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ThreadId(u64);

impl ThreadId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        ThreadId(NEXT_ID.fetch_add(1, Ordering::SeqCst))
    }
}


pub struct Thread {

}

pub struct ThreadRoll {
    native: Box<dyn FnOnce() + 'static>,
}