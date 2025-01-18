use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;

pub struct ScancodeStream {
    _private: (),
}


static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

//called by the keyboard interrupt handler
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        }
    } else {
        println!("WARNING: scancode queue uninitialized");
    }
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}


