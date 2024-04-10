use std::os::raw::{
    c_ulong,
    c_uint,
    c_void,
    c_int,
    c_long,
};

pub type LONG = c_long;
pub type DWORD = c_ulong;
pub type FIX32 = DWORD;
pub type WTPKT = DWORD;
pub type UINT = c_uint;
pub type LPVOID = *mut c_void;
pub type BOOL = c_int;