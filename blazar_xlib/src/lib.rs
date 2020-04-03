//! Xlib raw FFI bindings.

#![allow(clippy::too_many_arguments, non_snake_case, non_upper_case_globals)]

use blazar_dl::library;
use std::os::raw::*;

// Types
pub type Atom = XID;
pub type Bool = c_int;
pub type KeySym = XID;
pub type Status = Bool;
pub type Time = c_ulong;
pub type Window = XID;
pub type XID = c_ulong;

// Opaque structures
pub enum Display {}
pub enum XClassHint {}
pub enum XSizeHints {}
pub enum XWMHints {}

// Structures
#[derive(Clone, Copy)]
#[repr(C)]
pub struct XButtonEvent {
    pub r#type: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub button: c_uint,
    pub same_screen: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XClientMessageData {
    pub longs: [c_long; 5],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XClientMessageEvent {
    pub r#type: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub message_type: Atom,
    pub format: c_int,
    pub data: XClientMessageData,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XConfigureEvent {
    pub r#type: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub event: Window,
    pub window: Window,
    pub x: c_int,
    pub y: c_int,
    pub width: c_int,
    pub height: c_int,
    pub border_width: c_int,
    pub above: Window,
    pub override_redirect: Bool,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union XEvent {
    pub r#type: c_int,
    pub button: XButtonEvent,
    pub client_message: XClientMessageEvent,
    pub configure: XConfigureEvent,
    pub key: XKeyEvent,
    pub pad: [c_long; 24],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct XKeyEvent {
    pub r#type: c_int,
    pub serial: c_ulong,
    pub send_event: Bool,
    pub display: *mut Display,
    pub window: Window,
    pub root: Window,
    pub subwindow: Window,
    pub time: Time,
    pub x: c_int,
    pub y: c_int,
    pub x_root: c_int,
    pub y_root: c_int,
    pub state: c_uint,
    pub keycode: c_uint,
    pub same_screen: Bool,
}

// Constants
pub const Button1: c_uint = 1;
pub const Button2: c_uint = 2;
pub const Button3: c_uint = 3;
pub const Button4: c_uint = 4;
pub const Button5: c_uint = 5;

pub const KeyPressMask: c_long = 0x0000_0001;
pub const KeyReleaseMask: c_long = 0x0000_0002;
pub const ButtonPressMask: c_long = 0x0000_0004;
pub const ButtonReleaseMask: c_long = 0x0000_0008;
pub const PointerMotionMask: c_long = 0x0000_0040;
pub const ExposureMask: c_long = 0x0000_8000;
pub const StructureNotifyMask: c_long = 0x0002_0000;
pub const FocusChangeMask: c_long = 0x0020_0000;

pub const KeyPress: c_int = 2;
pub const KeyRelease: c_int = 3;
pub const ButtonPress: c_int = 4;
pub const ButtonRelease: c_int = 5;
pub const MotionNotify: c_int = 6;
pub const FocusIn: c_int = 9;
pub const FocusOut: c_int = 10;
pub const ConfigureNotify: c_int = 22;
pub const ClientMessage: c_int = 33;

pub const FALSE: Bool = 0;

pub const PropModeReplace: c_int = 0;

pub const XK_space: KeySym = 0x020;
pub const XK_apostrophe: KeySym = 0x027;
pub const XK_comma: KeySym = 0x02c;
pub const XK_minus: KeySym = 0x02d;
pub const XK_period: KeySym = 0x02e;
pub const XK_slash: KeySym = 0x02f;
pub const XK_0: KeySym = 0x030;
pub const XK_1: KeySym = 0x031;
pub const XK_2: KeySym = 0x032;
pub const XK_3: KeySym = 0x033;
pub const XK_4: KeySym = 0x034;
pub const XK_5: KeySym = 0x035;
pub const XK_6: KeySym = 0x036;
pub const XK_7: KeySym = 0x037;
pub const XK_8: KeySym = 0x038;
pub const XK_9: KeySym = 0x039;
pub const XK_semicolon: KeySym = 0x03b;
pub const XK_equal: KeySym = 0x03d;
pub const XK_bracketleft: KeySym = 0x05b;
pub const XK_backslash: KeySym = 0x05c;
pub const XK_bracketright: KeySym = 0x05d;
pub const XK_grave: KeySym = 0x060;
pub const XK_A: KeySym = 0x061;
pub const XK_B: KeySym = 0x062;
pub const XK_C: KeySym = 0x063;
pub const XK_D: KeySym = 0x064;
pub const XK_E: KeySym = 0x065;
pub const XK_F: KeySym = 0x066;
pub const XK_G: KeySym = 0x067;
pub const XK_H: KeySym = 0x068;
pub const XK_I: KeySym = 0x069;
pub const XK_J: KeySym = 0x06a;
pub const XK_K: KeySym = 0x06b;
pub const XK_L: KeySym = 0x06c;
pub const XK_M: KeySym = 0x06d;
pub const XK_N: KeySym = 0x06e;
pub const XK_O: KeySym = 0x06f;
pub const XK_P: KeySym = 0x070;
pub const XK_Q: KeySym = 0x071;
pub const XK_R: KeySym = 0x072;
pub const XK_S: KeySym = 0x073;
pub const XK_T: KeySym = 0x074;
pub const XK_U: KeySym = 0x075;
pub const XK_V: KeySym = 0x076;
pub const XK_W: KeySym = 0x077;
pub const XK_X: KeySym = 0x078;
pub const XK_Y: KeySym = 0x079;
pub const XK_Z: KeySym = 0x07a;
pub const XK_BackSpace: KeySym = 0xff08;
pub const XK_Tab: KeySym = 0xff09;
pub const XK_Return: KeySym = 0xff0d;
pub const XK_Scroll_Lock: KeySym = 0xff14;
pub const XK_Escape: KeySym = 0xff1b;
pub const XK_Home: KeySym = 0xff50;
pub const XK_Left: KeySym = 0xff51;
pub const XK_Up: KeySym = 0xff52;
pub const XK_Right: KeySym = 0xff53;
pub const XK_Down: KeySym = 0xff54;
pub const XK_Prior: KeySym = 0xff55;
pub const XK_Next: KeySym = 0xff56;
pub const XK_End: KeySym = 0xff57;
pub const XK_Print: KeySym = 0xff61;
pub const XK_Insert: KeySym = 0xff63;
pub const XK_Menu: KeySym = 0xff67;
pub const XK_Break: KeySym = 0xff6b;
pub const XK_Num_Lock: KeySym = 0xff7f;
pub const XK_KP_Enter: KeySym = 0xff8d;
pub const XK_KP_Multiply: KeySym = 0xffaa;
pub const XK_KP_Add: KeySym = 0xffab;
pub const XK_KP_Subtract: KeySym = 0xffad;
pub const XK_KP_Decimal: KeySym = 0xffae;
pub const XK_KP_Divide: KeySym = 0xffaf;
pub const XK_KP_0: KeySym = 0xffb0;
pub const XK_KP_1: KeySym = 0xffb1;
pub const XK_KP_2: KeySym = 0xffb2;
pub const XK_KP_3: KeySym = 0xffb3;
pub const XK_KP_4: KeySym = 0xffb4;
pub const XK_KP_5: KeySym = 0xffb5;
pub const XK_KP_6: KeySym = 0xffb6;
pub const XK_KP_7: KeySym = 0xffb7;
pub const XK_KP_8: KeySym = 0xffb8;
pub const XK_KP_9: KeySym = 0xffb9;
pub const XK_F1: KeySym = 0xffbe;
pub const XK_F2: KeySym = 0xffbf;
pub const XK_F3: KeySym = 0xffc0;
pub const XK_F4: KeySym = 0xffc1;
pub const XK_F5: KeySym = 0xffc2;
pub const XK_F6: KeySym = 0xffc3;
pub const XK_F7: KeySym = 0xffc4;
pub const XK_F8: KeySym = 0xffc5;
pub const XK_F9: KeySym = 0xffc6;
pub const XK_F10: KeySym = 0xffc7;
pub const XK_F11: KeySym = 0xffc8;
pub const XK_F12: KeySym = 0xffc9;
pub const XK_Shift_L: KeySym = 0xffe1;
pub const XK_Shift_R: KeySym = 0xffe2;
pub const XK_Control_L: KeySym = 0xffe3;
pub const XK_Control_R: KeySym = 0xffe4;
pub const XK_Caps_Lock: KeySym = 0xffe5;
pub const XK_Alt_L: KeySym = 0xffe9;
pub const XK_Alt_R: KeySym = 0xffea;
pub const XK_Super_L: KeySym = 0xffeb;
pub const XK_Super_R: KeySym = 0xffec;
pub const XK_Delete: KeySym = 0xffff;

// Functions
library! {
    #[load(name = "X11")]
    struct X11Library {
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
