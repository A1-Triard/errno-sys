#![deny(warnings)]
#![doc(test(attr(deny(warnings))))]
#![doc(test(attr(allow(dead_code))))]
#![doc(test(attr(allow(unused_variables))))]

#![no_std]

use libc::c_int;

#[link(name="_rust_errno_sys", kind="static")]
extern "C" {
    #[link_name="rust_errno_sys_errno_location"]
    pub fn errno_location() -> *mut c_int;
}
