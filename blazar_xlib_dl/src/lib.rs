//! Xlib dynamic loading.

#![allow(clippy::too_many_arguments, non_snake_case)]

use blazar_dl::dynamic_loading;
use blazar_xlib::*;
use std::os::raw::*;

dynamic_loading! {
    pub enum X11DynamicLoadingError {
        LibraryLoading,
        FunctionLoading,
    }

    #[load(name = "X11")]
    pub struct X11Library {
        fn XBlackPixel(display: *mut Display, screen_number: c_int) -> c_ulong;
        fn XCloseDisplay(display: *mut Display) -> c_int;
        fn XChangeProperty(
            display: *mut Display,
            w: c_ulong,
            property: c_ulong,
            r#type: c_ulong,
            format: c_int,
            mode: c_int,
            data: *const c_uchar,
            nelements: c_int
        ) -> c_int;
        fn XCreateSimpleWindow(
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
        fn XDefaultRootWindow(display: *mut Display) -> Window;
        fn XDefaultScreen(display: *mut Display) -> c_int;
        fn XDestroyWindow(display: *mut Display, window: Window) -> c_int;
        fn XFlush(display: *mut Display) -> c_int;
        fn XInternAtom(
            display: *mut Display,
            atom_name: *const c_char,
            only_if_exists: Bool
        ) -> Atom;
        fn XLookupKeysym(key_event: *mut XKeyEvent, index: c_int) -> KeySym;
        fn XMapWindow(display: *mut Display, w: Window) -> c_int;
        fn XNextEvent(display: *mut Display, event_return: *mut XEvent) -> c_int;
        fn XOpenDisplay(display_name: *const c_char) -> *mut Display;
        fn XPeekEvent(display: *mut Display, event_return: *mut XEvent) -> c_int;
        fn XPending(display: *mut Display) -> c_int;
        fn XSelectInput(display: *mut Display, w: Window, event_mask: c_long) -> c_int;
        fn XSetWMProtocols(
            display: *mut Display,
            w: Window,
            protocols: *mut Atom,
            count: c_int
        ) -> Status;
        fn Xutf8SetWMProperties(
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
