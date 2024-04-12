
//! 
//! Example Usage
//! ```
//! use wintab_wrapper::{
//!     WTOPENA,
//!     WTINFOA
//! }
//! let lib = unsafe{Library::new("Wintab32.dll")?};
//! let wtopena:Symbol<WTOPENA>  = unsafe{lib.get(c"WTOpenA".to_bytes())?};
//! let wtinfoa:Symbol<WTINFOA>  = unsafe{lib.get(c"WTInfoA".to_bytes())?};
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
mod cstring_types;

#[cfg(feature="use_libloading")]
mod wrapper;

pub use c_type_aliases::*;
pub use coordinate::{XY, XYZ};
pub use axis::AXIS;
pub use extern_function_types::{
    WTInfo,
    WTOpen,
    WTClose,
    WTPacket,
    WTQueuePacketsEx,
    WTDataGet
};
pub use log_context::LOGCONTEXT;
pub use wtpkt::WTPKT;
pub use packet::{
    Packet,
    ButtonChange,
    ButtonChangeType
};
pub use information_categories::WTI;


/// Casts a mutable reference to a void pointer (LPVOID).
/// This basically destroys type information and invites bugs
#[macro_export]
macro_rules! cast_void{
    ($e:expr) => {
        &mut $e as *mut _ as $crate::LPVOID
    };
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn doo(){
        
    }
}