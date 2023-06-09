// note that we're not declaring this one "pub"
// it's just our own!
pub(crate) mod icmp_sys;
use std::{mem::{size_of, transmute}, slice};

use icmp_sys::IcmpCreateFile;

pub fn something() {
    let handle = IcmpCreateFile();
}

use crate::ipv4;

pub struct Request {
    dest: ipv4::Addr,
    ttl: u8,
    timeout: u32,
    data: Option<Vec<u8>>,
}

use std::time::Duration;

pub struct Reply {
    pub addr: ipv4::Addr,
    pub data: Vec<u8>,
    pub rtt: Duration,
    pub ttl: u8,
}


impl Request {
    pub fn new(dest: ipv4::Addr) -> Self {
        Self {
            dest,
            ttl: 128,
            timeout: 4000,
            data: None,
        }
    }
    pub fn send(&self) -> Result<Reply, String> {
        let fns = &icmp_sys::FUNCTIONS;

        let handle = (fns.IcmpCreateFile)();

        let reply_size = size_of::<icmp_sys::IcmpEchoReply>();
        let reply_buf_size = reply_size + 8 + self.data.clone().unwrap().len();
        let mut reply_buf = vec![0u8; reply_buf_size];

        let ip_options = icmp_sys::IpOptionInformation {
            ttl: self.ttl,
            tos: 0,
            flags: 0,
            options_data: 0,
            options_size: 0,
        };

        let ret = (fns.IcmpSendEcho)(
            handle,
            self.dest,
            self.data.clone().unwrap().as_ptr(),
            self.data.clone().unwrap().len() as u16,
            Some(&ip_options),
            reply_buf.as_mut_ptr(),
            reply_buf_size as u32,
            self.timeout,
        );
        (fns.IcmpCloseHandle)(handle);
        // new:
        icmp_sys::IcmpCloseHandle(handle);

        match ret {
            0 => Err("IcmpSendEcho failed :(".to_string()),
            _ => {
                let reply: &icmp_sys::IcmpEchoReply = unsafe { transmute(&reply_buf[0]) };
    
                // this bit of code was explained in part 3
                let data: Vec<u8> = unsafe {
                    let data_ptr: *const u8 = transmute(&reply_buf[reply_size + 8]);
                    slice::from_raw_parts(data_ptr, reply.data_size as usize)
                }
                .into();
    
                Ok(Reply {
                    addr: reply.address,
                    data,
                    rtt: Duration::from_millis(reply.rtt as u64),
                    ttl: reply.options.ttl,
                })
            }
        }
    }
    pub fn ttl(mut self, ttl: u8) -> Self {
        self.ttl = ttl;
        self
    }
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.timeout = timeout;
        self
    }
    pub fn data<D>(mut self, data: D) -> Self
    where
        D: Into<Vec<u8>>,
    {
        self.data = Some(data.into());
        self
    }

}
