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

use octotablet::{axis::Pose, events::ToolEvent, Builder};

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

fn main() -> Result<()>{
    // ==================
    // winit setup hijinks
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("octotablet test: with winit")
        .build(&event_loop)?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let window_holder = Arc::new(window);
    // =============================================
    // persuade winit to disclose the  window handel
    let hwnd = extract_window_handel(&window_holder)?;
    // ==================
    // octotablet setup
    let mut manager = Builder::new().emulate_tool_from_mouse(false).build_shared(&window_holder)?;
    println!("Tablet Backend {:?}", manager.backed());

    // setup state variables
    let mut x = 0.0;
    let mut y = 0.0;
    let mut p = 0.0;
    let mut redraw = false;

    let _ = event_loop.run(move |event, elwt| {
       
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { device_id, event, is_synthetic },
                ..
            }=>{
                println!(
                    "Keyboard input: device_id={:?}, event={:?}, is_synthetic={}",
                    device_id,
                    event,
                    is_synthetic
                );
                if event.physical_key == KeyCode::Space {
                    redraw = true;
                }
            },
            Event::AboutToWait => {
                // Application update code.
                
                match manager.pump() {
                    Ok(events)=> {
                        for event in events {
                            use octotablet::events::Event::*;
                            match event {
                                Tablet { tablet, event } => println!("Tablet: {:?}, Event: {:?}", tablet, event),
                                Tool{ tool, event } => {
                                    //println!("Tool: {:?}, Event: {:?}", tool, event)
                                    match event {
                                        ToolEvent::Pose(Pose{
                                            position:[pos_x, pos_y],
                                            pressure,
                                            ..
                                        })=>{
                                            x = pos_x;
                                            y = pos_y;
                                            p = pressure.get().unwrap_or_default();
                                        }
                                        _=>{}
                                    }
                                },
                                Pad { pad, event } => println!("Pad: {:?}, Event: {:?}", pad, event),
                            }
                        }
                    },
                    Err(e)=> println!("Pump error {:?}", e)
                }

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                window_holder.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.
                let inner_size = window_holder.inner_size();
                let rc = RECT{
                    left: 0,
                    top: 0,
                    right: inner_size.width as i32,
                    bottom: inner_size.height as i32
                };
                let mut paint_struct = PAINTSTRUCT::default();
                unsafe {
                    assert!(InvalidateRect(hwnd, Some(&rc), true).as_bool());
                    let hdc = BeginPaint(hwnd, &mut paint_struct);
                    assert!(!hdc.is_invalid());

                    if redraw {
                        redraw = false;
                        let brush = HBRUSH(GetStockObject(WHITE_BRUSH).0);
                        FillRect(hdc, &rc, brush);
                    }
                    let tx:i32 = x as i32;
                    let ty:i32 = y as i32;
                    let size: i32 = (8192.0 / 150.0 * p) as i32;
                    assert!(Ellipse(hdc, tx - size, ty - size, tx + size, ty + size).as_bool());
                    assert!(EndPaint(hwnd, &mut paint_struct).as_bool());
                };

            },
            _ => ()
        }
    })?;

    Ok(())
}
