use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle},
    window::WindowBuilder
};
use anyhow::Result;
use libloading::{Library, Symbol};

use wintab_lite::{
    cast_void,
    Packet,
    WTClose,
    WTDataGet,
    WTInfo,
    WTOpen,
    WTPacket,
    WTQueuePacketsEx,
    AXIS,
    DVC,
    LOGCONTEXT,
    WTI,
    WTPKT,
    XYZ
};

fn main() -> Result<()> {
    // ==================
    // winit setup hijinks
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let window_holder = Arc::new(window);

    // ======================================================
    // wintab can only be dynamically linked as far as I know
    let lib = unsafe{Library::new("Wintab32.dll")?};
    let wtopena:Symbol<WTOpen>  = unsafe{lib.get(c"WTOpenA".to_bytes())?};
    let wtinfoa:Symbol<WTInfo>  = unsafe{lib.get(c"WTInfoA".to_bytes())?};
    let wtclose:Symbol<WTClose>  = unsafe{lib.get(c"WTClose".to_bytes())?};
    let wtqueue:Symbol<WTQueuePacketsEx>  = unsafe{lib.get(c"WTQueuePacketsEx".to_bytes())?};
    let wtdataget:Symbol<WTDataGet>  = unsafe{lib.get(c"WTDataGet".to_bytes())?};
    let wtpacket:Symbol<WTPacket> = unsafe {lib.get(c"WTPacket".to_bytes())?};

    // ==========================================
    // mutable variables that wintab can write to
    let mut log_context = LOGCONTEXT::default();
    let mut tablet_x = AXIS::default();
    let mut tablet_y = AXIS::default();

    // =============================================
    // persuade winit to disclose the  window handel
    let hwnd = match (window_holder.window_handle()?).as_raw(){
        RawWindowHandle::Win32(Win32WindowHandle{
            hinstance:Some(_),
            hwnd,
            ..
        })=>{
            hwnd
        },
        _=>panic!("Not windows, or invalid instance. Should be unreachable since WindowBuilder succeeded above.")
    };
    // ======================================
    // Query wintab for its default 'context'
    let return_value = unsafe{wtinfoa(WTI::DEFSYSCTX, 0, cast_void!(log_context))};
    assert_ne!(return_value, 0);
    println!("Default Wintab system context");
    println!("{:#?}", log_context);

    // ======================================
    // Give the context a custom name. Why? Dunno. The example code does this.
    log_context.lcName.write_str(format!("Custom Ctx Name {hwnd:?}").as_str());

    // ======================================
    // Configure the fields of the Packet Struct that will be returned
    // This flexibility is unnecessary if we statically define the struct
    // and just grab all fields
    log_context.lcPktData   = WTPKT::all();
    log_context.lcPktMode   = WTPKT::BUTTONS;
    log_context.lcMoveMask  = WTPKT::X | WTPKT::Y | WTPKT::BUTTONS | WTPKT::NORMAL_PRESSURE;
    // This is pointless as far as I can tell:
    log_context.lcBtnUpMask = log_context.lcBtnDnMask;

    // ======================================
    // Request Device Name. this is done in 2 steps since there is no documented maximum buffer length 👍
    let result = unsafe{wtinfoa(WTI::DEVICES, DVC::NAME as u32, std::ptr::null_mut())};
    println!("Byte syze of DVC::NAME {result:?}");
    let mut device_name = vec![0u8; result as usize];
    let result = unsafe{wtinfoa(WTI::DEVICES, DVC::NAME as u32, device_name.as_mut_ptr() as *mut std::ffi::c_void)};
    unsafe{println!("Result of DVC::NAME {:?}", String::from_utf8_unchecked(device_name))};
    
    // ======================================
    // Request device axes
    let result = unsafe{wtinfoa(WTI::DEVICES, DVC::X as u32, cast_void!(tablet_x))};
    assert_eq!(result as usize, std::mem::size_of::<AXIS>());
    let result = unsafe{wtinfoa(WTI::DEVICES, DVC::Y as u32, cast_void!(tablet_y))};
    assert_eq!(result as usize, std::mem::size_of::<AXIS>());
    println!("Tablet x,y axes");
    println!("{:#?}", tablet_x);
    println!("{:#?}", tablet_y);

    // ======================================
    // configure the context.
    // The example code does a heap more stuff here, assigning variables to the context to configure it.
    // It queries the window and desktop sizes etc. I have found that none of this is needed. The default
    // context is already configured as needed. I am not convinced there is a need for the next few lines
    log_context.lcInOrgXYZ = XYZ::default();
    log_context.lcInExtXYZ = XYZ{x:tablet_x.axMax,y:tablet_y.axMax,z:0};
    //log_context.lcOutExtXYZ.y = -log_context.lcOutExtXYZ.y;
    
    // ======================================
    // Open the context
    // use the laboriously configured LOGCONTEXT struct to finally open a connection with our window
    // The example says we are supposed to open it in the disabled state... but why. I just open it in enabled state.
    let wintab_context_handel = unsafe{wtopena(hwnd.into(), &mut log_context, 1)};
    println!("Wintab context handel {:?}", wintab_context_handel);
    println!("Log Context after open {log_context:#?}");

    let _ = event_loop.run(move |event, elwt| {
       
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                match unsafe{wtclose(wintab_context_handel)} {
                    0=>{},
                    _=>{
                        println!("WARNING: WTClose Failed");
                    }
                };
                elwt.exit();
            },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { device_id, event, is_synthetic },
                ..
            }=>{
                println!("Keyboard input: device_id={:?}, event={:?}, is_synthetic={}", device_id, event, is_synthetic);
            },
            Event::AboutToWait => {
                // Application update code.
                //println!("About to wait");
                let mut from = 0;
                let mut to   = 0;
                match unsafe{wtqueue(wintab_context_handel, &mut from, &mut to)} {
                    0=>{},
                    _=>{

                        // let mut packet = Packet::default();
                        // let result = unsafe{wtpacket(wintab_context_handel, to, cast_void!(packet))};
                        // if result!=0{
                        //     println!("Size of one packet {result:?}");
                        //     println!("{packet:#?}");
                        // }

                        let mut count_packets_removed_from_queue = 0;
                        const MAX_PACKETS_TO_GET: i32 = 100;
                        let mut packets:[Packet; MAX_PACKETS_TO_GET as usize] = core::array::from_fn(|_|Packet::default());
                        let total_actually_found = unsafe{wtdataget(
                            wintab_context_handel,
                            from,
                            to,
                            MAX_PACKETS_TO_GET,
                            cast_void!(packets),
                            &mut count_packets_removed_from_queue
                        )};

                        //println!("Avaliable: {from}-{to} Found {total_actually_found} Removed {count_packets_removed_from_queue}");
                        if count_packets_removed_from_queue>0{
                            println!("============ {count_packets_removed_from_queue}");
                            packets[0..count_packets_removed_from_queue as usize].iter().for_each(|packet|println!("{packet:#?}"));
                        }
                    }
                }

                //window_holder.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application (allows OS to request redraw)
            },
            Event::WindowEvent {
                event: WindowEvent::TouchpadPressure { device_id, pressure, stage },
                ..
            } => {
                println!("Touchpad pressure: device_id={:?}, pressure={:?}, stage={:?}", device_id, pressure, stage);
            }
            _ => ()
        }
    })?;

    Ok(())
}
