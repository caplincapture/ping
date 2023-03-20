// only one type to import!
use loadlibrary::Library;

mod loadlibrary;

use std::ffi::c_void;

pub type Handle = *const c_void;
use crate::icmp::icmp_sys::IcmpSendEcho;
use crate::icmp::icmp_sys::IcmpCreateFile;

pub mod icmp;
pub mod ipv4;

extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> *const c_void;
}


use std::{env, error::Error, process::exit};

fn main() -> Result<(), Box<dyn Error>> {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        println!("Usage: sup DEST");
        exit(1);
    });
    icmp::ping(arg.parse()?)?;

    Ok(())
}
