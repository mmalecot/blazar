//! Xlib-based windows.

use crate::{CreateWindowError, Result};
use blazar_event::{Button, Event, Key};
use blazar_vk_dl as vk_dl;
use blazar_xlib_dl as xlib_dl;
use blazar_xlib_sys as xlib_sys;
use std::{
    collections::VecDeque,
    ffi::CString,
    mem,
    os::raw::{c_int, c_uchar, c_uint},
    ptr,
};

/// Represents an object that holds on to global resources.
pub(crate) struct Context {
    pub(crate) x11: xlib_dl::X11Library,
    pub(crate) _vk: vk_dl::VulkanLibrary,
    pub(crate) display: *mut xlib_sys::Display,
    pub(crate) wm_protocols: xlib_sys::Atom,
    pub(crate) wm_delete_window: xlib_sys::Atom,
    pub(crate) utf8_string: xlib_sys::Atom,
    pub(crate) net_wm_name: xlib_sys::Atom,
    pub(crate) net_wm_icon_name: xlib_sys::Atom,
}

impl Context {
    fn create() -> Result<Context> {
        unsafe {
            // Loads Xlib.
            let x11 = xlib_dl::X11Library::load().map_err(|_| {
                CreateWindowError::ContextCreationFailed(String::from("Cannot load Xlib"))
            })?;

            // Loads Vulkan library.
            let _vk = vk_dl::VulkanLibrary::load().map_err(|_| {
                CreateWindowError::ContextCreationFailed(String::from("Cannot load Vulkan library"))
            })?;

            // Opens X display.
            let display = x11.XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err(CreateWindowError::ContextCreationFailed(String::from(
                    "Cannot open X display",
                )));
            }

            // Loads X atoms.
            let wm_protocols = CString::new("WM_PROTOCOLS").unwrap();
            let wm_protocols = x11.XInternAtom(display, wm_protocols.as_ptr(), xlib_sys::FALSE);
            let wm_delete_window = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_delete_window =
                x11.XInternAtom(display, wm_delete_window.as_ptr(), xlib_sys::FALSE);
            let utf8_string = CString::new("UTF8_STRING").unwrap();
            let utf8_string = x11.XInternAtom(display, utf8_string.as_ptr(), xlib_sys::FALSE);
            let net_wm_name = CString::new("_NET_WM_NAME").unwrap();
            let net_wm_name = x11.XInternAtom(display, net_wm_name.as_ptr(), xlib_sys::FALSE);
            let net_wm_icon_name = CString::new("_NET_WM_ICON_NAME").unwrap();
            let net_wm_icon_name =
                x11.XInternAtom(display, net_wm_icon_name.as_ptr(), xlib_sys::FALSE);

            Ok(Context {
                x11,
                _vk,
                display,
                wm_protocols,
                wm_delete_window,
                utf8_string,
                net_wm_name,
                net_wm_icon_name,
            })
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            self.x11.XCloseDisplay(self.display);
        }
    }
}

/// Represents a window.
pub struct Window {
    context: Context,
    handle: xlib_sys::Window,
    width: u32,
    height: u32,
    events: VecDeque<Event>,
}

impl Window {
    /// Creates a new window.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window> {
        unsafe {
            // Creates context.
            let context = Context::create()?;

            // Creates the window.
            let default_screen = context.x11.XDefaultScreen(context.display);
            let handle = context.x11.XCreateSimpleWindow(
                context.display,
                context.x11.XDefaultRootWindow(context.display),
                0,
                0,
                width,
                height,
                1,
                context.x11.XBlackPixel(context.display, default_screen),
                context.x11.XBlackPixel(context.display, default_screen),
            );
            if handle == 0 {
                return Err(CreateWindowError::WindowCreationFailed);
            }

            // Selects input events.
            context.x11.XSelectInput(
                context.display,
                handle,
                xlib_sys::ButtonPressMask
                    | xlib_sys::ButtonReleaseMask
                    | xlib_sys::ExposureMask
                    | xlib_sys::FocusChangeMask
                    | xlib_sys::PointerMotionMask
                    | xlib_sys::KeyPressMask
                    | xlib_sys::KeyReleaseMask
                    | xlib_sys::StructureNotifyMask,
            );

            // Sets window's title.
            let title = CString::new(title).unwrap();
            context.x11.Xutf8SetWMProperties(
                context.display,
                handle,
                title.as_ptr(),
                title.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            context.x11.XChangeProperty(
                context.display,
                handle,
                context.net_wm_name,
                context.utf8_string,
                8,
                xlib_sys::PropModeReplace,
                title.as_ptr() as *const c_uchar,
                title.as_bytes().len() as c_int,
            );
            context.x11.XChangeProperty(
                context.display,
                handle,
                context.net_wm_icon_name,
                context.utf8_string,
                8,
                xlib_sys::PropModeReplace,
                title.as_ptr() as *const c_uchar,
                title.as_bytes().len() as c_int,
            );
            context.x11.XFlush(context.display);

            // Handles WM delete window (close button).
            let mut protocols = [context.wm_delete_window];
            context.x11.XSetWMProtocols(
                context.display,
                handle,
                protocols.as_mut_ptr(),
                protocols.len() as c_int,
            );

            // Maps window to the root window.
            context.x11.XMapWindow(context.display, handle);

            // Creates event queue.
            let events = VecDeque::new();

            Ok(Window {
                context,
                handle,
                width,
                height,
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
            while self.context.x11.XPending(self.context.display) > 0 {
                let mut event: xlib_sys::XEvent = mem::zeroed();
                self.context
                    .x11
                    .XNextEvent(self.context.display, &mut event);
                if let Some(event) = self.translate_event(&mut event) {
                    self.events.push_back(event)
                }
            }
        }
    }

    /// Translates `XEvent` into `Option<Event>`.
    unsafe fn translate_event(&mut self, event: &mut xlib_sys::XEvent) -> Option<Event> {
        match event.r#type {
            // Window
            xlib_sys::ClientMessage
                if event.client_message.message_type == self.context.wm_protocols
                    && event.client_message.data.longs[0] as xlib_sys::Atom
                        == self.context.wm_delete_window =>
            {
                Some(Event::Close)
            }
            xlib_sys::FocusIn => Some(Event::GainFocus),
            xlib_sys::FocusOut => Some(Event::LoseFocus),
            xlib_sys::ConfigureNotify
                if event.configure.width as u32 != self.width
                    || event.configure.height as u32 != self.height =>
            {
                self.width = event.configure.width as u32;
                self.height = event.configure.width as u32;
                Some(Event::Resize {
                    width: self.width,
                    height: self.height,
                })
            }
            // Keyboard
            xlib_sys::KeyPress => self
                .translate_key_event(&mut event.key)
                .map(|key| Event::KeyPress { key }),
            xlib_sys::KeyRelease => {
                // Ignores auto-repeat.
                if self.context.x11.XPending(self.context.display) > 0 {
                    let mut next_event: xlib_sys::XEvent = mem::zeroed();
                    self.context
                        .x11
                        .XPeekEvent(self.context.display, &mut next_event);
                    if next_event.r#type == xlib_sys::KeyPress
                        && next_event.key.keycode == event.key.keycode
                        && next_event.key.time - event.key.time < 20
                    {
                        self.context
                            .x11
                            .XNextEvent(self.context.display, &mut next_event);
                        return None;
                    }
                }
                self.translate_key_event(&mut event.key)
                    .map(|key| Event::KeyRelease { key })
            }
            // Mouse
            xlib_sys::ButtonPress => match event.button.button {
                xlib_sys::Button4 => Some(Event::MouseScrollUp),
                xlib_sys::Button5 => Some(Event::MouseScrollDown),
                _ => translate_button(event.button.button).map(|button| Event::MouseButtonPress {
                    button,
                    x: event.button.x,
                    y: event.button.y,
                }),
            },
            xlib_sys::ButtonRelease => {
                translate_button(event.button.button).map(|button| Event::MouseButtonRelease {
                    button,
                    x: event.button.x,
                    y: event.button.y,
                })
            }
            xlib_sys::MotionNotify => Some(Event::MouseMove {
                x: event.button.x,
                y: event.button.y,
            }),
            // Unknown
            _ => None,
        }
    }

    /// Translates an `XKeyEvent` into `Option<Key>`.
    unsafe fn translate_key_event(&mut self, event: &mut xlib_sys::XKeyEvent) -> Option<Key> {
        for index in 0..4 {
            if let Some(key) = translate_key(self.context.x11.XLookupKeysym(event, index)) {
                return Some(key);
            }
        }
        None
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            self.context
                .x11
                .XDestroyWindow(self.context.display, self.handle);
        }
    }
}

/// Translates a X11 key to `Option<Key>`.
fn translate_key(symbol: xlib_sys::KeySym) -> Option<Key> {
    Some(match symbol {
        // Typing
        xlib_sys::XK_A => Key::A,
        xlib_sys::XK_B => Key::B,
        xlib_sys::XK_C => Key::C,
        xlib_sys::XK_D => Key::D,
        xlib_sys::XK_E => Key::E,
        xlib_sys::XK_F => Key::F,
        xlib_sys::XK_G => Key::G,
        xlib_sys::XK_H => Key::H,
        xlib_sys::XK_I => Key::I,
        xlib_sys::XK_J => Key::J,
        xlib_sys::XK_K => Key::K,
        xlib_sys::XK_L => Key::L,
        xlib_sys::XK_M => Key::M,
        xlib_sys::XK_N => Key::N,
        xlib_sys::XK_O => Key::O,
        xlib_sys::XK_P => Key::P,
        xlib_sys::XK_Q => Key::Q,
        xlib_sys::XK_R => Key::R,
        xlib_sys::XK_S => Key::S,
        xlib_sys::XK_T => Key::T,
        xlib_sys::XK_U => Key::U,
        xlib_sys::XK_V => Key::V,
        xlib_sys::XK_W => Key::W,
        xlib_sys::XK_X => Key::X,
        xlib_sys::XK_Y => Key::Y,
        xlib_sys::XK_Z => Key::Z,
        xlib_sys::XK_0 => Key::Digit0,
        xlib_sys::XK_1 => Key::Digit1,
        xlib_sys::XK_2 => Key::Digit2,
        xlib_sys::XK_3 => Key::Digit3,
        xlib_sys::XK_4 => Key::Digit4,
        xlib_sys::XK_5 => Key::Digit5,
        xlib_sys::XK_6 => Key::Digit6,
        xlib_sys::XK_7 => Key::Digit7,
        xlib_sys::XK_8 => Key::Digit8,
        xlib_sys::XK_9 => Key::Digit9,
        xlib_sys::XK_grave => Key::Backquote,
        xlib_sys::XK_minus => Key::Minus,
        xlib_sys::XK_equal => Key::Equal,
        xlib_sys::XK_bracketleft => Key::LeftBracket,
        xlib_sys::XK_bracketright => Key::RightBracket,
        xlib_sys::XK_backslash => Key::Backslash,
        xlib_sys::XK_semicolon => Key::Semicolon,
        xlib_sys::XK_apostrophe => Key::Quote,
        xlib_sys::XK_comma => Key::Comma,
        xlib_sys::XK_period => Key::Period,
        xlib_sys::XK_slash => Key::Slash,
        xlib_sys::XK_Tab => Key::Tab,
        xlib_sys::XK_Caps_Lock => Key::CapsLock,
        xlib_sys::XK_Shift_L => Key::LeftShift,
        xlib_sys::XK_BackSpace => Key::Backspace,
        xlib_sys::XK_Return => Key::Enter,
        xlib_sys::XK_Shift_R => Key::RightShift,
        xlib_sys::XK_space => Key::Space,
        // Control
        xlib_sys::XK_Escape => Key::Escape,
        xlib_sys::XK_Print => Key::PrintScreen,
        xlib_sys::XK_Scroll_Lock => Key::ScrollLock,
        xlib_sys::XK_Break => Key::Pause,
        xlib_sys::XK_Control_L => Key::LeftControl,
        xlib_sys::XK_Super_L => Key::LeftSuper,
        xlib_sys::XK_Alt_L => Key::LeftAlt,
        xlib_sys::XK_Alt_R => Key::RightAlt,
        xlib_sys::XK_Super_R => Key::RightSuper,
        xlib_sys::XK_Menu => Key::Menu,
        xlib_sys::XK_Control_R => Key::RightControl,
        // Function
        xlib_sys::XK_F1 => Key::F1,
        xlib_sys::XK_F2 => Key::F2,
        xlib_sys::XK_F3 => Key::F3,
        xlib_sys::XK_F4 => Key::F4,
        xlib_sys::XK_F5 => Key::F5,
        xlib_sys::XK_F6 => Key::F6,
        xlib_sys::XK_F7 => Key::F7,
        xlib_sys::XK_F8 => Key::F8,
        xlib_sys::XK_F9 => Key::F9,
        xlib_sys::XK_F10 => Key::F10,
        xlib_sys::XK_F11 => Key::F11,
        xlib_sys::XK_F12 => Key::F12,
        // Navigation
        xlib_sys::XK_Insert => Key::Insert,
        xlib_sys::XK_Delete => Key::Delete,
        xlib_sys::XK_Home => Key::Home,
        xlib_sys::XK_End => Key::End,
        xlib_sys::XK_Prior => Key::PageUp,
        xlib_sys::XK_Next => Key::PageDown,
        xlib_sys::XK_Up => Key::UpArrow,
        xlib_sys::XK_Down => Key::DownArrow,
        xlib_sys::XK_Left => Key::LeftArrow,
        xlib_sys::XK_Right => Key::RightArrow,
        // Numeric keypad
        xlib_sys::XK_Num_Lock => Key::NumLock,
        xlib_sys::XK_KP_0 => Key::Numpad0,
        xlib_sys::XK_KP_1 => Key::Numpad1,
        xlib_sys::XK_KP_2 => Key::Numpad2,
        xlib_sys::XK_KP_3 => Key::Numpad3,
        xlib_sys::XK_KP_4 => Key::Numpad4,
        xlib_sys::XK_KP_5 => Key::Numpad5,
        xlib_sys::XK_KP_6 => Key::Numpad6,
        xlib_sys::XK_KP_7 => Key::Numpad7,
        xlib_sys::XK_KP_8 => Key::Numpad8,
        xlib_sys::XK_KP_9 => Key::Numpad9,
        xlib_sys::XK_KP_Enter => Key::NumpadEnter,
        xlib_sys::XK_KP_Divide => Key::NumpadDivide,
        xlib_sys::XK_KP_Multiply => Key::NumpadMultiply,
        xlib_sys::XK_KP_Subtract => Key::NumpadSubtract,
        xlib_sys::XK_KP_Add => Key::NumpadAdd,
        xlib_sys::XK_KP_Decimal => Key::NumpadDecimal,
        // Unknown
        _ => return None,
    })
}

/// Translates a X11 button to `Option<Button>`.
fn translate_button(button: c_uint) -> Option<Button> {
    Some(match button {
        // Common
        xlib_sys::Button1 => Button::Left,
        xlib_sys::Button2 => Button::Middle,
        xlib_sys::Button3 => Button::Right,
        // Extra
        8 => Button::Back,
        9 => Button::Forward,
        // Unknown
        _ => return None,
    })
}
