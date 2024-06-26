use std::sync::Arc;

use anyhow::{anyhow, Result};
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{
        BeginPaint, Ellipse, EndPaint, FillRect, GetStockObject, InvalidateRect, SelectObject,
        BLACK_PEN, HBRUSH, PAINTSTRUCT, WHITE_BRUSH,
    },
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::KeyCode,
    raw_window_handle::{HasWindowHandle, RawWindowHandle, Win32WindowHandle},
    window::{Window, WindowBuilder},
};

use libloading::Library;
use wintab_lite::{
    cast_void, Packet, WTClose, WTDataGet, WTInfo, WTOpen, WTQueuePacketsEx, AXIS, CXO, DVC,
    LOGCONTEXT, WTI, WTPKT, XYZ,
};

fn extract_window_handel(window_holder: &Window) -> Result<HWND> {
    let handel = window_holder.window_handle()?;
    match handel.as_raw() {
        RawWindowHandle::Win32(Win32WindowHandle {
            hinstance: Some(_),
            hwnd,
            ..
        }) => Ok(HWND(hwnd.into())),
        _ => Err(anyhow!("Not windows, or invalid instance.")),
    }
}

fn main() -> Result<()> {
    // ==================
    // winit setup hijinks
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("wintab_lite example: winit_libloading")
        .build(&event_loop)?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let window_holder = Arc::new(window);

    // ======================================================
    // wintab lite can be loaded using libloading or raw_dylib depending on the feature flags
    let lib = unsafe { Library::new("Wintab32.dll")? };
    let wintab_open: WTOpen = unsafe { lib.get(c"WTOpenA".to_bytes())? };
    let wintab_info: WTInfo = unsafe { lib.get(c"WTInfoA".to_bytes())? };
    let wintab_close: WTClose = unsafe { lib.get(c"WTClose".to_bytes())? };
    let wintab_queue: WTQueuePacketsEx = unsafe { lib.get(c"WTQueuePacketsEx".to_bytes())? };
    let wintab_data_get: WTDataGet = unsafe { lib.get(c"WTDataGet".to_bytes())? };

    // ==========================================
    // mutable variables that wintab can write to
    let mut log_context = LOGCONTEXT::default();
    let mut tablet_x = AXIS::default();
    let mut tablet_y = AXIS::default();

    // =============================================
    // persuade winit to disclose the  window handel
    let hwnd = extract_window_handel(&window_holder)?;
    // ======================================
    // Query wintab for its default 'context'
    let return_value = unsafe { wintab_info(WTI::DEFSYSCTX, 0, cast_void!(log_context)) };
    assert_ne!(return_value, 0);
    println!("Default Wintab system context");
    println!("{:#?}", log_context);

    // ======================================
    // Give the context a custom name. Why? Dunno. The example code does this.
    log_context
        .lcName
        .write_str(format!("Custom Ctx Name {hwnd:?}").as_str());

    // ======================================
    // Make it so that the tablet moves the system mouse cursor
    // Although the docs say otherwise, the default context
    // appears to always have this set anyway
    log_context.lcOptions |= CXO::SYSTEM;

    // ======================================
    // Configure the fields of the Packet Struct that will be returned
    // This flexibility is unnecessary if we statically define the struct
    // and just grab all fields
    log_context.lcPktData = WTPKT::all();
    log_context.lcPktMode = WTPKT::empty();
    //log_context.lcPktMode   = WTPKT::BUTTONS; // does not work when this is set on my system
    log_context.lcMoveMask = WTPKT::X | WTPKT::Y | WTPKT::NORMAL_PRESSURE;
    // This is pointless as far as I can tell:
    log_context.lcBtnUpMask = log_context.lcBtnDnMask;

    // ======================================
    // Request Device Name. this is done in 2 steps since there is no documented maximum
    // buffer length 👍
    let result = unsafe { wintab_info(WTI::DEVICES, DVC::NAME as u32, std::ptr::null_mut()) };
    println!("Byte syze of DVC::NAME {result:?}");
    let mut device_name = vec![0u8; result as usize];
    let _result = unsafe {
        wintab_info(
            WTI::DEVICES,
            DVC::NAME as u32,
            device_name.as_mut_ptr() as *mut std::ffi::c_void,
        )
    };
    unsafe {
        println!(
            "Result of DVC::NAME {:?}",
            String::from_utf8_unchecked(device_name)
        )
    };

    // ======================================
    // Request device axes
    let result = unsafe { wintab_info(WTI::DEVICES, DVC::X as u32, cast_void!(tablet_x)) };
    assert_eq!(result as usize, std::mem::size_of::<AXIS>());
    let result = unsafe { wintab_info(WTI::DEVICES, DVC::Y as u32, cast_void!(tablet_y)) };
    assert_eq!(result as usize, std::mem::size_of::<AXIS>());
    println!("Tablet x,y axes");
    println!("{:#?}", tablet_x);
    println!("{:#?}", tablet_y);

    // ======================================
    // configure the context.

    // I found this was a redundant assignment when testing:
    log_context.lcInOrgXYZ = XYZ::default();
    // found this is a redundant assignment when testing:
    log_context.lcInExtXYZ = XYZ {
        x: tablet_x.axMax,
        y: tablet_y.axMax,
        z: 0,
    };

    // =======================================================
    // Tablet output coordinates are upside down by default 🙃
    let default_y_extent = log_context.lcOutExtXYZ.y;
    log_context.lcOutExtXYZ.y = -default_y_extent;
    //log_context.lcOutOrgXYZ.y = default_y_extent;

    // ======================================
    // Open the context
    // use the laboriously configured LOGCONTEXT struct to finally open a connection with our window
    // The example says we are supposed to open it in the disabled state... but why. I just open it
    // in enabled state.
    let wintab_context_handel = unsafe { wintab_open(hwnd, &mut log_context, 1) };
    println!("Wintab context handel {:?}", wintab_context_handel);
    println!("Log Context after open {log_context:#?}");

    let mut x = 0;
    let mut y = 0;
    let mut p = 0;
    let mut redraw = false;

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                match unsafe { wintab_close(wintab_context_handel) } {
                    0 => {}
                    _ => {
                        println!("WARNING: WTClose Failed");
                    }
                };
                elwt.exit();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        device_id,
                        event,
                        is_synthetic,
                    },
                ..
            } => {
                println!(
                    "Keyboard input: device_id={:?}, event={:?}, is_synthetic={}",
                    device_id, event, is_synthetic
                );
                if event.physical_key == KeyCode::Space {
                    redraw = true;
                }
            }
            Event::AboutToWait => {
                // Application update code.
                let mut from = 0;
                let mut to = 0;
                match unsafe { wintab_queue(wintab_context_handel, &mut from, &mut to) } {
                    0 => {}
                    _ => {
                        let mut count_packets_removed_from_queue = 0;
                        const MAX_PACKETS_TO_GET: i32 = 100;
                        let mut packets: [Packet; MAX_PACKETS_TO_GET as usize] =
                            core::array::from_fn(|_| Packet::default());
                        let _total_actually_found = unsafe {
                            wintab_data_get(
                                wintab_context_handel,
                                from,
                                to,
                                MAX_PACKETS_TO_GET,
                                cast_void!(packets),
                                &mut count_packets_removed_from_queue,
                            )
                        };

                        let packets = &packets[0..count_packets_removed_from_queue as usize];

                        // println!("Available: {from}-{to} Found {total_actually_found} Removed {count_packets_removed_from_queue}");
                        if count_packets_removed_from_queue > 0 {
                            // println!("============ {count_packets_removed_from_queue}");
                            // packets.iter().for_each(|packet|println!("{packet:#?}"));

                            packets.last().map(|packet| {
                                x = packet.pkXYZ.x;
                                y = packet.pkXYZ.y;
                                p = packet.pkNormalPressure;
                            });
                        }
                    }
                }

                window_holder.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                window_holder.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application (allows OS to request redraw)
                let inner_position = match window_holder.inner_position() {
                    Ok(inner_position) => inner_position,
                    Err(_) => panic!("Could not obtain inner position"),
                };
                let inner_size = window_holder.inner_size();

                let tx = x - inner_position.x;
                let ty = y - inner_position.y;
                //println!("Redraw requested, inner position {inner_position:?}, raw({x} {y}) tx({tx} {ty}) ");

                let mut paint_struct: PAINTSTRUCT = PAINTSTRUCT::default();
                let rc: RECT = RECT {
                    left: 0,
                    top: 0,
                    right: inner_size.width as i32,
                    bottom: inner_size.height as i32,
                };

                unsafe {
                    assert!(InvalidateRect(hwnd, Some(&rc), true).as_bool());

                    let hdc = BeginPaint(hwnd, &mut paint_struct);
                    assert!(!hdc.is_invalid());

                    if redraw {
                        redraw = false;
                        let brush = HBRUSH(GetStockObject(WHITE_BRUSH).0);
                        FillRect(hdc, &rc, brush);
                    }

                    //assert!(PatBlt(hdc, tx,   ty-15,1 ,30, BLACKNESS).as_bool());
                    //assert!(PatBlt(hdc, tx-15,ty   ,30, 1, BLACKNESS).as_bool());
                    let size: i32 = p as i32 / 150;
                    SelectObject(hdc, GetStockObject(BLACK_PEN));
                    SelectObject(hdc, GetStockObject(WHITE_BRUSH));
                    assert!(Ellipse(hdc, tx - size, ty - size, tx + size, ty + size).as_bool());
                    assert!(EndPaint(hwnd, &mut paint_struct).as_bool());
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::TouchpadPressure {
                        device_id,
                        pressure,
                        stage,
                    },
                ..
            } => {
                println!(
                    "Touchpad pressure: device_id={:?}, pressure={:?}, stage={:?}",
                    device_id, pressure, stage
                );
            }
            _ => (),
        }
    })?;

    Ok(())
}