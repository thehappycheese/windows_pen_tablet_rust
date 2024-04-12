use std::{borrow::Borrow, sync::Arc};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle},
    window::WindowBuilder
};
use anyhow::Result;
use libloading::{Library, Symbol};

use wintab_lite::{
    cast_void, WTClose, WTDataGet, WTInfo, WTOpen, WTPacket, WTQueuePacketsEx, LOGCONTEXT, WTI
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;

    event_loop.set_control_flow(ControlFlow::Poll);
    
    let window_holder = Arc::new(window);

    let lib = unsafe{Library::new("Wintab32.dll")?};
    let wtopena:Symbol<WTOpen>  = unsafe{lib.get(c"WTOpenA".to_bytes())?};
    let wtinfoa:Symbol<WTInfo>  = unsafe{lib.get(c"WTInfoA".to_bytes())?};
    let wtclose:Symbol<WTClose>  = unsafe{lib.get(c"WTClose".to_bytes())?};
    let wtqueue:Symbol<WTQueuePacketsEx>  = unsafe{lib.get(c"WTQueuePacketsEx".to_bytes())?};
    let wtdataget:Symbol<WTDataGet>  = unsafe{lib.get(c"WTDataGet".to_bytes())?};

    let mut wintab_context_data = LOGCONTEXT::default();
    let return_value = unsafe{wtinfoa(WTI::DEFSYSCTX, 0, cast_void!(wintab_context_data))};
    assert_ne!(return_value, 0);
    println!("Default Wintab system context");
    println!("{:#?}", wintab_context_data);

    let raw_handel = (window_holder.window_handle()?).as_raw();
    let hwnd = match raw_handel{
        RawWindowHandle::Win32(Win32WindowHandle{
            hinstance:Some(_),
            hwnd,
            ..
        })=>{
            hwnd
        },
        _=>panic!("Not windows, or invalid instance. Should be unreachable since WindowBuilder succeeded above.")
    };
    let mut wintab_context_handel = unsafe{wtopena(wintab_lite::HWND(hwnd.into()), &mut wintab_context_data, 1)};
    println!("Wintab context handel");
    println!("{:?}", wintab_context_handel);

    //let wintab_context_handel = Arc::new(wintab_context_handel);

    let _ = event_loop.run(move |event, elwt| {
       
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                match unsafe{wtclose(&mut wintab_context_handel)} {
                    0=>{},
                    _=>{
                        println!("WARNIGN: WTClose Failed");
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
                let mut to = 0;
                match unsafe{wtqueue(&mut wintab_context_handel, &mut from, &mut to)} {
                    0=>{},
                    _=>{
                        println!("The packet ids from {from} to {to} are available")
                        let mut count_packets_removed_from_queue = 0;
                        const MAX_PACKETS_TO_GET: u32 = 100;
                        let mut total_actually_found = unsafe{wtdataget(
                            &mut wintab_context_handel,
                            from,
                            to,
                            MAX_PACKETS_TO_GET,
                            std::ptr::null_mut(),
                            *mut count_packets_removed_from_queue
                        )};
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
