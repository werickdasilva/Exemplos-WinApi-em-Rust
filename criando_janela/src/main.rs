mod window;

use std::ptr::null_mut;
use winapi::{shared::windef::POINT, um::winuser::{DispatchMessageW, GetMessageW, MSG, SW_SHOW, ShowWindow}};
use window as win;

fn main() {
    unsafe {
        let register = win::register_window_class();
        let window = win::create_window(register.lpszClassName, register.hInstance);
        
        ShowWindow(window, SW_SHOW);

        // events loop
        let mut msg = MSG {
            hwnd: std::ptr::null_mut(),
            message: 0,
            wParam: 0,
            lParam: 0,
            time: 0,
            pt: POINT { x: 0, y: 0 },
        };

        loop {
            let res = GetMessageW(&mut msg, null_mut(), 0, 0);
            if res == 0 || res == -1 {
                break;
            }

            // Despacha mensagem de procedimento da janela more information
            // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessage
            DispatchMessageW(&msg);
        }
    }
}

