
mod c_type_aliases;
mod log_context;

pub use c_type_aliases::*;
pub use log_context::*;

macro_rules! cast_void{
    ($e:expr) => {
        &mut $e as *mut _ as LPVOID
    };
}

pub struct HWND(pub isize);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct HCTX__ {
    pub unused: ::std::os::raw::c_int,
}
pub type HCTX = *mut HCTX__;


/// This function returns global information about the interface in an application-supplied buffer. Different types of
/// information are specified by different index arguments. Applications use this function to receive information about
/// tablet coordinates, physical dimensions, capabilities, and cursor types.
/// 
/// - `wCategory` Identifies the category from which information is being requested.
/// - `nIndex` Identifies which information is being requested from within the category.
/// - `lpOutput` Points to a buffer to hold the requested information.
/// 
/// The return value specifies the size of the returned information in bytes. If the information is not supported, the
/// function returns zero. If a tablet is not physically present, this function always returns zero.
type WTINFOA  = unsafe extern fn (wCategory: UINT, nIndex: UINT, lpOutput: LPVOID) -> UINT;

/// This function establishes an active context on the tablet. On successful completion of this function, the
/// application may begin receiving tablet events via messages (if they were requested), and may use the handle returned
/// to poll the context, or to perform other context-related functions.
/// 
/// - `hWnd` Identifies the window that owns the tablet context, and receives messages from the context.
/// - `lpLogCtx` Points to an application-provided LOGCONTEXT data structure describing the context to be opened.
/// - `fEnable` Specifies whether the new context will immediately begin processing input data.
/// 
/// The return value identifies the new context. It is NULL if the context is not opened.
type WTOPENA  = unsafe extern fn (hWnd: HWND, lpLogCtx: LPLOGCONTEXTA, fEnable: BOOL  ) -> HCTX;

/// This function fills in the passed lpPkt buffer with the context event packet having the specified serial number.
/// The returned packet and any older packets are removed from the context's internal queue.
/// 
/// - `hCtx` Identifies the context whose packets are being returned.
/// - `wSerial` Serial number of the tablet event to return.
/// - `lpPkts` Points to a buffer to receive the event packets.
/// 
/// The return value is non-zero if the specified packet was found and returned. It is zero if the specified packet was
/// not found in the queue.
type WTPACKET =     unsafe extern fn (hCtx:HCTX, wSerial:UINT, lpPkts:LPVOID) -> BOOL;


