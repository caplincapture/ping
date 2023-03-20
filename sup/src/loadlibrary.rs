// in src/loadlibrary.rs
use std::ffi::c_void;

pub struct Library {
    handle: *const c_void;
}