#![feature(allocator_api, const_generics)]

use std::thread::ThreadId;
use std::alloc::{GlobalAlloc, Layout, System, Alloc};
use std::sync::atomic::{Ordering, AtomicBool,AtomicUsize};

/// Maximal number of events that can be recorded.
///
/// A number of calls might reset this limit clear up used events.
pub const MAX_EVENTS: usize = {
    use std::convert::TryFrom;
    
    if let Some(x) = option_env!("ACHTUNG_ALLOCATION_MAX_EVENTS") {
        x.len()
    } else {
        4
    };
//        let y = x.unwrap().len();
//    usize::try_from(x);
    33
};

/// Any event produced.

/// Next slot of our events to pick from;
static mut NEXT: AtomicUsize = AtomicUsize::new(0);

static mut TRACK: AtomicBool = AtomicBool::new(false);

//static mut X : Manager<100000> = Manager::<1000>::new();


//struct Manager<const N: usize> {
//    x: [Event; N]
//}
//
//impl<const X: usize> Manager<X> {
//    const fn new() -> Self {
//        unimplemented!()
////        Manager {
////
////        }
//    }
//}

/// We set ourselves as global allocator and intercept every call with our own statistics.
//#[global_allocator]
//static mut ALLOCATOR: UserAlloc = UserAlloc;

pub struct UserAlloc;


/// An allocation Event is one call to the allocator, either an `alloc` or `free` call.
#[derive(Debug, Copy, Clone)]
enum Event {
    None,
    Free {
    
    },
    Alloc {
        thread_id: Option<ThreadId>,
        time: usize,
        size: usize,
        align: usize,
    }
}

impl Event {
    /// Produce a new, empty event.
    const fn new() -> Self {
        Event::None
    }
}
//
//unsafe impl GlobalAlloc for UserAlloc {
//    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
//        if TRACK.load(Ordering::SeqCst) {
//            // let x = thread::current(); // crash ?!
//            EVENTS[NEXT.load(Ordering::SeqCst)] = Event {
//                thread_id: None,
//                size: layout.size(),
//                align: layout.align(),
//                time: 0,
//            };
//
//            NEXT.fetch_add(1, Ordering::SeqCst);
//        }
//
//        System.alloc(layout)
//    }
//
//    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
//        unsafe { System.dealloc(ptr, layout) }
//    }
//}
//
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
