//! Xlib-based windows.

use crate::{Result, WindowError};
use blazar_event::{Button, Event, Key};
use blazar_xlib as xlib;
use std::{
    collections::VecDeque,
    ffi::CString,
    mem,
    os::raw::{c_int, c_uchar, c_uint},
    ptr,
};

/// Represents a window.
pub struct Window {
    x11: xlib::dl::X11Library,
    display: *mut xlib::Display,
    handle: xlib::Window,
    wm_protocols: xlib::Atom,
    wm_delete_window: xlib::Atom,
    width: u32,
    height: u32,
    events: VecDeque<Event>,
}

impl Window {
    /// Creates a new window.
    pub fn create(title: &str, width: u32, height: u32) -> Result<Window> {
        unsafe {
            // Loads Xlib.
            let x11 = xlib::dl::X11Library::load().map_err(|_| WindowError::CreateWindowError)?;

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
                xlib::ButtonPressMask
                    | xlib::ButtonReleaseMask
                    | xlib::ExposureMask
                    | xlib::FocusChangeMask
                    | xlib::PointerMotionMask
                    | xlib::KeyPressMask
                    | xlib::KeyReleaseMask
                    | xlib::StructureNotifyMask,
            );

            // Sets window's title.
            let utf8_string = CString::new("UTF8_STRING").unwrap();
            let utf8_string = x11.XInternAtom(display, utf8_string.as_ptr(), xlib::FALSE);
            let net_wm_name = CString::new("_NET_WM_NAME").unwrap();
            let net_wm_name = x11.XInternAtom(display, net_wm_name.as_ptr(), xlib::FALSE);
            let net_wm_icon_name = CString::new("_NET_WM_ICON_NAME").unwrap();
            let net_wm_icon_name = x11.XInternAtom(display, net_wm_icon_name.as_ptr(), xlib::FALSE);
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
                xlib::PropModeReplace,
                title.as_ptr() as *const c_uchar,
                title.as_bytes().len() as c_int,
            );
            x11.XChangeProperty(
                display,
                handle,
                net_wm_icon_name,
                utf8_string,
                8,
                xlib::PropModeReplace,
                title.as_ptr() as *const c_uchar,
                title.as_bytes().len() as c_int,
            );
            x11.XFlush(display);

            // Handles WM delete window (close button).
            let wm_protocols = CString::new("WM_PROTOCOLS").unwrap();
            let wm_protocols = x11.XInternAtom(display, wm_protocols.as_ptr(), xlib::FALSE);
            let wm_delete_window = CString::new("WM_DELETE_WINDOW").unwrap();
            let wm_delete_window = x11.XInternAtom(display, wm_delete_window.as_ptr(), xlib::FALSE);
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
                let mut event: xlib::XEvent = mem::zeroed();
                self.x11.XNextEvent(self.display, &mut event);
                if let Some(event) = self.translate_event(&mut event) {
                    self.events.push_back(event)
                }
            }
        }
    }

    /// Translates `XEvent` into `Option<Event>`.
    unsafe fn translate_event(&mut self, event: &mut xlib::XEvent) -> Option<Event> {
        match event.r#type {
            // Window
            xlib::ClientMessage
                if event.client_message.message_type == self.wm_protocols
                    && event.client_message.data.longs[0] as xlib::Atom
                        == self.wm_delete_window =>
            {
                Some(Event::Close)
            }
            xlib::FocusIn => Some(Event::GainFocus),
            xlib::FocusOut => Some(Event::LoseFocus),
            xlib::ConfigureNotify
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
            xlib::KeyPress => self
                .translate_key_event(&mut event.key)
                .map(|key| Event::KeyPress { key }),
            xlib::KeyRelease => {
                // Ignores auto-repeat.
                if self.x11.XPending(self.display) > 0 {
                    let mut next_event: xlib::XEvent = mem::zeroed();
                    self.x11.XPeekEvent(self.display, &mut next_event);
                    if next_event.r#type == xlib::KeyPress
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
            xlib::ButtonPress => match event.button.button {
                xlib::Button4 => Some(Event::MouseScrollUp),
                xlib::Button5 => Some(Event::MouseScrollDown),
                _ => translate_button(event.button.button).map(|button| Event::MouseButtonPress {
                    button,
                    x: event.button.x,
                    y: event.button.y,
                }),
            },
            xlib::ButtonRelease => {
                translate_button(event.button.button).map(|button| Event::MouseButtonRelease {
                    button,
                    x: event.button.x,
                    y: event.button.y,
                })
            }
            xlib::MotionNotify => Some(Event::MouseMove {
                x: event.button.x,
                y: event.button.y,
            }),
            // Unknown
            _ => None,
        }
    }

    /// Translates an `XKeyEvent` into `Option<Key>`.
    unsafe fn translate_key_event(&mut self, event: &mut xlib::XKeyEvent) -> Option<Key> {
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
fn translate_key(symbol: xlib::KeySym) -> Option<Key> {
    Some(match symbol {
        // Typing
        xlib::XK_A => Key::A,
        xlib::XK_B => Key::B,
        xlib::XK_C => Key::C,
        xlib::XK_D => Key::D,
        xlib::XK_E => Key::E,
        xlib::XK_F => Key::F,
        xlib::XK_G => Key::G,
        xlib::XK_H => Key::H,
        xlib::XK_I => Key::I,
        xlib::XK_J => Key::J,
        xlib::XK_K => Key::K,
        xlib::XK_L => Key::L,
        xlib::XK_M => Key::M,
        xlib::XK_N => Key::N,
        xlib::XK_O => Key::O,
        xlib::XK_P => Key::P,
        xlib::XK_Q => Key::Q,
        xlib::XK_R => Key::R,
        xlib::XK_S => Key::S,
        xlib::XK_T => Key::T,
        xlib::XK_U => Key::U,
        xlib::XK_V => Key::V,
        xlib::XK_W => Key::W,
        xlib::XK_X => Key::X,
        xlib::XK_Y => Key::Y,
        xlib::XK_Z => Key::Z,
        xlib::XK_0 => Key::Digit0,
        xlib::XK_1 => Key::Digit1,
        xlib::XK_2 => Key::Digit2,
        xlib::XK_3 => Key::Digit3,
        xlib::XK_4 => Key::Digit4,
        xlib::XK_5 => Key::Digit5,
        xlib::XK_6 => Key::Digit6,
        xlib::XK_7 => Key::Digit7,
        xlib::XK_8 => Key::Digit8,
        xlib::XK_9 => Key::Digit9,
        xlib::XK_grave => Key::Backquote,
        xlib::XK_minus => Key::Minus,
        xlib::XK_equal => Key::Equal,
        xlib::XK_bracketleft => Key::LeftBracket,
        xlib::XK_bracketright => Key::RightBracket,
        xlib::XK_backslash => Key::Backslash,
        xlib::XK_semicolon => Key::Semicolon,
        xlib::XK_apostrophe => Key::Quote,
        xlib::XK_comma => Key::Comma,
        xlib::XK_period => Key::Period,
        xlib::XK_slash => Key::Slash,
        xlib::XK_Tab => Key::Tab,
        xlib::XK_Caps_Lock => Key::CapsLock,
        xlib::XK_Shift_L => Key::LeftShift,
        xlib::XK_BackSpace => Key::Backspace,
        xlib::XK_Return => Key::Enter,
        xlib::XK_Shift_R => Key::RightShift,
        xlib::XK_space => Key::Space,
        // Control
        xlib::XK_Escape => Key::Escape,
        xlib::XK_Print => Key::PrintScreen,
        xlib::XK_Scroll_Lock => Key::ScrollLock,
        xlib::XK_Break => Key::Pause,
        xlib::XK_Control_L => Key::LeftControl,
        xlib::XK_Super_L => Key::LeftSuper,
        xlib::XK_Alt_L => Key::LeftAlt,
        xlib::XK_Alt_R => Key::RightAlt,
        xlib::XK_Super_R => Key::RightSuper,
        xlib::XK_Menu => Key::Menu,
        xlib::XK_Control_R => Key::RightControl,
        // Function
        xlib::XK_F1 => Key::F1,
        xlib::XK_F2 => Key::F2,
        xlib::XK_F3 => Key::F3,
        xlib::XK_F4 => Key::F4,
        xlib::XK_F5 => Key::F5,
        xlib::XK_F6 => Key::F6,
        xlib::XK_F7 => Key::F7,
        xlib::XK_F8 => Key::F8,
        xlib::XK_F9 => Key::F9,
        xlib::XK_F10 => Key::F10,
        xlib::XK_F11 => Key::F11,
        xlib::XK_F12 => Key::F12,
        // Navigation
        xlib::XK_Insert => Key::Insert,
        xlib::XK_Delete => Key::Delete,
        xlib::XK_Home => Key::Home,
        xlib::XK_End => Key::End,
        xlib::XK_Prior => Key::PageUp,
        xlib::XK_Next => Key::PageDown,
        xlib::XK_Up => Key::UpArrow,
        xlib::XK_Down => Key::DownArrow,
        xlib::XK_Left => Key::LeftArrow,
        xlib::XK_Right => Key::RightArrow,
        // Numeric keypad
        xlib::XK_Num_Lock => Key::NumLock,
        xlib::XK_KP_0 => Key::Numpad0,
        xlib::XK_KP_1 => Key::Numpad1,
        xlib::XK_KP_2 => Key::Numpad2,
        xlib::XK_KP_3 => Key::Numpad3,
        xlib::XK_KP_4 => Key::Numpad4,
        xlib::XK_KP_5 => Key::Numpad5,
        xlib::XK_KP_6 => Key::Numpad6,
        xlib::XK_KP_7 => Key::Numpad7,
        xlib::XK_KP_8 => Key::Numpad8,
        xlib::XK_KP_9 => Key::Numpad9,
        xlib::XK_KP_Enter => Key::NumpadEnter,
        xlib::XK_KP_Divide => Key::NumpadDivide,
        xlib::XK_KP_Multiply => Key::NumpadMultiply,
        xlib::XK_KP_Subtract => Key::NumpadSubtract,
        xlib::XK_KP_Add => Key::NumpadAdd,
        xlib::XK_KP_Decimal => Key::NumpadDecimal,
        // Unknown
        _ => return None,
    })
}

/// Translates a X11 button to `Option<Button>`.
fn translate_button(button: c_uint) -> Option<Button> {
    Some(match button {
        // Common
        xlib::Button1 => Button::Left,
        xlib::Button2 => Button::Middle,
        xlib::Button3 => Button::Right,
        // Extra
        8 => Button::Back,
        9 => Button::Forward,
        // Unknown
        _ => return None,
    })
}
