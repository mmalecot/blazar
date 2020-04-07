//! Win32-based windows.

use crate::{CreateWindowError, Result};
use blazar_event::{Button, Event, Key};
use blazar_vk_dl as vk_dl;
use blazar_winapi_sys as winapi_sys;
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

/// Represents an object that holds on to global resources.
pub(crate) struct Context {
    _vk: vk_dl::VulkanLibrary,
    instance: winapi_sys::HMODULE,
    class_name: Vec<winapi_sys::WCHAR>,
}

impl Context {
    fn create() -> Result<Context> {
        unsafe {
            // Loads Vulkan library.
            let _vk = vk_dl::VulkanLibrary::load().map_err(|_| {
                CreateWindowError::ContextCreationFailed(String::from("Cannot load Vulkan library"))
            })?;

            // Retrieves a module handle.
            let instance = winapi_sys::GetModuleHandleW(ptr::null());
            if instance.is_null() {
                return Err(CreateWindowError::ContextCreationFailed(String::from(
                    "Cannot retrieve a module handle",
                )));
            }

            // Registers the window class.
            let class_name = wide_string(env!("CARGO_PKG_NAME"));
            let mut class: winapi_sys::WNDCLASSW = mem::zeroed();
            class.style = winapi_sys::CS_HREDRAW | winapi_sys::CS_VREDRAW;
            class.lpfnWndProc = mem::transmute::<
                Option<
                    unsafe fn(
                        winapi_sys::HWND,
                        winapi_sys::UINT,
                        winapi_sys::WPARAM,
                        winapi_sys::LPARAM,
                    ) -> winapi_sys::LRESULT,
                >,
                winapi_sys::WNDPROC,
            >(Some(Window::handle_message));
            class.hInstance = instance;
            class.lpszClassName = class_name.as_ptr();
            if winapi_sys::RegisterClassW(&class) == 0 {
                return Err(CreateWindowError::ContextCreationFailed(String::from(
                    "Cannot register the window class",
                )));
            }

            Ok(Context {
                _vk,
                instance,
                class_name,
            })
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            winapi_sys::UnregisterClassW(self.class_name.as_ptr(), self.instance);
        }
    }
}

/// Represents a window.
pub struct Window {
    _context: Context,
    handle: winapi_sys::HWND,
    events: VecDeque<Event>,
}

impl Window {
    /// Creates a new window.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window> {
        unsafe {
            // Creates context.
            let _context = Context::create()?;

            // Creates the window.
            let mut rectangle = winapi_sys::RECT {
                left: 0,
                top: 0,
                right: width as winapi_sys::LONG,
                bottom: height as winapi_sys::LONG,
            };
            winapi_sys::AdjustWindowRect(
                &mut rectangle,
                winapi_sys::WS_OVERLAPPEDWINDOW | winapi_sys::WS_VISIBLE,
                winapi_sys::FALSE,
            );
            let width = rectangle.right - rectangle.left;
            let height = rectangle.bottom - rectangle.top;
            let title = wide_string(&title);
            let handle = winapi_sys::CreateWindowExW(
                0,
                _context.class_name.as_ptr(),
                title.as_ptr(),
                winapi_sys::WS_OVERLAPPEDWINDOW | winapi_sys::WS_VISIBLE,
                winapi_sys::CW_USEDEFAULT,
                winapi_sys::CW_USEDEFAULT,
                width as c_int,
                height as c_int,
                ptr::null_mut(),
                ptr::null_mut(),
                _context.instance,
                ptr::null_mut(),
            );
            if handle.is_null() {
                return Err(CreateWindowError::WindowCreationFailed);
            }

            // Creates event queue.
            let events = VecDeque::new();

            Ok(Window {
                _context,
                handle,
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
            let window = wide_string(env!("CARGO_PKG_NAME"));
            winapi_sys::SetPropW(
                self.handle,
                window.as_ptr(),
                self as *mut Window as *mut c_void,
            );
            let mut message: winapi_sys::MSG = mem::zeroed();
            while winapi_sys::PeekMessageW(&mut message, self.handle, 0, 0, winapi_sys::PM_REMOVE)
                > 0
            {
                winapi_sys::TranslateMessage(&message);
                winapi_sys::DispatchMessageW(&message);
            }
        }
    }

    /// Function that processes messages sent to the window.
    unsafe fn handle_message(
        handle: winapi_sys::HWND,
        message: winapi_sys::UINT,
        w_param: winapi_sys::WPARAM,
        l_param: winapi_sys::LPARAM,
    ) -> winapi_sys::LRESULT {
        let window = wide_string(env!("CARGO_PKG_NAME"));
        let window = winapi_sys::GetPropW(handle, window.as_ptr()) as *mut Window;
        if !window.is_null() {
            if let Some(event) = translate_event(message, w_param, l_param) {
                (*window).events.push_back(event);
                return 0;
            }
        }
        winapi_sys::DefWindowProcW(handle, message, w_param, l_param)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            winapi_sys::DestroyWindow(self.handle);
        }
    }
}

/// Translates an event message into `Option<Event>`.
unsafe fn translate_event(
    message: winapi_sys::UINT,
    w_param: winapi_sys::WPARAM,
    l_param: winapi_sys::LPARAM,
) -> Option<Event> {
    match message {
        // Window
        winapi_sys::WM_CLOSE => Some(Event::Close),
        winapi_sys::WM_SETFOCUS => Some(Event::GainFocus),
        winapi_sys::WM_KILLFOCUS => Some(Event::LoseFocus),
        winapi_sys::WM_SIZE => Some(Event::Resize {
            width: u32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            height: u32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        // Keyboard
        winapi_sys::WM_KEYDOWN | winapi_sys::WM_SYSKEYDOWN
            if winapi_sys::HIWORD(l_param as winapi_sys::DWORD) & winapi_sys::KF_REPEAT == 0 =>
        {
            translate_key(w_param, l_param).map(|key| Event::KeyPress { key })
        }
        winapi_sys::WM_KEYUP | winapi_sys::WM_SYSKEYUP => {
            translate_key(w_param, l_param).map(|key| Event::KeyRelease { key })
        }
        // Mouse
        winapi_sys::WM_MOUSEWHEEL => Some(if winapi_sys::GET_WHEEL_DELTA_WPARAM(w_param) > 0 {
            Event::MouseScrollUp
        } else {
            Event::MouseScrollDown
        }),
        winapi_sys::WM_MOUSEMOVE => Some(Event::MouseMove {
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_LBUTTONDOWN => Some(Event::MouseButtonPress {
            button: Button::Left,
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_LBUTTONUP => Some(Event::MouseButtonRelease {
            button: Button::Left,
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_MBUTTONDOWN => Some(Event::MouseButtonPress {
            button: Button::Middle,
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_MBUTTONUP => Some(Event::MouseButtonRelease {
            button: Button::Middle,
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_RBUTTONDOWN => Some(Event::MouseButtonPress {
            button: Button::Right,
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_RBUTTONUP => Some(Event::MouseButtonRelease {
            button: Button::Right,
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_XBUTTONDOWN => Some(Event::MouseButtonPress {
            button: if winapi_sys::GET_XBUTTON_WPARAM(w_param) == winapi_sys::XBUTTON1 {
                Button::Back
            } else {
                Button::Forward
            },
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        winapi_sys::WM_XBUTTONUP => Some(Event::MouseButtonRelease {
            button: if winapi_sys::GET_XBUTTON_WPARAM(w_param) == winapi_sys::XBUTTON1 {
                Button::Back
            } else {
                Button::Forward
            },
            x: i32::from(winapi_sys::LOWORD(l_param as winapi_sys::DWORD)),
            y: i32::from(winapi_sys::HIWORD(l_param as winapi_sys::DWORD)),
        }),
        // Unknown
        _ => None,
    }
}

/// Translates a Win32 key to `Option<Key>`
fn translate_key(key: winapi_sys::WPARAM, flags: winapi_sys::LPARAM) -> Option<Key> {
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
        winapi_sys::VK_OEM_3 => Key::Backquote,
        winapi_sys::VK_OEM_MINUS => Key::Minus,
        winapi_sys::VK_OEM_PLUS => Key::Equal,
        winapi_sys::VK_OEM_4 => Key::LeftBracket,
        winapi_sys::VK_OEM_6 => Key::RightBracket,
        winapi_sys::VK_OEM_5 => Key::Backslash,
        winapi_sys::VK_OEM_1 => Key::Semicolon,
        winapi_sys::VK_OEM_7 => Key::Quote,
        winapi_sys::VK_OEM_COMMA => Key::Comma,
        winapi_sys::VK_OEM_PERIOD => Key::Period,
        winapi_sys::VK_OEM_2 => Key::Slash,
        winapi_sys::VK_TAB => Key::Tab,
        winapi_sys::VK_CAPITAL => Key::CapsLock,
        winapi_sys::VK_SHIFT => unsafe {
            let left_shift = winapi_sys::MapVirtualKeyW(
                winapi_sys::VK_LSHIFT as winapi_sys::UINT,
                winapi_sys::MAPVK_VK_TO_VSC,
            );
            let code = ((flags & (0xFF << 16)) >> 16) as winapi_sys::UINT;
            if code == left_shift {
                Key::LeftShift
            } else {
                Key::RightShift
            }
        },
        winapi_sys::VK_BACK => Key::Backspace,
        winapi_sys::VK_RETURN => {
            if is_extended_key_flag(flags) {
                Key::NumpadEnter
            } else {
                Key::Enter
            }
        }
        winapi_sys::VK_SPACE => Key::Space,
        // Control
        winapi_sys::VK_ESCAPE => Key::Escape,
        winapi_sys::VK_PRINT => Key::PrintScreen,
        winapi_sys::VK_SCROLL => Key::ScrollLock,
        winapi_sys::VK_PAUSE => Key::Pause,
        winapi_sys::VK_CONTROL => {
            if is_extended_key_flag(flags) {
                Key::RightControl
            } else {
                Key::LeftControl
            }
        }
        winapi_sys::VK_MENU => {
            if is_extended_key_flag(flags) {
                Key::RightAlt
            } else {
                Key::LeftAlt
            }
        }
        winapi_sys::VK_LWIN => Key::LeftSuper,
        winapi_sys::VK_RWIN => Key::RightSuper,
        winapi_sys::VK_APPS => Key::Menu,
        // Function
        winapi_sys::VK_F1 => Key::F1,
        winapi_sys::VK_F2 => Key::F2,
        winapi_sys::VK_F3 => Key::F3,
        winapi_sys::VK_F4 => Key::F4,
        winapi_sys::VK_F5 => Key::F5,
        winapi_sys::VK_F6 => Key::F6,
        winapi_sys::VK_F7 => Key::F7,
        winapi_sys::VK_F8 => Key::F8,
        winapi_sys::VK_F9 => Key::F9,
        winapi_sys::VK_F10 => Key::F10,
        winapi_sys::VK_F11 => Key::F11,
        winapi_sys::VK_F12 => Key::F12,
        // Navigation
        winapi_sys::VK_INSERT => Key::Insert,
        winapi_sys::VK_DELETE => Key::Delete,
        winapi_sys::VK_HOME => Key::Home,
        winapi_sys::VK_END => Key::End,
        winapi_sys::VK_PRIOR => Key::PageUp,
        winapi_sys::VK_NEXT => Key::PageDown,
        winapi_sys::VK_UP => Key::UpArrow,
        winapi_sys::VK_DOWN => Key::DownArrow,
        winapi_sys::VK_LEFT => Key::LeftArrow,
        winapi_sys::VK_RIGHT => Key::RightArrow,
        // Numeric keypad
        winapi_sys::VK_NUMLOCK => Key::NumLock,
        winapi_sys::VK_NUMPAD0 => Key::Numpad0,
        winapi_sys::VK_NUMPAD1 => Key::Numpad1,
        winapi_sys::VK_NUMPAD2 => Key::Numpad2,
        winapi_sys::VK_NUMPAD3 => Key::Numpad3,
        winapi_sys::VK_NUMPAD4 => Key::Numpad4,
        winapi_sys::VK_NUMPAD5 => Key::Numpad5,
        winapi_sys::VK_NUMPAD6 => Key::Numpad6,
        winapi_sys::VK_NUMPAD7 => Key::Numpad7,
        winapi_sys::VK_NUMPAD8 => Key::Numpad8,
        winapi_sys::VK_NUMPAD9 => Key::Numpad9,
        winapi_sys::VK_DIVIDE => Key::NumpadDivide,
        winapi_sys::VK_MULTIPLY => Key::NumpadMultiply,
        winapi_sys::VK_SUBTRACT => Key::NumpadSubtract,
        winapi_sys::VK_ADD => Key::NumpadAdd,
        winapi_sys::VK_DECIMAL => Key::NumpadDecimal,
        // Unknown
        _ => return None,
    })
}

/// Converts an `&str` into wide characters.
#[inline]
fn wide_string(string: &str) -> Vec<winapi_sys::WCHAR> {
    OsStr::new(string)
        .encode_wide()
        .chain(iter::once(0))
        .collect()
}

/// Indicates if the extended-key flag is included in the specified flags.
#[inline]
fn is_extended_key_flag(flags: winapi_sys::LPARAM) -> bool {
    winapi_sys::HIWORD(flags as winapi_sys::DWORD) & winapi_sys::KF_EXTENDED != 0
}
