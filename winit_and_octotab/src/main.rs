use std::sync::Arc;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,

};
use anyhow::Result;
use octotablet::Builder;



fn main() -> Result<()>{
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new().build(&event_loop)?;

    event_loop.set_control_flow(ControlFlow::Poll);
    
    let window_holder = Arc::new(window); // This assumes your window or a wrapper thereof implements the necessary traits
    let mut manager = Builder::new().emulate_tool_from_mouse(false).build_shared(&window_holder)?;
    println!("Tablet Backend {:?}", manager.backed());

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
                
                match manager.pump() {
                    Ok(events)=> {
                        for event in events {
                            use octotablet::events::Event::*;
                            match event {
                                // want events from "Huion Tablet_GT2401"
                                Tablet { tablet, event } => println!("Tablet: {:?}, Event: {:?}", tablet, event),
                                Tool{ tool, event } => println!("Tool: {:?}, Event: {:?}", tool, event),
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
            },
            _ => ()
        }
    })?;

    Ok(())
}
