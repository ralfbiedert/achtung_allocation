#![feature(allocator_api)]

mod allocator;

use std::thread::ThreadId;
use std::alloc::{GlobalAlloc, Layout, System, Alloc};
use std::sync::atomic::{Ordering, AtomicBool,AtomicUsize};
use self::allocator::TracingAllocator;

/// We set ourselves as global allocator and intercept every call with our own statistics.
#[global_allocator]
static mut ALLOCATOR: TracingAllocator = TracingAllocator::new();


//fn measure<T: FnOnce() -> R, R>(label: &str, x: T) -> R {
//    unsafe {
//        TRACK.store(true, Ordering::SeqCst);
//
//        let r = x();
//
//        TRACK.store(false, Ordering::SeqCst);
//
//        let N = NEXT.load(Ordering::SeqCst);
//        let mut total_bytes = 0;
//
//        for i in 0..N {
//            let event = EVENTS[i];
//
//            total_bytes += event.size;
//        }
//
//        println!("{:?} -- Events: {:?}, Bytes: {:?}", label, N, total_bytes);
//
//        NEXT.store(0, Ordering::SeqCst);
//
//        r
//    }
//}
