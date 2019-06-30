use std::thread::ThreadId;
use std::alloc::{GlobalAlloc, Layout, System};



struct Data {
    x: u32
}

/// An allocator that supports keeping track of allocations.
pub(crate) struct TracingAllocator {
    events: Option<Vec<Event>>
}



impl TracingAllocator {
    pub(crate) const fn new() -> Self {
        Self {
            events: None
        }
    }
    
    fn try_init(&mut self) {
    
    }
}


unsafe impl GlobalAlloc for TracingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
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
        
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}


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


