
//! 
//! Example Usage
//! ```
//! use wintab_wrapper::{
//!     WTOPENA,
//!     WTINFOA,
//!     LOGCONTEXT,
//!     PACKET
//! }
//! use libloading::{Library, Symbol};
//! 
//! fn main(){
//!     let lib = unsafe{Library::new("Wintab32.dll")?};
//!     let wtopena:Symbol<WTOPENA>  = unsafe{lib.get(c"WTOpenA".to_bytes())?};
//!     let wtinfoa:Symbol<WTINFOA>  = unsafe{lib.get(c"WTInfoA".to_bytes())?};
//! }
//! ```
//! 
mod c_type_aliases;
mod log_context;
mod axis;
mod wtpkt;
mod packet;
mod extern_function_types;
mod coordinate;
mod information_categories;
mod c_string_types;
mod bitmask;

#[cfg(feature="use_libloading")]
mod wrapper;

pub use c_type_aliases::*;
pub use c_string_types::CString40;
pub use bitmask::Bitmask;
pub use coordinate::{XY, XYZ};
pub use axis::AXIS;
pub use extern_function_types::{
    WTInfo,
    WTOpen,
    WTClose,
    WTPacket,
    WTQueuePacketsEx,
    WTDataGet,
    WTPacketsGet
};
pub use log_context::{
    LOGCONTEXT,
    CXO,
    CXL,
    CXS,
};
pub use wtpkt::WTPKT;
pub use packet::{
    Packet,
    ButtonChange,
    ButtonChangeType
};
pub use information_categories::{
    WTI,
    DVC,
    CRC,
    CTX,
    CSR,
    EXT,
    HWC,
    IFC,
    STA,
};


/// Takes a mutable reference and casts it to a void pointer: `type LPVOID = *mut std::ffi::c_void`.
#[macro_export]
macro_rules! cast_void{
    ($e:expr) => {
        &mut $e as *mut _ as *mut std::ffi::c_void
    };
}
