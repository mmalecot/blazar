//! Xlib-based windows.

use crate::{Result, WindowError};
use blazar_event::{Button, Event, Key};
use blazar_x11 as x11;
use std::{
    collections::VecDeque,
    ffi::CString,
    mem,
    os::raw::{c_int, c_uchar, c_uint},
    ptr,
};

/// Represents a window.
pub struct Window {
    x11: x11::X11Library,
    display: *mut x11::Display,
    handle: x11::Window,
    wm_protocols: x11::Atom,
    wm_delete_window: x11::Atom,
    width: u32,
    height: u32,
    events: VecDeque<Event>,
}

impl Window {
    /// Creates a new window.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window> {
        unsafe {
            // Loads Xlib.
            let x11 = x11::X11Library::load().map_err(|_| WindowError::CreateWindowError)?;

            // Opens X Display.
            let display = x11.XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err(WindowError::CreateWindowError);
            }

            // Creates the window.
            let default_screen = x11.XDefaultScreen(display);
            let handle = x11.XCreateSimpleWindow(
                display,
                x11.XDefaultRootWindow(display),
                0,
                0,
                width,
                height,
                1,
                x11.XBlackPixel(display, default_screen),
                x11.XBlackPixel(display, default_screen),
            );
            if handle == 0 {
                x11.XCloseDisplay(display);
                return Err(WindowError::CreateWindowError);
            }

            // Selects input events.
            x11.XSelectInput(
                display,
                handle,
                x11::ButtonPressMask
                    | x11::ButtonReleaseMask
                    | x11::ExposureMask
                    | x11::FocusChangeMask
                    | x11::PointerMotionMask
                    | x11::KeyPressMask
                    | x11::KeyReleaseMask
                    | x11::StructureNotifyMask,
            );

            // Sets window's title.
            let utf8_string = CString::new("UTF8_STRING").unwrap();
            let utf8_string = x11.XInternAtom(display, utf8_string.as_ptr(), x11::FALSE);
            let net_wm_name = CString::new("_NET_WM_NAME").unwrap();
            let net_wm_name = x11.XInternAtom(display, net_wm_name.as_ptr(), x11::FALSE);
            let net_wm_icon_name = CString::new("_NET_WM_ICON_NAME").unwrap();
            let net_wm_icon_name = x11.XInternAtom(display, net_wm_icon_name.as_ptr(), x11::FALSE);
            let title = CString::new(title).unwrap();
            x11.Xutf8SetWMProperties(
                display,
                handle,
                title.as_ptr(),
                title.as_ptr(),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            );
            x11.XChangeProperty(
                display,
                handle,
                net_wm_name,
                utf8_string,
                8,
                x11::PropModeReplace,
                title.as_ptr() as *const c_uchar,
                title.as_bytes().len() as c_int,
            );
            x11.XChangeProperty(
                display,
                handle,
                net_wm_icon_name,
                utf8_string,
                8,
                x11::PropModeReplace,
                title.as_ptr() as *const c_uchar,
                title.as_bytes().len() as c_int,
            );
            x11.XFlush(display);

            // Handles WM delete window (close button).
            let wm_protocols = CString::new("WM_PROTOCOLS").unwrap();
            let wm_protocols = x11.XInternAtom(display, wm_protocols.as_ptr(), x11::FALSE);
            let wm_delete_window = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_delete_window = x11.XInternAtom(display, wm_delete_window.as_ptr(), x11::FALSE);
            let mut protocols = [wm_delete_window];
            x11.XSetWMProtocols(
                display,
                handle,
                protocols.as_mut_ptr(),
                protocols.len() as c_int,
            );

            // Maps window to the root window.
            x11.XMapWindow(display, handle);

            // Creates event queue.
            let events = VecDeque::new();

            Ok(Window {
                x11,
                display,
                handle,
                wm_protocols,
                wm_delete_window,
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
            while self.x11.XPending(self.display) > 0 {
                let mut event: x11::XEvent = mem::zeroed();
                self.x11.XNextEvent(self.display, &mut event);
                if let Some(event) = self.translate_event(&mut event) {
                    self.events.push_back(event)
                }
            }
        }
    }

    /// Translates `XEvent` into `Option<Event>`.
    unsafe fn translate_event(&mut self, event: &mut x11::XEvent) -> Option<Event> {
        match event.r#type {
            // Window
            x11::ClientMessage
                if event.client_message.message_type == self.wm_protocols
                    && event.client_message.data.longs[0] as x11::Atom == self.wm_delete_window =>
            {
                Some(Event::Close)
            }
            x11::FocusIn => Some(Event::GainFocus),
            x11::FocusOut => Some(Event::LoseFocus),
            x11::ConfigureNotify
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
            x11::KeyPress => self
                .translate_key_event(&mut event.key)
                .map(|key| Event::KeyPress { key }),
            x11::KeyRelease => {
                // Ignores auto-repeat.
                if self.x11.XPending(self.display) > 0 {
                    let mut next_event: x11::XEvent = mem::zeroed();
                    self.x11.XPeekEvent(self.display, &mut next_event);
                    if next_event.r#type == x11::KeyPress
                        && next_event.key.keycode == event.key.keycode
                        && next_event.key.time - event.key.time < 20
                    {
                        self.x11.XNextEvent(self.display, &mut next_event);
                        return None;
                    }
                }
                self.translate_key_event(&mut event.key)
                    .map(|key| Event::KeyRelease { key })
            }
            // Mouse
            x11::ButtonPress => match event.button.button {
                x11::Button4 => Some(Event::MouseScrollUp),
                x11::Button5 => Some(Event::MouseScrollDown),
                _ => translate_button(event.button.button).map(|button| Event::MouseButtonPress {
                    button,
                    x: event.button.x,
                    y: event.button.y,
                }),
            },
            x11::ButtonRelease => {
                translate_button(event.button.button).map(|button| Event::MouseButtonRelease {
                    button,
                    x: event.button.x,
                    y: event.button.y,
                })
            }
            x11::MotionNotify => Some(Event::MouseMove {
                x: event.button.x,
                y: event.button.y,
            }),
            // Unknown
            _ => None,
        }
    }

    /// Translates an `XKeyEvent` into `Option<Key>`.
    unsafe fn translate_key_event(&mut self, event: &mut x11::XKeyEvent) -> Option<Key> {
        for index in 0..4 {
            if let Some(key) = translate_key(self.x11.XLookupKeysym(event, index)) {
                return Some(key);
            }
        }
        None
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            self.x11.XDestroyWindow(self.display, self.handle);
            self.x11.XCloseDisplay(self.display);
        }
    }
}

/// Translates a X11 key to `Option<Key>`.
fn translate_key(symbol: x11::KeySym) -> Option<Key> {
    Some(match symbol {
        // Typing
        x11::XK_A => Key::A,
        x11::XK_B => Key::B,
        x11::XK_C => Key::C,
        x11::XK_D => Key::D,
        x11::XK_E => Key::E,
        x11::XK_F => Key::F,
        x11::XK_G => Key::G,
        x11::XK_H => Key::H,
        x11::XK_I => Key::I,
        x11::XK_J => Key::J,
        x11::XK_K => Key::K,
        x11::XK_L => Key::L,
        x11::XK_M => Key::M,
        x11::XK_N => Key::N,
        x11::XK_O => Key::O,
        x11::XK_P => Key::P,
        x11::XK_Q => Key::Q,
        x11::XK_R => Key::R,
        x11::XK_S => Key::S,
        x11::XK_T => Key::T,
        x11::XK_U => Key::U,
        x11::XK_V => Key::V,
        x11::XK_W => Key::W,
        x11::XK_X => Key::X,
        x11::XK_Y => Key::Y,
        x11::XK_Z => Key::Z,
        x11::XK_0 => Key::Digit0,
        x11::XK_1 => Key::Digit1,
        x11::XK_2 => Key::Digit2,
        x11::XK_3 => Key::Digit3,
        x11::XK_4 => Key::Digit4,
        x11::XK_5 => Key::Digit5,
        x11::XK_6 => Key::Digit6,
        x11::XK_7 => Key::Digit7,
        x11::XK_8 => Key::Digit8,
        x11::XK_9 => Key::Digit9,
        x11::XK_grave => Key::Backquote,
        x11::XK_minus => Key::Minus,
        x11::XK_equal => Key::Equal,
        x11::XK_bracketleft => Key::LeftBracket,
        x11::XK_bracketright => Key::RightBracket,
        x11::XK_backslash => Key::Backslash,
        x11::XK_semicolon => Key::Semicolon,
        x11::XK_apostrophe => Key::Quote,
        x11::XK_comma => Key::Comma,
        x11::XK_period => Key::Period,
        x11::XK_slash => Key::Slash,
        x11::XK_Tab => Key::Tab,
        x11::XK_Caps_Lock => Key::CapsLock,
        x11::XK_Shift_L => Key::LeftShift,
        x11::XK_BackSpace => Key::Backspace,
        x11::XK_Return => Key::Enter,
        x11::XK_Shift_R => Key::RightShift,
        x11::XK_space => Key::Space,
        // Control
        x11::XK_Escape => Key::Escape,
        x11::XK_Print => Key::PrintScreen,
        x11::XK_Scroll_Lock => Key::ScrollLock,
        x11::XK_Break => Key::Pause,
        x11::XK_Control_L => Key::LeftControl,
        x11::XK_Super_L => Key::LeftSuper,
        x11::XK_Alt_L => Key::LeftAlt,
        x11::XK_Alt_R => Key::RightAlt,
        x11::XK_Super_R => Key::RightSuper,
        x11::XK_Menu => Key::Menu,
        x11::XK_Control_R => Key::RightControl,
        // Function
        x11::XK_F1 => Key::F1,
        x11::XK_F2 => Key::F2,
        x11::XK_F3 => Key::F3,
        x11::XK_F4 => Key::F4,
        x11::XK_F5 => Key::F5,
        x11::XK_F6 => Key::F6,
        x11::XK_F7 => Key::F7,
        x11::XK_F8 => Key::F8,
        x11::XK_F9 => Key::F9,
        x11::XK_F10 => Key::F10,
        x11::XK_F11 => Key::F11,
        x11::XK_F12 => Key::F12,
        // Navigation
        x11::XK_Insert => Key::Insert,
        x11::XK_Delete => Key::Delete,
        x11::XK_Home => Key::Home,
        x11::XK_End => Key::End,
        x11::XK_Prior => Key::PageUp,
        x11::XK_Next => Key::PageDown,
        x11::XK_Up => Key::UpArrow,
        x11::XK_Down => Key::DownArrow,
        x11::XK_Left => Key::LeftArrow,
        x11::XK_Right => Key::RightArrow,
        // Numeric keypad
        x11::XK_Num_Lock => Key::NumLock,
        x11::XK_KP_0 => Key::Numpad0,
        x11::XK_KP_1 => Key::Numpad1,
        x11::XK_KP_2 => Key::Numpad2,
        x11::XK_KP_3 => Key::Numpad3,
        x11::XK_KP_4 => Key::Numpad4,
        x11::XK_KP_5 => Key::Numpad5,
        x11::XK_KP_6 => Key::Numpad6,
        x11::XK_KP_7 => Key::Numpad7,
        x11::XK_KP_8 => Key::Numpad8,
        x11::XK_KP_9 => Key::Numpad9,
        x11::XK_KP_Enter => Key::NumpadEnter,
        x11::XK_KP_Divide => Key::NumpadDivide,
        x11::XK_KP_Multiply => Key::NumpadMultiply,
        x11::XK_KP_Subtract => Key::NumpadSubtract,
        x11::XK_KP_Add => Key::NumpadAdd,
        x11::XK_KP_Decimal => Key::NumpadDecimal,
        // Unknown
        _ => return None,
    })
}

/// Translates a X11 button to `Option<Button>`.
fn translate_button(button: c_uint) -> Option<Button> {
    Some(match button {
        // Common
        x11::Button1 => Button::Left,
        x11::Button2 => Button::Middle,
        x11::Button3 => Button::Right,
        // Extra
        8 => Button::Back,
        9 => Button::Forward,
        // Unknown
        _ => return None,
    })
}
