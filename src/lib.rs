#![feature(allocator_api)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread::ThreadId;


struct Manager {
    events: [Event; 1000000],
    next: AtomicUsize,
    track: AtomicBool,
}

impl Manager {
    const fn new() -> Self {
        Self {
            events: [Event::new(); 1000000],
            next: AtomicUsize::new(0),
            track: AtomicBool::new(false),
        }
    }
}


static mut MANAGER: Manager = Manager::new();

pub struct UserAlloc;

#[global_allocator]
static mut ALLOCATOR: UserAlloc = UserAlloc;

#[derive(Debug, Copy, Clone)]
struct Event {
    thread_id: Option<ThreadId>,
    time: usize,
    size: usize,
    align: usize,
}

impl Event {
    const fn new() -> Self {
        Self {
            thread_id: None,
            time: 0,
            size: 0,
            align: 0,
        }
    }
}

unsafe impl GlobalAlloc for UserAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if MANAGER.track.load(Ordering::SeqCst) {
            // let x = thread::current(); // crash ?!
            MANAGER.events[MANAGER.next.load(Ordering::SeqCst)] = Event {
                thread_id: None,
                size: layout.size(),
                align: layout.align(),
                time: 0,
            };
    
            MANAGER.next.fetch_add(1, Ordering::SeqCst);
        }
        
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

fn measure<T: FnOnce() -> R, R>(label: &str, x: T) -> R {
    unsafe {
        MANAGER.track.store(true, Ordering::SeqCst);
        
        let r = x();
    
        MANAGER.track.store(false, Ordering::SeqCst);
        
        let N = MANAGER.next.load(Ordering::SeqCst);
        let mut total_bytes = 0;
        
        for i in 0..N {
            let event = MANAGER.events[i];
            
            total_bytes += event.size;
        }
        
        println!("{:?} -- Events: {:?}, Bytes: {:?}", label, N, total_bytes);
    
        MANAGER.next.store(0, Ordering::SeqCst);
        
        r
    }
}

fn ff<T: Fn()>(f: T) {
    f()
}

pub fn main() {

}
