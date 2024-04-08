mod wt {
    pub use crate::wintab32_bindings::*;
}
mod wintab32_extra_impl;

use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::ValidateRect,
        System::LibraryLoader::GetModuleHandleA,
        UI::WindowsAndMessaging::*
    },
};
mod wintab32_bindings;
use anyhow::Result;

// GETPROCADDRESS( WTOPENA, WTOpenA );
// GETPROCADDRESS( WTINFOA, WTInfoA );
// GETPROCADDRESS( WTGETA, WTGetA );
// GETPROCADDRESS( WTSETA, WTSetA );
// GETPROCADDRESS( WTPACKET, WTPacket );
// GETPROCADDRESS( WTCLOSE, WTClose );
// GETPROCADDRESS( WTENABLE, WTEnable );
// GETPROCADDRESS( WTOVERLAP, WTOverlap );
// GETPROCADDRESS( WTSAVE, WTSave );
// GETPROCADDRESS( WTCONFIG, WTConfig );
// GETPROCADDRESS( WTRESTORE, WTRestore );
// GETPROCADDRESS( WTEXTSET, WTExtSet );
// GETPROCADDRESS( WTEXTGET, WTExtGet );
// GETPROCADDRESS( WTQUEUESIZESET, WTQueueSizeSet );
// GETPROCADDRESS( WTDATAPEEK, WTDataPeek );
// GETPROCADDRESS( WTPACKETSGET, WTPacketsGet );
// GETPROCADDRESS( WTMGROPEN, WTMgrOpen );
// GETPROCADDRESS( WTMGRCLOSE, WTMgrClose );
// GETPROCADDRESS( WTMGRDEFCONTEXT, WTMgrDefContext );
// GETPROCADDRESS( WTMGRDEFCONTEXTEX, WTMgrDefContextEx );

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
type WTINFOA  = unsafe extern fn (wCategory: wt::UINT, nIndex: wt::UINT, lpOutput: wt::LPVOID) -> wt::UINT;

/// This function establishes an active context on the tablet. On successful completion of this function, the
/// application may begin receiving tablet events via messages (if they were requested), and may use the handle returned
/// to poll the context, or to perform other context-related functions.
/// 
/// - `hWnd` Identifies the window that owns the tablet context, and receives messages from the context.
/// - `lpLogCtx` Points to an application-provided LOGCONTEXT data structure describing the context to be opened.
/// - `fEnable` Specifies whether the new context will immediately begin processing input data.
/// 
/// The return value identifies the new context. It is NULL if the context is not opened.
type WTOPENA  = unsafe extern fn (hWnd: HWND, lpLogCtx: wt::LPLOGCONTEXTA, fEnable: wt::BOOL  ) -> wt::HCTX;

/// This function fills in the passed lpPkt buffer with the context event packet having the specified serial number.
/// The returned packet and any older packets are removed from the context's internal queue.
/// 
/// - `hCtx` Identifies the context whose packets are being returned.
/// - `wSerial` Serial number of the tablet event to return.
/// - `lpPkts` Points to a buffer to receive the event packets.
/// 
/// The return value is non-zero if the specified packet was found and returned. It is zero if the specified packet was
/// not found in the queue.
type WTPACKET =     unsafe extern fn (hCtx: wt::HCTX, wSerial: wt::UINT, lpPkts: wt::LPVOID) -> wt::BOOL;

static mut WINTAB_CONTEXT:Option<wt::HCTX> = None;


macro_rules! cast_void{
    ($e:expr) => {
        &mut $e as *mut _ as wt::LPVOID
    };
}

fn main() -> Result<()> {
    unsafe {
        let lib = match libloading::Library::new("Wintab32.dll"){
            Ok(lib) => lib,
            Err(e) => panic!("Unable to load Wintab32.dll {}",e)
        };

        let wtopena:libloading::Symbol<WTOPENA>  = lib.get(c"WTOpenA".to_bytes())?;
        let wtinfoa:libloading::Symbol<WTINFOA>  = lib.get(c"WTInfoA".to_bytes())?;

        let window_module_handle = GetModuleHandleA(None)?;
        debug_assert!(window_module_handle.0 != 0);

        let window_class_name = s!("window");

        let window_class = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: window_module_handle.into(),
            lpszClassName: window_class_name,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&window_class);
        debug_assert!(atom != 0);

        let window_handel = CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class_name,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            window_module_handle,
            None,
        );
        
        let mut log_context = wt::LOGCONTEXT::default();

        // let hctx:wt::HCTX = std::ptr::null_mut();
        let wDevice       : wt::UINT = 0;
        let wExtX         : wt::UINT = 0;
        let wExtY         : wt::UINT = 0;
        let wWTInfoRetVal : wt::UINT = 0;
        
        let mut TabletX   : wt::AXIS = wt::AXIS{
            axMin:0,
            axMax:0,
            axUnits:0,
            axResolution:0,
        };

        let mut TabletY   : wt::AXIS = wt::AXIS{
            axMin: 0,
            axMax: 0,
            axUnits: 0,
            axResolution: 0,
        };
        
        // move system cursor before getting default system context
        log_context.lcOptions |= wt::CXO_SYSTEM;


        
        let wtinfoa_return_value = wtinfoa(wt::WTI_DEFCONTEXT, 0, cast_void!(log_context));

        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<wt::LOGCONTEXT>());
        assert!(log_context.lcOptions & wt::CXO_SYSTEM != 0);

        let new_lcname = format!("PrsTest Digitizing {window_module_handle:?}");
        // Write to lcname without using too many characters.
        for (i, c) in log_context.lcName.iter_mut().enumerate() {
            *c = match new_lcname.chars().nth(i) {
                Some(c) => match (c as i8).try_into() {
                    Ok(c) => c,
                    Err(_) => 32,
                },
                None => 0,
            }
        }

        // Note
        // PACKETMODE and PACKETDATA are manually added to ./wintab32_headers/wrapper.h
        // this is required because they influence the typedef of wt::PACKET in PKTDEF.H

        log_context.lcPktData = wt::PACKETDATA;

        // Which packet items should show change in value since the last
        // packet (referred to as 'relative' data) and which items
        // should be 'absolute'.
        log_context.lcPktMode = wt::PACKETMODE;

        // This bitfield determines whether or not this context will receive
        // a packet when a value for each packet field changes.  This is not
        // supported by the Intuos Wintab.  Your context will always receive
        // packets, even if there has been no change in the data.
        log_context.lcMoveMask = wt::PACKETDATA;

        // Which buttons events will be handled by this context.  lcBtnMask
        // is a bitfield with one bit per button.
        log_context.lcBtnUpMask = log_context.lcBtnDnMask;

        // Set the entire tablet as active
        // Note: only works with 0th tablet! clear your tablet prefs;
        //       otherwise, you may get some funky behavior
        let wtinfoa_return_value = wtinfoa(wt::WTI_DEVICES + 0, wt::DVC_X, cast_void!(TabletX));
        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<wt::AXIS>());

        let wtinfoa_return_value = wtinfoa(wt::WTI_DEVICES, wt::DVC_Y, cast_void!(TabletY));
        assert_eq!(wtinfoa_return_value as usize, std::mem::size_of::<wt::AXIS>());

        log_context.lcInOrgX = 0;
        log_context.lcInOrgY = 0;
        log_context.lcInExtX = TabletX.axMax;
        log_context.lcInExtY = TabletY.axMax;

        // Guarantee the output coordinate space to be in screen coordinates.
        log_context.lcOutOrgX = GetSystemMetrics(SM_XVIRTUALSCREEN);
        log_context.lcOutOrgY = GetSystemMetrics(SM_YVIRTUALSCREEN);
        log_context.lcOutExtX = GetSystemMetrics(SM_CXVIRTUALSCREEN); //SM_CXSCREEN);

        // In Wintab, the tablet origin is lower left.  Move origin to upper left
        // so that it coincides with screen origin.
        log_context.lcOutExtY = -GetSystemMetrics(SM_CYVIRTUALSCREEN);	//SM_CYSCREEN);

        // Leave the system origin and extents as received:
        // lcSysOrgX, lcSysOrgY, lcSysExtX, lcSysExtY

        // open the tablet context
        // The Wintab spec says we must open the context disabled if we are using cursor masks.
        let _hctx = wtopena(window_handel, &mut log_context as *mut _, 0);

        let mut message = MSG::default();
        
        
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }
    }
    Ok(())
}



// Window Message Enum
const WT_PACKET    : u32 = wt::WT_DEFBASE + 0;
const WT_CTXOPEN   : u32 = wt::WT_DEFBASE + 1;
const WT_CTXCLOSE  : u32 = wt::WT_DEFBASE + 2;
const WT_CTXUPDATE : u32 = wt::WT_DEFBASE + 3;
const WT_CTXOVERLAP: u32 = wt::WT_DEFBASE + 4;
const WT_PROXIMITY : u32 = wt::WT_DEFBASE + 5;
const WT_INFOCHANGE: u32 = wt::WT_DEFBASE + 6;
const WT_CSRCHANGE : u32 = wt::WT_DEFBASE + 7;
const WT_PACKETEXT : u32 = wt::WT_DEFBASE + 8;
const WT_MAX       : u32 = wt::WT_DEFBASE + wt::WT_MAXOFFSET;

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // The original header used macro templates to generate these, because apparently it isnt allowed to be easy.
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_SETCURSOR
            | WM_LBUTTONDOWN
            | WM_LBUTTONUP
            | WM_GETMINMAXINFO
            | WM_CREATE
            | WM_SHOWWINDOW
            | WM_NCCREATE
            | WM_NCHITTEST
            | WM_IME_SETCONTEXT
            | WM_IME_NOTIFY
            | WM_KILLFOCUS
            | WM_ACTIVATE
            | WM_CLOSE
            | WM_NCMOUSEMOVE
            | WM_NCLBUTTONDOWN
            | WM_MOUSEFIRST
            | WM_ACTIVATEAPP
            | WM_NCMOUSELEAVE
            | WM_GETICON
            | WM_GETOBJECT
            | WM_WINDOWPOSCHANGING
            | WM_SIZE
            | WM_MOVE
            | WM_DWMNCRENDERINGCHANGED
            | WM_WINDOWPOSCHANGED
            | WM_MOVING
            | WM_CAPTURECHANGED
            | WM_EXITSIZEMOVE
            | WM_NCPAINT
            | WM_ERASEBKGND => {
                DefWindowProcA(window, message, wparam, lparam)
            }
            // WINTAB EVENTS
            WT_PACKET=>{
                println!("GOT A WT_PACKET! Yay!");
                let mut packet = wt::PACKET::default();

                // Have load the dynamic library again because I can't seem persude rust make this a `mut static`
                // variable. Sure, cool, great. Thanks for that rust. I sure hope this processes is cached or memoized
                // internally somewhere.
                let lib = libloading::Library::new("wintab32.dll").unwrap();
                let wtpacket: libloading::Symbol<WTPACKET> = match lib.get(c"WTPacket".to_bytes()){
                    Ok(symbol) => symbol,
                    Err(err)   => panic!("Failed get wtpacket symbol :( {err}"),
                };

                let wtpacket_response =  wtpacket(lparam.0 as wt::HCTX, wparam.0 as u32, cast_void!(packet));
                assert!(wtpacket_response!=0);
                println!("Got a packet! Whooo! {:?}", packet);

                DefWindowProcA(window, message, wparam, lparam)
            }
            x => {
                println!("Something else? {:?}", x);
                DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }
}
