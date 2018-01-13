extern crate libc;

mod signal;

use self::libc::c_int;
pub use self::signal::Signal;

pub type SignalHandlerFn = extern fn(i32);

extern {
    fn signal(signal: c_int, sighandler_t: SignalHandlerFn) -> SignalHandlerFn;
}

pub fn register(signal_code: Signal, callback: SignalHandlerFn) {
    unsafe {
        signal(signal_code as c_int, callback);
    }
}
