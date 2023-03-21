use std::{mem, ops::Deref, ptr};

use crate::error::Error;

use std::marker::PhantomData;

pub struct VLS<T> {
    v: Vec<u8>,
    _phantom: PhantomData<T>,
}

impl<T> Deref for VLS<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { mem::transmute(self.v.as_ptr()) }
    }
}

impl<T> VLS<T> {
    pub fn new<F>(f: F) -> Result<Self, Error>
    where
        F: Fn(*mut T, *mut u32) -> u32,
    {
        let mut size = 0;
        match f(ptr::null_mut(), &mut size) {
            ERROR_INSUFFICIENT_BUFFER => {} // good
            ret => return Err(Error::Win32(ret)),
        };

        let mut v = vec![0u8; size as usize];
        match f(unsafe { mem::transmute(v.as_mut_ptr()) }, &mut size) {
            0 => {} // good
            ret => return Err(Error::Win32(ret)),
        };

        Ok(Self {
            v,
            _phantom: PhantomData::default(),
        })
    }
}