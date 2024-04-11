use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use anyhow::Result;
use libloading::{Library, Symbol};

use wintab_wrapper::{
    HCTX,
    WTOPENA,
    WTINFOA,
    WTPACKET,
    WTI,
    cast_void
};

fn main() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;

    event_loop.set_control_flow(ControlFlow::Wait);
    
    let window_holder = Arc::new(window);

    let lib = unsafe{Library::new("Wintab32.dll")?};
    let wtopena:Symbol<WTOPENA>  = unsafe{lib.get(c"WTOpenA".to_bytes())?};
    let wtinfoa:Symbol<WTINFOA>  = unsafe{lib.get(c"WTInfoA".to_bytes())?};


    

    // let ctx:HCTX = HCTX::default();
    // Get a default System context 
    //wtinfoa(WTI::DEFSYSCTX, 0, cast_void!(ctx) );
    


    
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
                println!("Keyboard input: device_id={:?}, event={:?}, is_synthetic={}", device_id, event, is_synthetic);
            },
            Event::AboutToWait => {
                // Application update code.
                
                window_holder.request_redraw();
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
