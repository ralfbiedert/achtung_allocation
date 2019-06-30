#![feature(allocator_api)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::thread::ThreadId;
use std::sync::{RwLock, Arc};

use lazy_static::lazy_static;
use std::sync::atomic::{AtomicBool, Ordering};


struct Manager {
    events: [Event; 1000000],
    next: usize,
    track: bool,
}


#[derive(Default, Copy, Clone)]
struct Statistics {
    allocated: usize,
    events: usize,
}

static mut INITIALIZED: AtomicBool = AtomicBool::new(false);


lazy_static! {
    static ref MANAGER: Arc<RwLock<Manager>> = {
        let rval = Arc::new(RwLock::new(Manager::new()));
    
        println!("DDDD");
    
        // Sets our initialization status
        unsafe { INITIALIZED.fetch_and(true, Ordering::SeqCst) } ;
        
        rval
    };
}


impl Manager {
    const fn new() -> Self {
        Self {
            events: [Event::new(); 1000000],
            next: 0,
            track: false,
        }
    }
    
    pub fn enabled(&mut self, val: bool) {
        self.track = val;
    }
    
    pub fn is_enabled(&mut self) -> bool {
        self.track
    }
    
    
    pub fn record(&mut self, event: Event) {
        self.events[self.next] = event;
        self.next += 1;
    }
    
    pub fn statistics(&self) -> Statistics {
        let statistics = Statistics::default();
        
        for event in &self.events[0..self.next] {
        
        }
        
        statistics
    }
}


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


fn is_initialized() -> bool {
    unsafe { INITIALIZED.load(Ordering::SeqCst) }
}

unsafe impl GlobalAlloc for UserAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        
        if is_initialized() {
            let event = Event {
                thread_id: None,
                size: layout.size(),
                align: layout.align(),
                time: 0,
            };
    
            let mut manager = MANAGER.write().unwrap();
            if manager.is_enabled() {
                manager.record(event);
                
            }
        }
        
        System.alloc(layout)
    }
    
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

pub fn measure<T: FnOnce() -> R, R>(label: &str, x: T) -> R {
    // We must be initialized here, otherwise something went wrong.
//    assert!(is_initialized());
    
    {
        let mut manager = MANAGER.write().unwrap();
        manager.enabled(true);
    }
    
    
    let x = x();
    
    let statistics: Statistics = {
        let mut manager = MANAGER.write().unwrap();
        manager.enabled(false);
        manager.statistics()
    };
    
    println!("{:?} -- Events: {:?}, Bytes: {:?}", label, statistics.events, statistics.allocated);
    
    
    x
    
//    unsafe {
//        MANAGER.track.store(true, Ordering::SeqCst);
//
//        let r = x();
//
//        MANAGER.track.store(false, Ordering::SeqCst);
//
//        let N = MANAGER.next.load(Ordering::SeqCst);
//        let mut total_bytes = 0;
//
//        for i in 0..N {
//            let event = MANAGER.events[i];
//
//            total_bytes += event.size;
//        }
//
//        println!("{:?} -- Events: {:?}, Bytes: {:?}", label, N, total_bytes);
//
//        MANAGER.next.store(0, Ordering::SeqCst);
//
//        r
}
