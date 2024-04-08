use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::ValidateRect,
        System::LibraryLoader::GetModuleHandleA,
        UI::{
            Input::Pointer::{
                EnableMouseInPointer, GetPointerPenInfo, GetPointerType, POINTER_INFO, POINTER_PEN_INFO, POINTER_TOUCH_INFO, GetPointerTouchInfo
            },

            WindowsAndMessaging::*
        }
    },
};

fn main() -> Result<()> {
    
    unsafe {
        let window_handle_instance = GetModuleHandleA(None)?;
        debug_assert!(window_handle_instance.0 != 0);

        let window_class_name = s!("window");

        let window_class = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: window_handle_instance.into(),
            lpszClassName: window_class_name,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&window_class);
        debug_assert!(atom != 0);

        CreateWindowExA(
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
            window_handle_instance,
            None,
        );
        EnableMouseInPointer(true)?;
        

        let mut message = MSG::default();
        
        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

macro_rules! GET_X_LPARAM {
    ($lparam:expr) => {
        $lparam.0 & 0xFFFF
    };
}

macro_rules! GET_Y_LPARAM {
    ($lparam:expr) => {
        ($lparam.0 >> 16) & 0xFFFF
    };
}
extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    
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
            // WM_MOUSEMOVE => {
            //     // Extract the mouse position from lparam
            //     let x_pos = GET_X_LPARAM!(lparam);
            //     let y_pos = GET_Y_LPARAM!(lparam);

            //     // You can perform any action here. For demonstration, let's print the position.
            //     //println!("Mouse move at x: {}, y: {}", x_pos, y_pos);

            //     LRESULT(0)
            // },
            WM_POINTERUPDATE => {
                let pointer_id = wparam.0 as u32 & 0xFFFF;
                let mut pointer_type = POINTER_INPUT_TYPE::default();
                match GetPointerType(pointer_id, &mut pointer_type){
                    Ok(_)=>{
                        
                        match pointer_type{
                            PT_PEN =>{
                                let mut pen_info = POINTER_PEN_INFO::default();
                                match GetPointerPenInfo(pointer_id, &mut pen_info){
                                    Ok(_)=>println!("Pen Info: {:?}", pen_info),
                                    Err(_)=>println!("Pen Info Unavaliable")
                                }
                            }
                            PT_MOUSE
                            | PT_POINTER
                            | PT_TOUCH
                            | PT_TOUCHPAD=>{
                                //println!("Pointer Type: {:?}", pointer_type);
                                // let mut pen_info = POINTER_PEN_INFO::default();
                                // match GetPointerPenInfo(pointer_id, &mut pen_info){
                                //     Ok(_)=>println!("Pen Pressure: {:?}", pen_info.pressure),
                                //     Err(_)=>println!("Pen Info Unavaliable")
                                // }
                                let mut touch_info = POINTER_TOUCH_INFO::default();
                                match GetPointerTouchInfo(pointer_id, &mut touch_info){
                                    Ok(_)=>println!("Touch Info: {:?}", touch_info),
                                    Err(_)=>println!("Touch Info Unavaliable")
                                }
                            }
                            _=> unreachable!("Unrecognised Pointer Type Added to Win32")
                        }
                    }
                    Err(_)=>println!("Pointer Type Unavaliable")
                };
                DefWindowProcA(window, message, wparam, lparam)
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
            WM_POINTERACTIVATE
            //| WM_POINTERUPDATE
            | WM_POINTERCAPTURECHANGED
            | WM_POINTERDEVICECHANGE
            | WM_POINTERDEVICEINRANGE
            | WM_POINTERDEVICEOUTOFRANGE
            | WM_POINTERDOWN
            | WM_POINTERENTER
            | WM_POINTERHWHEEL
            | WM_POINTERLEAVE
            | WM_POINTERROUTEDAWAY
            | WM_POINTERROUTEDRELEASED
            | WM_POINTERROUTEDTO
            | WM_POINTERUP
            | WM_POINTERWHEEL => {
                println!("Some Pointer Event");
                DefWindowProcA(window, message, wparam, lparam)
            }
            x => {
                println!("Something else? {:?}", x);
                DefWindowProcA(window, message, wparam, lparam)
            }
        }
    }
}