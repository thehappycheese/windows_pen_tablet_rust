
mod c_type_aliases;
mod log_context;
mod axis;
mod wtpkt;
mod packet;
mod extern_function_types;
mod coordinate;
mod information_categories;

pub use c_type_aliases::*;
pub use coordinate::{XY, XYZ};
pub use axis::AXIS;
pub use extern_function_types::{
    WTINFOA,
    WTOPENA,
    WTPACKET,
};
pub use log_context::LOGCONTEXT;
pub use wtpkt::WTPKT;
pub use information_categories::WTI;


/// Casts a mutable reference to a void pointer (LPVOID).
/// This basically destroys type information and invites bugs
macro_rules! cast_void{
    ($e:expr) => {
        &mut $e as *mut _ as crate::wintab::LPVOID
    };
}
