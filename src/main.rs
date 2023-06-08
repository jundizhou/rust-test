use std::collections::HashMap;
use std::ffi::{c_void, CStr, CString};
use std::{mem, slice};
use std::alloc::{alloc, Layout};
use rust_kindling_test::adhesive::start;


fn main() {
    println!("Hello, world!");
    start()
}