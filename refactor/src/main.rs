use crate::icmp::Request;
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

    let dest = arg.parse()?;
    let data = "O Romeo.";

    println!();
    println!("Pinging {:?} with {} bytes of data:", dest, data.len());

    use std::{thread::sleep, time::Duration};

    for _ in 0..4 {
        match Request::new(dest).ttl(128).timeout(4000).data(data).send() {
            Ok(res) => println!(
                "Reply from {:?}: bytes={} time={:?} TTL={}",
                res.addr,
                res.data.len(),
                res.rtt,
                res.ttl,
            ),
            Err(_) => println!("Something went wrong"),
        }

        sleep(Duration::from_secs(1));
    }

    Ok(())
}