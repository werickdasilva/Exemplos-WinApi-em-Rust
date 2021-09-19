use std::ptr::null_mut;

use winapi::{
    shared::{
        minwindef::{HINSTANCE, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::LPCWSTR,
        windef::{HBRUSH, HWND},
    },
    um::{
        libloaderapi::GetModuleHandleW,
        wingdi::{GetStockObject, WHITE_BRUSH},
        winuser::{
            CreateWindowExW, DefWindowProcW, LoadCursorW,
            PostQuitMessage, RegisterClassExW, CS_DBLCLKS, CS_HREDRAW, CS_VREDRAW,
            IDC_ARROW, WM_DESTROY, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
        },
    },
};


fn to_utf_16(value: &str) -> Vec<u16> {
    use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

pub unsafe fn register_window_class() -> WNDCLASSEXW {
    use std::mem::size_of;

    let style = CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS;
    let instance = GetModuleHandleW(null_mut()) as HINSTANCE;
    let cursor = LoadCursorW(null_mut(), IDC_ARROW);
    // define a cor de fundo da janela 
    let background = GetStockObject(WHITE_BRUSH as i32) as HBRUSH;

    // para mais informação de como funciona o WNDCLASSEXW acesse:
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexw
    let wc = WNDCLASSEXW {
        cbSize: size_of::<WNDCLASSEXW>() as UINT,
        style,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: null_mut(),
        hCursor: cursor,
        hbrBackground: background,
        lpszMenuName: null_mut(),
        lpszClassName: to_utf_16("WC_DIALOG").as_ptr(),
        hIconSm: null_mut(),
    };

    // verifica se é possivel registrar o WNDCLASSEXW
    if RegisterClassExW(&wc) == 0 {
        panic!("Erro ao registrar Classe");
    }

    wc
}

// função usada para ver os procedimento e evento da janela
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // se o botão de fechar for clicado enviara um porcessamento de destruimento
    if msg == WM_DESTROY {
        // define para o sistema que a thread fez um procedimento de fechamento
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage
        PostQuitMessage(0);
        return 0;
    }

    // chamada dos procedimentos padrão da janela.
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw
    return DefWindowProcW(hwnd, msg, wparam, lparam);
}

pub unsafe fn create_window(class_name: LPCWSTR, instance: HINSTANCE) -> HWND {
    let window_style_extern = 0;
    let title = to_utf_16("Criando Janela");
    let window_style = WS_OVERLAPPEDWINDOW;
    let position_x = 20;
    let position_y = 30;
    let width = 300;
    let height = 400;

    // Cria uma janela para mais informação acesse:
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw
    // dependendo do tipo de valor passado para o parametro pode dar erro
    let window = CreateWindowExW(
        window_style_extern,
        class_name,
        title.as_ptr(),
        window_style,
        position_x,
        position_y,
        width,
        height,
        null_mut(),
        null_mut(),
        instance,
        null_mut(),
    );

    if window == null_mut() {
        panic!("Erro ao criar a janela....")
    }
    window
}