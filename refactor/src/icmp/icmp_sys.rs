use crate::ipv4;

use std::ffi::{self, c_void};

pub type Handle = *const c_void;

use crate::loadlibrary::Library;

pub type IcmpCreateFile = extern "stdcall" fn() -> Handle;

use once_cell::sync::Lazy;

pub static FUNCTIONS: Lazy<Functions> = Lazy::new(|| Functions::get());

pub fn IcmpCreateFile() -> Handle {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpCreateFile: IcmpCreateFile = unsafe { iphlp.get_proc("IcmpCreateFile").unwrap() };
    IcmpCreateFile()
}

type IcmpCloseHandle = extern "stdcall" fn(handle: Handle);

pub fn IcmpCloseHandle(handle: Handle) {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpCloseHandle: IcmpCloseHandle = unsafe { iphlp.get_proc("IcmpCloseHandle").unwrap() };
    IcmpCloseHandle(handle)
}

pub type IcmpSendEcho = extern "stdcall" fn(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32;

pub fn IcmpSendEcho(
    handle: Handle,
    dest: ipv4::Addr,
    request_data: *const u8,
    request_size: u16,
    request_options: Option<&IpOptionInformation>,
    reply_buffer: *mut u8,
    reply_size: u32,
    timeout: u32,
) -> u32 {
    let iphlp = Library::new("IPHLPAPI.dll").unwrap();
    let IcmpSendEcho: IcmpSendEcho = unsafe { iphlp.get_proc("IcmpSendEcho").unwrap() };
    IcmpSendEcho(
        handle,
        dest,
        request_data,
        request_size,
        request_options,
        reply_buffer,
        reply_size,
        timeout,
    )
}

pub struct Functions {
    pub IcmpCreateFile: extern "stdcall" fn() -> Handle,
    pub IcmpSendEcho: extern "stdcall" fn(
        handle: Handle,
        dest: ipv4::Addr,
        request_data: *const u8,
        request_size: u16,
        request_options: Option<&IpOptionInformation>,
        reply_buffer: *mut u8,
        reply_size: u32,
        timeout: u32,
    ) -> u32,
    pub IcmpCloseHandle: extern "stdcall" fn(handle: Handle),
}

impl Functions {
    fn get() -> Self {
        let iphlp = Library::new("IPHLPAPI.dll").unwrap();
        Self {
            IcmpCreateFile: unsafe { iphlp.get_proc("IcmpCreateFile").unwrap() },
            IcmpSendEcho: unsafe { iphlp.get_proc("IcmpSendEcho").unwrap() },
            IcmpCloseHandle: unsafe { iphlp.get_proc("IcmpCloseHandle").unwrap() },
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IpOptionInformation {
    pub ttl: u8,
    pub tos: u8,
    pub flags: u8,
    pub options_size: u8,
    pub options_data: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct IcmpEchoReply {
    pub address: ipv4::Addr,
    pub status: u32,
    pub rtt: u32,
    pub data_size: u16,
    pub reserved: u16,
    pub data: *const u8,
    pub options: IpOptionInformation,
}

use std::time::Duration;

pub struct Reply {
    pub addr: ipv4::Addr,
    pub data: Vec<u8>,
    pub rtt: Duration,
    pub ttl: u8,
}

macro_rules! bind {
    ($(fn $name:ident($($arg:ident: $type:ty),*) -> $ret:ty;)*) => {
        struct Functions {
            $(pub $name: extern "stdcall" fn ($($arg: $type),*) -> $ret),*
        }

        static FUNCTIONS: once_cell::sync::Lazy<Functions> =
            once_cell::sync::Lazy::new(|| {
                let lib = crate::loadlibrary::Library::new("IPHLPAPI.dll").unwrap();
                Functions {
                    $($name: unsafe { lib.get_proc(stringify!($name)).unwrap() }),*
                }
            });

        $(
            #[inline(always)]
            pub fn $name($($arg: $type),*) -> $ret {
                (FUNCTIONS.$name)($($arg),*)
            }
        )*
    };
}