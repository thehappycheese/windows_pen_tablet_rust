use std::ffi::{
    c_ulong,
    c_uint,
    c_void,
    c_int,
    c_long,
};

pub type LONG = c_long;
pub type DWORD = c_ulong;
pub type FIX32 = DWORD;
pub type UINT = c_uint;
pub type LPVOID = *mut c_void;
pub type BOOL = c_int;


#[repr(C)]
pub struct HWND(pub isize);

#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct HCTX {
    pub unused: ::std::ffi::c_int,
}