//! Win32 FFI.

#![allow(non_camel_case_types, non_snake_case)]

use std::os::raw::*;

// Types
pub type ATOM = WORD;
pub type BOOL = c_int;
pub type CHAR = c_char;
pub type DWORD = c_ulong;
pub type FARPROC = *mut __some_function;
pub type HANDLE = *mut c_void;
pub type HBRUSH = *mut HBRUSH__;
pub type HCURSOR = HICON;
pub type HICON = *mut HICON__;
pub type HINSTANCE = *mut HINSTANCE__;
pub type HMENU = *mut HMENU__;
pub type HMODULE = HINSTANCE;
pub type HWND = *mut HWND__;
pub type LONG = c_long;
pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LPCSTR = *const CHAR;
pub type LPCWSTR = *const WCHAR;
pub type LPMSG = *mut MSG;
pub type LPRECT = *mut RECT;
pub type LPVOID = *mut c_void;
pub type LRESULT = LONG_PTR;
pub type UINT = c_uint;
pub type UINT_PTR = usize;
pub type WCHAR = wchar_t;
pub type WNDPROC = Option<unsafe extern "system" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT>;
pub type WORD = c_ushort;
pub type WPARAM = UINT_PTR;
pub type wchar_t = u16;

// Opaque structures
pub enum HBRUSH__ {}
pub enum HICON__ {}
pub enum HINSTANCE__ {}
pub enum HMENU__ {}
pub enum HWND__ {}
pub enum __some_function {}

// Structures
#[repr(C)]
pub struct MSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
}

#[repr(C)]
pub struct POINT {
    pub x: LONG,
    pub y: LONG,
}

#[repr(C)]
pub struct RECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

#[repr(C)]
pub struct WNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

// Constants
pub const FALSE: BOOL = 0;

pub const CS_VREDRAW: UINT = 0x0001;
pub const CS_HREDRAW: UINT = 0x0002;

pub const CW_USEDEFAULT: c_int = 0x8000_0000u32 as c_int;

pub const KF_EXTENDED: WORD = 0x0100;
pub const KF_REPEAT: WORD = 0x4000;

pub const MAPVK_VK_TO_VSC: UINT = 0;

pub const PM_REMOVE: UINT = 0x0001;

pub const VK_BACK: c_int = 0x08;
pub const VK_TAB: c_int = 0x09;
pub const VK_RETURN: c_int = 0x0d;
pub const VK_SHIFT: c_int = 0x10;
pub const VK_CONTROL: c_int = 0x11;
pub const VK_MENU: c_int = 0x12;
pub const VK_PAUSE: c_int = 0x13;
pub const VK_CAPITAL: c_int = 0x14;
pub const VK_ESCAPE: c_int = 0x1b;
pub const VK_SPACE: c_int = 0x20;
pub const VK_PRIOR: c_int = 0x21;
pub const VK_NEXT: c_int = 0x22;
pub const VK_END: c_int = 0x23;
pub const VK_HOME: c_int = 0x24;
pub const VK_LEFT: c_int = 0x25;
pub const VK_UP: c_int = 0x26;
pub const VK_RIGHT: c_int = 0x27;
pub const VK_DOWN: c_int = 0x28;
pub const VK_PRINT: c_int = 0x2a;
pub const VK_INSERT: c_int = 0x2d;
pub const VK_DELETE: c_int = 0x2e;
pub const VK_LWIN: c_int = 0x5b;
pub const VK_RWIN: c_int = 0x5c;
pub const VK_APPS: c_int = 0x5d;
pub const VK_NUMPAD0: c_int = 0x60;
pub const VK_NUMPAD1: c_int = 0x61;
pub const VK_NUMPAD2: c_int = 0x62;
pub const VK_NUMPAD3: c_int = 0x63;
pub const VK_NUMPAD4: c_int = 0x64;
pub const VK_NUMPAD5: c_int = 0x65;
pub const VK_NUMPAD6: c_int = 0x66;
pub const VK_NUMPAD7: c_int = 0x67;
pub const VK_NUMPAD8: c_int = 0x68;
pub const VK_NUMPAD9: c_int = 0x69;
pub const VK_MULTIPLY: c_int = 0x6a;
pub const VK_ADD: c_int = 0x6b;
pub const VK_SUBTRACT: c_int = 0x6d;
pub const VK_DECIMAL: c_int = 0x6e;
pub const VK_DIVIDE: c_int = 0x6f;
pub const VK_F1: c_int = 0x70;
pub const VK_F2: c_int = 0x71;
pub const VK_F3: c_int = 0x72;
pub const VK_F4: c_int = 0x73;
pub const VK_F5: c_int = 0x74;
pub const VK_F6: c_int = 0x75;
pub const VK_F7: c_int = 0x76;
pub const VK_F8: c_int = 0x77;
pub const VK_F9: c_int = 0x78;
pub const VK_F10: c_int = 0x79;
pub const VK_F11: c_int = 0x7a;
pub const VK_F12: c_int = 0x7b;
pub const VK_NUMLOCK: c_int = 0x90;
pub const VK_SCROLL: c_int = 0x91;
pub const VK_LSHIFT: c_int = 0xa0;
pub const VK_OEM_1: c_int = 0xba;
pub const VK_OEM_PLUS: c_int = 0xbb;
pub const VK_OEM_COMMA: c_int = 0xbc;
pub const VK_OEM_MINUS: c_int = 0xbd;
pub const VK_OEM_PERIOD: c_int = 0xbe;
pub const VK_OEM_2: c_int = 0xbf;
pub const VK_OEM_3: c_int = 0xc0;
pub const VK_OEM_4: c_int = 0xdb;
pub const VK_OEM_5: c_int = 0xdc;
pub const VK_OEM_6: c_int = 0xdd;
pub const VK_OEM_7: c_int = 0xde;

pub const WM_SIZE: UINT = 0x0005;
pub const WM_SETFOCUS: UINT = 0x0007;
pub const WM_KILLFOCUS: UINT = 0x0008;
pub const WM_CLOSE: UINT = 0x0010;
pub const WM_KEYDOWN: UINT = 0x0100;
pub const WM_KEYUP: UINT = 0x0101;
pub const WM_SYSKEYDOWN: UINT = 0x0104;
pub const WM_SYSKEYUP: UINT = 0x0105;
pub const WM_MOUSEMOVE: UINT = 0x0200;
pub const WM_LBUTTONDOWN: UINT = 0x0201;
pub const WM_LBUTTONUP: UINT = 0x0202;
pub const WM_RBUTTONDOWN: UINT = 0x0204;
pub const WM_RBUTTONUP: UINT = 0x0205;
pub const WM_MBUTTONDOWN: UINT = 0x0207;
pub const WM_MBUTTONUP: UINT = 0x0208;
pub const WM_MOUSEWHEEL: UINT = 0x020a;
pub const WM_XBUTTONDOWN: UINT = 0x020b;
pub const WM_XBUTTONUP: UINT = 0x020c;

pub const WS_OVERLAPPED: DWORD = 0x0000_0000;
pub const WS_MAXIMIZEBOX: DWORD = 0x0001_0000;
pub const WS_MINIMIZEBOX: DWORD = 0x0002_0000;
pub const WS_THICKFRAME: DWORD = 0x0004_0000;
pub const WS_SYSMENU: DWORD = 0x0008_0000;
pub const WS_CAPTION: DWORD = 0x00c0_0000;
pub const WS_VISIBLE: DWORD = 0x1000_0000;
pub const WS_OVERLAPPEDWINDOW: DWORD =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;

pub const XBUTTON1: WORD = 0x0001;

// Functions
#[cfg(feature = "dynamic_loading")]
#[link(name = "kernel32")]
extern "stdcall" {
    pub fn FreeLibrary(hLibModule: HMODULE) -> BOOL;
    pub fn GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC;
    pub fn LoadLibraryA(lpFileName: LPCSTR) -> HMODULE;
}

#[link(name = "kernel32")]
extern "stdcall" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;
}

#[link(name = "user32")]
extern "stdcall" {
    pub fn AdjustWindowRect(lpRect: LPRECT, dwStyle: DWORD, bMenu: BOOL) -> BOOL;
    pub fn CreateWindowExW(
        dwExStyle: DWORD,
        lpClassName: LPCWSTR,
        lpWindowName: LPCWSTR,
        dwStyle: DWORD,
        x: c_int,
        y: c_int,
        nWidth: c_int,
        nHeight: c_int,
        hWndParent: HWND,
        hMenu: HMENU,
        hInstance: HINSTANCE,
        lpParam: LPVOID,
    ) -> HWND;
    pub fn DefWindowProcW(hWnd: HWND, Msg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT;
    pub fn DestroyWindow(hWnd: HWND) -> BOOL;
    pub fn DispatchMessageW(lpmsg: *const MSG) -> LRESULT;
    pub fn GetPropW(hwnd: HWND, lpString: LPCWSTR) -> HANDLE;
    pub fn MapVirtualKeyW(nCode: UINT, uMapType: UINT) -> UINT;
    pub fn PeekMessageW(
        lpMsg: LPMSG,
        hWnd: HWND,
        wMsgFilterMin: UINT,
        wMsgFilterMax: UINT,
        wRemoveMsg: UINT,
    ) -> BOOL;
    pub fn RegisterClassW(lpWndClass: *const WNDCLASSW) -> ATOM;
    pub fn SetPropW(hWnd: HWND, lpString: LPCWSTR, hData: HANDLE) -> BOOL;
    pub fn TranslateMessage(lpmsg: *const MSG) -> BOOL;
    pub fn UnregisterClassW(lpClassName: LPCWSTR, hInstance: HINSTANCE) -> BOOL;
}

#[inline]
pub fn GET_WHEEL_DELTA_WPARAM(wParam: WPARAM) -> c_short {
    HIWORD(wParam as DWORD) as c_short
}

#[inline]
pub fn GET_XBUTTON_WPARAM(wParam: WPARAM) -> WORD {
    HIWORD(wParam as DWORD)
}

#[inline]
pub fn LOWORD(dwValue: DWORD) -> WORD {
    (dwValue & 0xffff) as WORD
}

#[inline]
pub fn HIWORD(dwValue: DWORD) -> WORD {
    ((dwValue >> 16) & 0xffff) as WORD
}
