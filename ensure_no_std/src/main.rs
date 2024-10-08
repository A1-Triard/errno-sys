#![feature(start)]

#![deny(warnings)]

#![no_std]

#[cfg(windows)]
#[link(name="msvcrt")]
extern { }

mod no_std {
    use core::panic::PanicInfo;
    use exit_no_std::exit;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        exit(99)
    }

    #[no_mangle]
    extern "C" fn rust_eh_personality() { }
}

use errno_sys::*;
use libc::EINVAL;

#[start]
pub fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let c = unsafe { &mut *errno_location() };
    *c = EINVAL;
    assert_eq!(*c, EINVAL);
    0
}
