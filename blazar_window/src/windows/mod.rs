//! Win32-based windows.

use crate::{Result, WindowError};
use blazar_event::{Button, Event, Key};
use blazar_vk_dl as vk_dl;
use blazar_winapi as winapi;
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
    instance: winapi::HMODULE,
    class_name: Vec<winapi::WCHAR>,
}

impl Context {
    fn create() -> Result<Context> {
        unsafe {
            // Loads Vulkan library.
            let _vk = vk_dl::VulkanLibrary::load().map_err(|_| {
                WindowError::ContextCreation(String::from("Cannot load Vulkan library"))
            })?;

            // Retrieves a module handle.
            let instance = winapi::GetModuleHandleW(ptr::null());
            if instance.is_null() {
                return Err(WindowError::ContextCreation(String::from(
                    "Cannot retrieve a module handle",
                )));
            }

            // Registers the window class.
            let class_name = wide_string(env!("CARGO_PKG_NAME"));
            let mut class: winapi::WNDCLASSW = mem::zeroed();
            class.style = winapi::CS_HREDRAW | winapi::CS_VREDRAW;
            class.lpfnWndProc = mem::transmute::<
                Option<
                    unsafe fn(
                        winapi::HWND,
                        winapi::UINT,
                        winapi::WPARAM,
                        winapi::LPARAM,
                    ) -> winapi::LRESULT,
                >,
                winapi::WNDPROC,
            >(Some(Window::handle_message));
            class.hInstance = instance;
            class.lpszClassName = class_name.as_ptr();
            if winapi::RegisterClassW(&class) == 0 {
                return Err(WindowError::ContextCreation(String::from(
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
            winapi::UnregisterClassW(self.class_name.as_ptr(), self.instance);
        }
    }
}

/// Represents a window.
pub struct Window {
    _context: Context,
    handle: winapi::HWND,
    events: VecDeque<Event>,
}

impl Window {
    /// Creates a new window.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window> {
        unsafe {
            // Creates context.
            let _context = Context::create()?;

            // Creates the window.
            let mut rectangle = winapi::RECT {
                left: 0,
                top: 0,
                right: width as winapi::LONG,
                bottom: height as winapi::LONG,
            };
            winapi::AdjustWindowRect(
                &mut rectangle,
                winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE,
                winapi::FALSE,
            );
            let width = rectangle.right - rectangle.left;
            let height = rectangle.bottom - rectangle.top;
            let title = wide_string(&title);
            let handle = winapi::CreateWindowExW(
                0,
                _context.class_name.as_ptr(),
                title.as_ptr(),
                winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE,
                winapi::CW_USEDEFAULT,
                winapi::CW_USEDEFAULT,
                width as c_int,
                height as c_int,
                ptr::null_mut(),
                ptr::null_mut(),
                _context.instance,
                ptr::null_mut(),
            );
            if handle.is_null() {
                return Err(WindowError::WindowCreation);
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
            let window = wide_string("window");
            winapi::SetPropW(
                self.handle,
                window.as_ptr(),
                self as *mut Window as *mut c_void,
            );
            let mut message: winapi::MSG = mem::zeroed();
            while winapi::PeekMessageW(&mut message, self.handle, 0, 0, winapi::PM_REMOVE) > 0 {
                winapi::TranslateMessage(&message);
                winapi::DispatchMessageW(&message);
            }
        }
    }

    /// Function that processes messages sent to the window.
    unsafe fn handle_message(
        handle: winapi::HWND,
        message: winapi::UINT,
        w_param: winapi::WPARAM,
        l_param: winapi::LPARAM,
    ) -> winapi::LRESULT {
        let window = wide_string("window");
        let window = winapi::GetPropW(handle, window.as_ptr()) as *mut Window;
        if !window.is_null() {
            if let Some(event) = translate_event(message, w_param, l_param) {
                (*window).events.push_back(event);
                return 0;
            }
        }
        winapi::DefWindowProcW(handle, message, w_param, l_param)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            winapi::DestroyWindow(self.handle);
        }
    }
}

/// Translates an event message into `Option<Event>`.
unsafe fn translate_event(
    message: winapi::UINT,
    w_param: winapi::WPARAM,
    l_param: winapi::LPARAM,
) -> Option<Event> {
    match message {
        // Window
        winapi::WM_CLOSE => Some(Event::Close),
        winapi::WM_SETFOCUS => Some(Event::GainFocus),
        winapi::WM_KILLFOCUS => Some(Event::LoseFocus),
        winapi::WM_SIZE => Some(Event::Resize {
            width: u32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            height: u32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        // Keyboard
        winapi::WM_KEYDOWN | winapi::WM_SYSKEYDOWN
            if winapi::HIWORD(l_param as winapi::DWORD) & winapi::KF_REPEAT == 0 =>
        {
            translate_key(w_param, l_param).map(|key| Event::KeyPress { key })
        }
        winapi::WM_KEYUP | winapi::WM_SYSKEYUP => {
            translate_key(w_param, l_param).map(|key| Event::KeyRelease { key })
        }
        // Mouse
        winapi::WM_MOUSEWHEEL => Some(if winapi::GET_WHEEL_DELTA_WPARAM(w_param) > 0 {
            Event::MouseScrollUp
        } else {
            Event::MouseScrollDown
        }),
        winapi::WM_MOUSEMOVE => Some(Event::MouseMove {
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_LBUTTONDOWN => Some(Event::MouseButtonPress {
            button: Button::Left,
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_LBUTTONUP => Some(Event::MouseButtonRelease {
            button: Button::Left,
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_MBUTTONDOWN => Some(Event::MouseButtonPress {
            button: Button::Middle,
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_MBUTTONUP => Some(Event::MouseButtonRelease {
            button: Button::Middle,
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_RBUTTONDOWN => Some(Event::MouseButtonPress {
            button: Button::Right,
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_RBUTTONUP => Some(Event::MouseButtonRelease {
            button: Button::Right,
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_XBUTTONDOWN => Some(Event::MouseButtonPress {
            button: if winapi::GET_XBUTTON_WPARAM(w_param) == winapi::XBUTTON1 {
                Button::Back
            } else {
                Button::Forward
            },
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        winapi::WM_XBUTTONUP => Some(Event::MouseButtonRelease {
            button: if winapi::GET_XBUTTON_WPARAM(w_param) == winapi::XBUTTON1 {
                Button::Back
            } else {
                Button::Forward
            },
            x: i32::from(winapi::LOWORD(l_param as winapi::DWORD)),
            y: i32::from(winapi::HIWORD(l_param as winapi::DWORD)),
        }),
        // Unknown
        _ => None,
    }
}

/// Translates a Win32 key to `Option<Key>`
fn translate_key(key: winapi::WPARAM, flags: winapi::LPARAM) -> Option<Key> {
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
        winapi::VK_OEM_3 => Key::Backquote,
        winapi::VK_OEM_MINUS => Key::Minus,
        winapi::VK_OEM_PLUS => Key::Equal,
        winapi::VK_OEM_4 => Key::LeftBracket,
        winapi::VK_OEM_6 => Key::RightBracket,
        winapi::VK_OEM_5 => Key::Backslash,
        winapi::VK_OEM_1 => Key::Semicolon,
        winapi::VK_OEM_7 => Key::Quote,
        winapi::VK_OEM_COMMA => Key::Comma,
        winapi::VK_OEM_PERIOD => Key::Period,
        winapi::VK_OEM_2 => Key::Slash,
        winapi::VK_TAB => Key::Tab,
        winapi::VK_CAPITAL => Key::CapsLock,
        winapi::VK_SHIFT => unsafe {
            let left_shift =
                winapi::MapVirtualKeyW(winapi::VK_LSHIFT as winapi::UINT, winapi::MAPVK_VK_TO_VSC);
            let code = ((flags & (0xFF << 16)) >> 16) as winapi::UINT;
            if code == left_shift {
                Key::LeftShift
            } else {
                Key::RightShift
            }
        },
        winapi::VK_BACK => Key::Backspace,
        winapi::VK_RETURN => {
            if is_extended_key_flag(flags) {
                Key::NumpadEnter
            } else {
                Key::Enter
            }
        }
        winapi::VK_SPACE => Key::Space,
        // Control
        winapi::VK_ESCAPE => Key::Escape,
        winapi::VK_PRINT => Key::PrintScreen,
        winapi::VK_SCROLL => Key::ScrollLock,
        winapi::VK_PAUSE => Key::Pause,
        winapi::VK_CONTROL => {
            if is_extended_key_flag(flags) {
                Key::RightControl
            } else {
                Key::LeftControl
            }
        }
        winapi::VK_MENU => {
            if is_extended_key_flag(flags) {
                Key::RightAlt
            } else {
                Key::LeftAlt
            }
        }
        winapi::VK_LWIN => Key::LeftSuper,
        winapi::VK_RWIN => Key::RightSuper,
        winapi::VK_APPS => Key::Menu,
        // Function
        winapi::VK_F1 => Key::F1,
        winapi::VK_F2 => Key::F2,
        winapi::VK_F3 => Key::F3,
        winapi::VK_F4 => Key::F4,
        winapi::VK_F5 => Key::F5,
        winapi::VK_F6 => Key::F6,
        winapi::VK_F7 => Key::F7,
        winapi::VK_F8 => Key::F8,
        winapi::VK_F9 => Key::F9,
        winapi::VK_F10 => Key::F10,
        winapi::VK_F11 => Key::F11,
        winapi::VK_F12 => Key::F12,
        // Navigation
        winapi::VK_INSERT => Key::Insert,
        winapi::VK_DELETE => Key::Delete,
        winapi::VK_HOME => Key::Home,
        winapi::VK_END => Key::End,
        winapi::VK_PRIOR => Key::PageUp,
        winapi::VK_NEXT => Key::PageDown,
        winapi::VK_UP => Key::UpArrow,
        winapi::VK_DOWN => Key::DownArrow,
        winapi::VK_LEFT => Key::LeftArrow,
        winapi::VK_RIGHT => Key::RightArrow,
        // Numeric keypad
        winapi::VK_NUMLOCK => Key::NumLock,
        winapi::VK_NUMPAD0 => Key::Numpad0,
        winapi::VK_NUMPAD1 => Key::Numpad1,
        winapi::VK_NUMPAD2 => Key::Numpad2,
        winapi::VK_NUMPAD3 => Key::Numpad3,
        winapi::VK_NUMPAD4 => Key::Numpad4,
        winapi::VK_NUMPAD5 => Key::Numpad5,
        winapi::VK_NUMPAD6 => Key::Numpad6,
        winapi::VK_NUMPAD7 => Key::Numpad7,
        winapi::VK_NUMPAD8 => Key::Numpad8,
        winapi::VK_NUMPAD9 => Key::Numpad9,
        winapi::VK_DIVIDE => Key::NumpadDivide,
        winapi::VK_MULTIPLY => Key::NumpadMultiply,
        winapi::VK_SUBTRACT => Key::NumpadSubtract,
        winapi::VK_ADD => Key::NumpadAdd,
        winapi::VK_DECIMAL => Key::NumpadDecimal,
        // Unknown
        _ => return None,
    })
}

/// Converts an `&str` into wide characters.
#[inline]
fn wide_string(string: &str) -> Vec<winapi::WCHAR> {
    OsStr::new(string)
        .encode_wide()
        .chain(iter::once(0))
        .collect()
}

/// Indicates if the extended-key flag is included in the specified flags.
#[inline]
fn is_extended_key_flag(flags: winapi::LPARAM) -> bool {
    winapi::HIWORD(flags as winapi::DWORD) & winapi::KF_EXTENDED != 0
}
