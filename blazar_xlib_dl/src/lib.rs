//! Xlib dynamic loading.

#![allow(clippy::too_many_arguments, non_snake_case)]

use blazar_dl::dynamic_loading;
use blazar_xlib::*;
use std::os::raw::*;

dynamic_loading! {
    #[load(wrapper = X11Library, error = LoadX11Error, name = "X11")]
    extern "C" {
        pub fn XBlackPixel(display: *mut Display, screen_number: c_int) -> c_ulong;
        pub fn XCloseDisplay(display: *mut Display) -> c_int;
        pub fn XChangeProperty(
            display: *mut Display,
            w: c_ulong,
            property: c_ulong,
            r#type: c_ulong,
            format: c_int,
            mode: c_int,
            data: *const c_uchar,
            nelements: c_int
        ) -> c_int;
        pub fn XCreateSimpleWindow(
            display: *mut Display,
            parent: Window,
            x: c_int,
            y: c_int,
            width: c_uint,
            height: c_uint,
            border_width: c_uint,
            border: c_ulong,
            background: c_ulong
        ) -> Window;
        pub fn XDefaultRootWindow(display: *mut Display) -> Window;
        pub fn XDefaultScreen(display: *mut Display) -> c_int;
        pub fn XDestroyWindow(display: *mut Display, window: Window) -> c_int;
        pub fn XFlush(display: *mut Display) -> c_int;
        pub fn XInternAtom(
            display: *mut Display,
            atom_name: *const c_char,
            only_if_exists: Bool
        ) -> Atom;
        pub fn XLookupKeysym(key_event: *mut XKeyEvent, index: c_int) -> KeySym;
        pub fn XMapWindow(display: *mut Display, w: Window) -> c_int;
        pub fn XNextEvent(display: *mut Display, event_return: *mut XEvent) -> c_int;
        pub fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
        pub fn XPeekEvent(display: *mut Display, event_return: *mut XEvent) -> c_int;
        pub fn XPending(display: *mut Display) -> c_int;
        pub fn XSelectInput(display: *mut Display, w: Window, event_mask: c_long) -> c_int;
        pub fn XSetWMProtocols(
            display: *mut Display,
            w: Window,
            protocols: *mut Atom,
            count: c_int
        ) -> Status;
        pub fn Xutf8SetWMProperties(
            display: *mut Display,
            w: c_ulong,
            window_name: *const c_char,
            icon_name: *const c_char,
            argv: *mut *mut c_char,
            argc: c_int,
            normal_hints: *mut XSizeHints,
            wm_hints: *mut XWMHints,
            class_hints: *mut XClassHint
        ) -> c_void;
    }
}
