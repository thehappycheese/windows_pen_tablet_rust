use windows::{
    core::*,
    Win32::{
        Foundation::*,
        Graphics::Gdi::{
            BeginPaint, ClientToScreen, Ellipse, EndPaint, FillRect, GetStockObject, InvalidateRect, RedrawWindow, SelectObject, BLACK_PEN, HBRUSH, HRGN, PAINTSTRUCT, RDW_INTERNALPAINT, WHITE_BRUSH
        },
        System::LibraryLoader::GetModuleHandleA,
        UI::{
            Input::{
                KeyboardAndMouse::{VIRTUAL_KEY, VK_SPACE}, Pointer::{
                    EnableMouseInPointer, GetPointerPenInfo, GetPointerTouchInfo, GetPointerType, POINTER_INFO, POINTER_PEN_INFO, POINTER_TOUCH_INFO
                }
            },

            WindowsAndMessaging::*
        }
    },
};


static mut X: i32 = 0;
static mut Y: i32 = 0;
static mut P:u32 = 0;
static mut REDRAW: bool = false;

fn main() -> Result<()> {
    
    unsafe {
        let window_handle_instance = GetModuleHandleA(None)?;
        debug_assert!(!window_handle_instance.is_invalid());

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
                //assert!(ValidateRect(window, None).as_bool());
                assert!(InvalidateRect(window, None, false).as_bool());

                let mut client_rect: RECT = RECT::default();
                GetClientRect(window, &mut client_rect).unwrap();
                // Transform the tablet input into client window coordinates
                let mut inner_position = POINT::default();
                assert!(ClientToScreen(window, &mut inner_position).as_bool());

                let tx = X - inner_position.x;
                let ty = Y - inner_position.y;

                println!("{client_rect:?} {inner_position:?} {X} {Y} {tx} {ty}");

                // create a paint context objet thingo
                let mut paint_struct: PAINTSTRUCT = PAINTSTRUCT::default();

                // start painting
                let hdc = BeginPaint(window, &mut paint_struct);

                if REDRAW {
                    println!("REDRAW was true");
                    REDRAW = false;
                    let brush = HBRUSH(GetStockObject(WHITE_BRUSH).0);
                    FillRect(hdc, &client_rect, brush);
                }
                let size: i32 = ((P as f32 )/1024.0 * 8192.0 / 150.0) as i32;
                SelectObject(hdc, GetStockObject(BLACK_PEN));
                SelectObject(hdc, GetStockObject(WHITE_BRUSH));
                assert!(Ellipse(hdc, tx - size, ty - size, tx + size, ty + size).as_bool());
                assert!(EndPaint(window, &mut paint_struct).as_bool());
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            // commented out to reduce terminal noise
            // WM_MOUSEMOVE => {
            //     // Extract the mouse position from lparam
            //     let x_pos = GET_X_LPARAM!(lparam);
            //     let y_pos = GET_Y_LPARAM!(lparam);
            //     //println!("Mouse move at x: {}, y: {}", x_pos, y_pos);

            //     LRESULT(0)
            // },
            WM_KEYDOWN => {
                let key = VIRTUAL_KEY(wparam.0 as u16);
                if key == VK_SPACE {
                    REDRAW = true;
                    assert!(RedrawWindow(window, None, HRGN(0), RDW_INTERNALPAINT).as_bool());
                }
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
            WM_POINTERUPDATE => {
                let pointer_id = wparam.0 as u32 & 0xFFFF;
                let mut pointer_type = POINTER_INPUT_TYPE::default();
                match GetPointerType(pointer_id, &mut pointer_type){
                    Ok(_)=>{
                        
                        match pointer_type{
                            PT_PEN =>{
                                let mut pen_info = POINTER_PEN_INFO::default();
                                match GetPointerPenInfo(pointer_id, &mut pen_info){
                                    Ok(_)=>{

                                        let POINTER_PEN_INFO{
                                            pressure,
                                            pointerInfo: POINTER_INFO{
                                                ptPixelLocation,
                                                ..
                                            },
                                            ..
                                        }=pen_info;
                                        P = pressure;
                                        X = ptPixelLocation.x;
                                        Y = ptPixelLocation.y;
                                        if P>0{
                                            assert!(RedrawWindow(window, None, HRGN(0), RDW_INTERNALPAINT).as_bool());
                                        }
                                        //println!("Pen Info: {:?}", pen_info)

                                    },
                                    Err(_)=>{}//println!("Pen Info Unavailable")
                                }
                            }
                            PT_MOUSE
                            | PT_POINTER
                            | PT_TOUCH
                            | PT_TOUCHPAD=>{
                                // ...
                            }
                            _=> unreachable!("Unrecognized Pointer Type Added to Win32")
                        }
                    }
                    Err(_)=>println!("Pointer Type Unavailable")
                };
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