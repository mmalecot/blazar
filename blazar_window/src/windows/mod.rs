//! Win32-based windows.

use crate::{Result, WindowError};
use blazar_event::{Button, Event, Key};
use blazar_win32 as win32;
use std::{
    collections::VecDeque,
    ffi::OsStr,
    iter, mem,
    os::{
        raw::{c_int, c_void},
        windows::ffi::OsStrExt,
    },
    ptr,
};

/// Represents a window.
pub struct Window {
    instance: win32::HMODULE,
    handle: win32::HWND,
    class_name: Vec<win32::WCHAR>,
    events: VecDeque<Event>,
}

impl Window {
    /// Creates a new window.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window> {
        unsafe {
            // Retrieves a module handle.
            let instance = win32::GetModuleHandleW(ptr::null());
            if instance.is_null() {
                return Err(WindowError::CreateWindowError);
            }

            // Registers the window class.
            let class_name = wide_string(concat!(env!("CARGO_PKG_NAME"), "_window"));
            let mut class: win32::WNDCLASSW = mem::zeroed();
            class.style = win32::CS_HREDRAW | win32::CS_VREDRAW;
            class.lpfnWndProc = mem::transmute::<
                Option<
                    unsafe fn(
                        win32::HWND,
                        win32::UINT,
                        win32::WPARAM,
                        win32::LPARAM,
                    ) -> win32::LRESULT,
                >,
                win32::WNDPROC,
            >(Some(Window::handle_message));
            class.hInstance = instance;
            class.lpszClassName = class_name.as_ptr();
            if win32::RegisterClassW(&class) == 0 {
                return Err(WindowError::CreateWindowError);
            }

            // Creates the window.
            let title = wide_string(&title);
            let mut rectangle = win32::RECT {
                left: 0,
                top: 0,
                right: width as win32::LONG,
                bottom: height as win32::LONG,
            };
            win32::AdjustWindowRect(
                &mut rectangle,
                win32::WS_OVERLAPPEDWINDOW | win32::WS_VISIBLE,
                win32::FALSE,
            );
            let width = rectangle.right - rectangle.left;
            let height = rectangle.bottom - rectangle.top;
            let handle = win32::CreateWindowExW(
                0,
                class_name.as_ptr(),
                title.as_ptr(),
                win32::WS_OVERLAPPEDWINDOW | win32::WS_VISIBLE,
                win32::CW_USEDEFAULT,
                win32::CW_USEDEFAULT,
                width as c_int,
                height as c_int,
                ptr::null_mut(),
                ptr::null_mut(),
                instance,
                ptr::null_mut(),
            );
            if handle.is_null() {
                win32::UnregisterClassW(class_name.as_ptr(), instance);
                return Err(WindowError::CreateWindowError);
            }

            // Creates event queue.
            let events = VecDeque::new();

            Ok(Window {
                instance,
                handle,
                class_name,
                events,
            })
        }
    }

    /// Pop the event on the top of the event queue, if any, and return it.
    pub fn poll_event(&mut self) -> Option<Event> {
        self.update_event_queue();
        self.events.pop_front()
    }

    /// Updates the event queue.
    fn update_event_queue(&mut self) {
        unsafe {
            let mut message: win32::MSG = mem::zeroed();
            while win32::PeekMessageW(&mut message, self.handle, 0, 0, win32::PM_REMOVE) > 0 {
                let window = wide_string("Window");
                win32::SetPropW(
                    self.handle,
                    window.as_ptr(),
                    self as *mut Window as *mut c_void,
                );
                win32::TranslateMessage(&message);
                win32::DispatchMessageW(&message);
            }
        }
    }

    /// Function that processes messages sent to the window.
    unsafe fn handle_message(
        handle: win32::HWND,
        message: win32::UINT,
        w_param: win32::WPARAM,
        l_param: win32::LPARAM,
    ) -> win32::LRESULT {
        let window = wide_string("Window");
        let window = win32::GetPropW(handle, window.as_ptr()) as *mut Window;
        if let Some(event) = (*window).translate_event(message, w_param, l_param) {
            (*window).events.push_back(event);
            0
        } else {
            win32::DefWindowProcW(handle, message, w_param, l_param)
        }
    }

    /// Translates an event message into `Option<Event>`.
    unsafe fn translate_event(
        &self,
        message: win32::UINT,
        w_param: win32::WPARAM,
        l_param: win32::LPARAM,
    ) -> Option<Event> {
        match message {
            // Window
            win32::WM_CLOSE => Some(Event::Close),
            win32::WM_SETFOCUS => Some(Event::GainFocus),
            win32::WM_KILLFOCUS => Some(Event::LoseFocus),
            win32::WM_SIZE => Some(Event::Resize {
                width: u32::from(win32::LOWORD(l_param as win32::DWORD)),
                height: u32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            // Keyboard
            win32::WM_KEYDOWN | win32::WM_SYSKEYDOWN
                if win32::HIWORD(l_param as win32::DWORD) & win32::KF_REPEAT == 0 =>
            {
                translate_key(w_param, l_param).map(|key| Event::KeyPress { key })
            }
            win32::WM_KEYUP | win32::WM_SYSKEYUP => {
                translate_key(w_param, l_param).map(|key| Event::KeyRelease { key })
            }
            // Mouse
            win32::WM_MOUSEWHEEL => Some(if win32::GET_WHEEL_DELTA_WPARAM(w_param) > 0 {
                Event::MouseScrollUp
            } else {
                Event::MouseScrollDown
            }),
            win32::WM_MOUSEMOVE => Some(Event::MouseMove {
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_LBUTTONDOWN => Some(Event::MouseButtonPress {
                button: Button::Left,
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_LBUTTONUP => Some(Event::MouseButtonRelease {
                button: Button::Left,
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_MBUTTONDOWN => Some(Event::MouseButtonPress {
                button: Button::Middle,
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_MBUTTONUP => Some(Event::MouseButtonRelease {
                button: Button::Middle,
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_RBUTTONDOWN => Some(Event::MouseButtonPress {
                button: Button::Right,
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_RBUTTONUP => Some(Event::MouseButtonRelease {
                button: Button::Right,
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_XBUTTONDOWN => Some(Event::MouseButtonPress {
                button: if win32::GET_XBUTTON_WPARAM(w_param) == win32::XBUTTON1 {
                    Button::Back
                } else {
                    Button::Forward
                },
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            win32::WM_XBUTTONUP => Some(Event::MouseButtonRelease {
                button: if win32::GET_XBUTTON_WPARAM(w_param) == win32::XBUTTON1 {
                    Button::Back
                } else {
                    Button::Forward
                },
                x: i32::from(win32::LOWORD(l_param as win32::DWORD)),
                y: i32::from(win32::HIWORD(l_param as win32::DWORD)),
            }),
            // Unknown
            _ => None,
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            win32::DestroyWindow(self.handle);
            win32::UnregisterClassW(self.class_name.as_ptr(), self.instance);
        }
    }
}

/// Translates a Win32 key to `Option<Key>`
fn translate_key(key: win32::WPARAM, flags: win32::LPARAM) -> Option<Key> {
    Some(match key as c_int {
        // Typing
        0x41 => Key::A,
        0x42 => Key::B,
        0x43 => Key::C,
        0x44 => Key::D,
        0x45 => Key::E,
        0x46 => Key::F,
        0x47 => Key::G,
        0x48 => Key::H,
        0x49 => Key::I,
        0x4A => Key::J,
        0x4B => Key::K,
        0x4C => Key::L,
        0x4D => Key::M,
        0x4E => Key::N,
        0x4F => Key::O,
        0x50 => Key::P,
        0x51 => Key::Q,
        0x52 => Key::R,
        0x53 => Key::S,
        0x54 => Key::T,
        0x55 => Key::U,
        0x56 => Key::V,
        0x57 => Key::W,
        0x58 => Key::X,
        0x59 => Key::Y,
        0x5A => Key::Z,
        0x30 => Key::Digit0,
        0x31 => Key::Digit1,
        0x32 => Key::Digit2,
        0x33 => Key::Digit3,
        0x34 => Key::Digit4,
        0x35 => Key::Digit5,
        0x36 => Key::Digit6,
        0x37 => Key::Digit7,
        0x38 => Key::Digit8,
        0x39 => Key::Digit9,
        win32::VK_OEM_3 => Key::Backquote,
        win32::VK_OEM_MINUS => Key::Minus,
        win32::VK_OEM_PLUS => Key::Equal,
        win32::VK_OEM_4 => Key::LeftBracket,
        win32::VK_OEM_6 => Key::RightBracket,
        win32::VK_OEM_5 => Key::Backslash,
        win32::VK_OEM_1 => Key::Semicolon,
        win32::VK_OEM_7 => Key::Quote,
        win32::VK_OEM_COMMA => Key::Comma,
        win32::VK_OEM_PERIOD => Key::Period,
        win32::VK_OEM_2 => Key::Slash,
        win32::VK_TAB => Key::Tab,
        win32::VK_CAPITAL => Key::CapsLock,
        win32::VK_SHIFT => unsafe {
            let left_shift =
                win32::MapVirtualKeyW(win32::VK_LSHIFT as win32::UINT, win32::MAPVK_VK_TO_VSC);
            let code = ((flags & (0xFF << 16)) >> 16) as win32::UINT;
            if code == left_shift {
                Key::LeftShift
            } else {
                Key::RightShift
            }
        },
        win32::VK_BACK => Key::Backspace,
        win32::VK_RETURN => {
            if is_extended_key_flag(flags) {
                Key::NumpadEnter
            } else {
                Key::Enter
            }
        }
        win32::VK_SPACE => Key::Space,
        // Control
        win32::VK_ESCAPE => Key::Escape,
        win32::VK_PRINT => Key::PrintScreen,
        win32::VK_SCROLL => Key::ScrollLock,
        win32::VK_PAUSE => Key::Pause,
        win32::VK_CONTROL => {
            if is_extended_key_flag(flags) {
                Key::RightControl
            } else {
                Key::LeftControl
            }
        }
        win32::VK_MENU => {
            if is_extended_key_flag(flags) {
                Key::RightAlt
            } else {
                Key::LeftAlt
            }
        }
        win32::VK_LWIN => Key::LeftSuper,
        win32::VK_RWIN => Key::RightSuper,
        win32::VK_APPS => Key::Menu,
        // Function
        win32::VK_F1 => Key::F1,
        win32::VK_F2 => Key::F2,
        win32::VK_F3 => Key::F3,
        win32::VK_F4 => Key::F4,
        win32::VK_F5 => Key::F5,
        win32::VK_F6 => Key::F6,
        win32::VK_F7 => Key::F7,
        win32::VK_F8 => Key::F8,
        win32::VK_F9 => Key::F9,
        win32::VK_F10 => Key::F10,
        win32::VK_F11 => Key::F11,
        win32::VK_F12 => Key::F12,
        // Navigation
        win32::VK_INSERT => Key::Insert,
        win32::VK_DELETE => Key::Delete,
        win32::VK_HOME => Key::Home,
        win32::VK_END => Key::End,
        win32::VK_PRIOR => Key::PageUp,
        win32::VK_NEXT => Key::PageDown,
        win32::VK_UP => Key::UpArrow,
        win32::VK_DOWN => Key::DownArrow,
        win32::VK_LEFT => Key::LeftArrow,
        win32::VK_RIGHT => Key::RightArrow,
        // Numeric keypad
        win32::VK_NUMLOCK => Key::NumLock,
        win32::VK_NUMPAD0 => Key::Numpad0,
        win32::VK_NUMPAD1 => Key::Numpad1,
        win32::VK_NUMPAD2 => Key::Numpad2,
        win32::VK_NUMPAD3 => Key::Numpad3,
        win32::VK_NUMPAD4 => Key::Numpad4,
        win32::VK_NUMPAD5 => Key::Numpad5,
        win32::VK_NUMPAD6 => Key::Numpad6,
        win32::VK_NUMPAD7 => Key::Numpad7,
        win32::VK_NUMPAD8 => Key::Numpad8,
        win32::VK_NUMPAD9 => Key::Numpad9,
        win32::VK_DIVIDE => Key::NumpadDivide,
        win32::VK_MULTIPLY => Key::NumpadMultiply,
        win32::VK_SUBTRACT => Key::NumpadSubtract,
        win32::VK_ADD => Key::NumpadAdd,
        win32::VK_DECIMAL => Key::NumpadDecimal,
        // Unknown
        _ => return None,
    })
}

/// Converts an `&str` into wide characters.
#[inline]
fn wide_string(string: &str) -> Vec<win32::WCHAR> {
    OsStr::new(string)
        .encode_wide()
        .chain(iter::once(0))
        .collect()
}

/// Indicates if the extended-key flag is included in the specified flags.
#[inline]
fn is_extended_key_flag(flags: win32::LPARAM) -> bool {
    win32::HIWORD(flags as win32::DWORD) & win32::KF_EXTENDED != 0
}
