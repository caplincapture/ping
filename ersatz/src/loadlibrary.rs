// in src/loadlibrary.rs
use std::ffi::c_void;

pub struct Library {
    handle: *const c_void;
}

#[macro_export]
macro_rules! bind {
    // new: `library $lib:expr;`
    (library $lib:expr; $(fn $name:ident($($arg:ident: $type:ty),*) -> $ret:ty;)*) => {
        struct Functions {
            $(pub $name: extern "stdcall" fn ($($arg: $type),*) -> $ret),*
        }

        static FUNCTIONS: once_cell::sync::Lazy<Functions> =
            once_cell::sync::Lazy::new(|| {
                let lib = crate::loadlibrary::Library::new($lib).unwrap();
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