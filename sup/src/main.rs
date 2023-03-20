mod loadlibrary

use pretty_hex::*;
use std::{
    ffi::c_void,
    fmt,
    mem::{size_of, transmute},
    slice,
};

use std::{
    ffi::c_void,
    ptr::NonNull,
};

type HModule = NonNull<c_void>;

pub struct Library {
    handle: HModule,
}



type HModule = *const c_void;
type FarProc = *const c_void;

extern "stdcall" {
    fn LoadLibraryA(name: *const u8) -> HModule;
    fn GetProcAddress(module: HModule, name: *const u8) -> FarProc;
}

struct IPAddr([u8; 4]);

impl fmt::Debug for IPAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let [a, b, c, d] = self.0;
        write!(f, "{}.{}.{}.{}", a, b, c, d)
    }
}

#[repr(C)]
#[derive(Debug)]
struct IpOptionInformation {
    ttl: u8,
    tos: u8,
    flags: u8,
    options_size: u8,
    options_data: u32,
}

type Handle = *const c_void;

#[repr(C)]
#[derive(Debug)]
struct IcmpEchoReply {
    address: IPAddr,
    status: u32,
    rtt: u32,
    data_size: u16,
    reserved: u16,
    data: *const u8,
    options: IpOptionInformation,
}

type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: IPAddr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;
type IcmpCreateFile = extern "stdcall" fn() -> Handle;

fn main() {
    #[allow(non_snake_case)]
    unsafe {
        let h = LoadLibraryA("IPHLPAPI.dll\0".as_ptr());
        let IcmpCreateFile: IcmpCreateFile =
            transmute(GetProcAddress(h, "IcmpCreateFile\0".as_ptr()));
        let IcmpSendEcho: IcmpSendEcho = transmute(GetProcAddress(h, "IcmpSendEcho\0".as_ptr()));

        let handle = IcmpCreateFile();

        let data = "O Romeo, Romeo. Reachable art thou Romeo?";
        let ip_opts = IpOptionInformation {
            ttl: 128,
            tos: 0,
            flags: 0,
            options_data: 0,
            options_size: 0,
        };

        let reply_size = size_of::<IcmpEchoReply>();
        let reply_buf_size = reply_size + 8 + data.len();
        let mut reply_buf = vec![0u8; reply_buf_size];

        let ret = IcmpSendEcho(
            handle,
            IPAddr([8, 8, 8, 8]),
            data.as_ptr(),
            data.len() as u16,
            Some(&ip_opts),
            reply_buf.as_mut_ptr(),
            reply_buf_size as u32,
            4000,
        );
        if ret == 0 {
            panic!("IcmpSendEcho failed! ret = {}", ret);
        }

        let reply: &IcmpEchoReply = transmute(&reply_buf[0]);
        println!("{:#?}", *reply);

        let reply_data: *const u8 = transmute(&reply_buf[reply_size + 8]);
        let reply_data = slice::from_raw_parts(reply_data, reply.data_size as usize);
        println!("{:?}", reply_data.hex_dump());
    }
}