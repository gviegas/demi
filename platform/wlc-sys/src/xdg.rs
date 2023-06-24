use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::{
    proxy_add_listener, proxy_get_version, Interface, Message, Output, Seat, Surface,
    MARSHAL_FLAG_DESTROY, OUTPUT_INTERFACE, SEAT_INTERFACE, SURFACE_INTERFACE,
};

const NULL_TYPES: *const *const Interface = [ptr::null(); 4].as_ptr();

/// struct xdg_wm_base
#[repr(C)]
pub struct WmBase {
    _opaque: [u8; 0],
}

/// xdg_wm_base_interface
pub static WM_BASE_INTERFACE: Interface = Interface {
    name: b"xdg_wm_base\0".as_ptr().cast(),
    version: 4,
    method_count: 4,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"create_positioner\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&POSITIONER_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"get_xdg_surface\0".as_ptr().cast(),
            signature: b"no\0".as_ptr().cast(),
            types: [
                &XDG_SURFACE_INTERFACE as *const Interface,
                &SURFACE_INTERFACE,
            ]
            .as_ptr(),
        },
        Message {
            name: b"pong\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 1,
    events: [Message {
        name: b"ping\0".as_ptr().cast(),
        signature: b"u\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
};

/// XDG_WM_BASE_ERROR_ROLE
pub const WM_BASE_ERROR_ROLE: c_int = 0;

/// XDG_WM_BASE_ERROR_DEFUNCT_SURFACES
pub const WM_BASE_ERROR_DEFUNCT_SURFACES: c_int = 1;

/// XDG_WM_BASE_ERROR_NOT_THE_TOPMOST_POPUP
pub const WM_BASE_ERROR_NOT_THE_TOPMOST_POPUP: c_int = 2;

/// XDG_WM_BASE_ERROR_INVALID_POPUP_PARENT
pub const WM_BASE_ERROR_INVALID_POPUP_PARENT: c_int = 3;

/// XDG_WM_BASE_ERROR_INVALID_SURFACE_STATE
pub const WM_BASE_ERROR_INVALID_SURFACE_STATE: c_int = 4;

/// XDG_WM_BASE_ERROR_INVALID_POSITIONER
pub const WM_BASE_ERROR_INVALID_POSITIONER: c_int = 5;

/// struct xdg_wm_base_listener
#[repr(C)]
pub struct WmBaseListener {
    pub ping: unsafe extern "C" fn(data: *mut c_void, wm_base: *mut WmBase, serial: u32),
}

/// xdg_wm_base_add_listener
pub unsafe fn wm_base_add_listener(
    wm_base: *mut WmBase,
    listener: *const WmBaseListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(wm_base.cast(), listener as *mut _, data)
}

const WM_BASE_DESTROY: u32 = 0;
const WM_BASE_CREATE_POSITIONER: u32 = 1;
const WM_BASE_GET_XDG_SURFACE: u32 = 2;
const WM_BASE_PONG: u32 = 3;

/// xdg_wm_base_destroy
pub unsafe fn wm_base_destroy(wm_base: *mut WmBase) {
    proxy_marshal_flags!(
        wm_base.cast(),
        WM_BASE_DESTROY,
        ptr::null(),
        proxy_get_version(wm_base.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// xdg_wm_base_create_positioner
pub unsafe fn wm_base_create_positioner(wm_base: *mut WmBase) -> *mut Positioner {
    proxy_marshal_flags!(
        wm_base.cast(),
        WM_BASE_CREATE_POSITIONER,
        &POSITIONER_INTERFACE,
        proxy_get_version(wm_base.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// xdg_wm_base_get_xdg_surface
pub unsafe fn wm_base_get_xdg_surface(
    wm_base: *mut WmBase,
    surface: *mut Surface,
) -> *mut XdgSurface {
    proxy_marshal_flags!(
        wm_base.cast(),
        WM_BASE_GET_XDG_SURFACE,
        &XDG_SURFACE_INTERFACE,
        proxy_get_version(wm_base.cast()),
        0,
        ptr::null::<usize>(), // XXX
        surface
    )
    .cast()
}

/// xdg_wm_base_pong
pub unsafe fn wm_base_pong(wm_base: *mut WmBase, serial: u32) {
    proxy_marshal_flags!(
        wm_base.cast(),
        WM_BASE_PONG,
        ptr::null(),
        proxy_get_version(wm_base.cast()),
        0,
        serial
    );
}

/// struct xdg_positioner
#[repr(C)]
pub struct Positioner {
    _opaque: [u8; 0],
}

/// xdg_positioner_interface
pub static POSITIONER_INTERFACE: Interface = Interface {
    name: b"xdg_positioner\0".as_ptr().cast(),
    version: 4,
    method_count: 10,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_size\0".as_ptr().cast(),
            signature: b"ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_anchor_rect\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_anchor\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_gravity\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_constraint_adjustment\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_offset\0".as_ptr().cast(),
            signature: b"ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_reactive\0".as_ptr().cast(),
            signature: b"3\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_parent_size\0".as_ptr().cast(),
            signature: b"3ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_parent_configure\0".as_ptr().cast(),
            signature: b"3u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 0,
    events: ptr::null(),
};

/// XDG_POSITIONER_ERROR_INVALID_INPUT
pub const POSITIONER_ERROR_INVALID_INPUT: c_int = 0;

/// XDG_POSITIONER_ANCHOR_NONE
pub const POSITIONER_ANCHOR_NONE: u32 = 0;

/// XDG_POSITIONER_ANCHOR_TOP
pub const POSITIONER_ANCHOR_TOP: u32 = 1;

/// XDG_POSITIONER_ANCHOR_BOTTOM
pub const POSITIONER_ANCHOR_BOTTOM: u32 = 2;

/// XDG_POSITIONER_ANCHOR_LEFT
pub const POSITIONER_ANCHOR_LEFT: u32 = 3;

/// XDG_POSITIONER_ANCHOR_RIGHT
pub const POSITIONER_ANCHOR_RIGHT: u32 = 4;

/// XDG_POSITIONER_ANCHOR_TOP_LEFT
pub const POSITIONER_ANCHOR_TOP_LEFT: u32 = 5;

/// XDG_POSITIONER_ANCHOR_BOTTOM_LEFT
pub const POSITIONER_ANCHOR_BOTTOM_LEFT: u32 = 6;

/// XDG_POSITIONER_ANCHOR_TOP_RIGHT
pub const POSITIONER_ANCHOR_TOP_RIGHT: u32 = 7;

/// XDG_POSITIONER_ANCHOR_BOTTOM_RIGHT
pub const POSITIONER_ANCHOR_BOTTOM_RIGHT: u32 = 8;

/// XDG_POSITIONER_GRAVITY_NONE
pub const POSITIONER_GRAVITY_NONE: u32 = 0;

/// XDG_POSITIONER_GRAVITY_TOP
pub const POSITIONER_GRAVITY_TOP: u32 = 1;

/// XDG_POSITIONER_GRAVITY_BOTTOM
pub const POSITIONER_GRAVITY_BOTTOM: u32 = 2;

/// XDG_POSITIONER_GRAVITY_LEFT
pub const POSITIONER_GRAVITY_LEFT: u32 = 3;

/// XDG_POSITIONER_GRAVITY_RIGHT
pub const POSITIONER_GRAVITY_RIGHT: u32 = 4;

/// XDG_POSITIONER_GRAVITY_TOP_LEFT
pub const POSITIONER_GRAVITY_TOP_LEFT: u32 = 5;

/// XDG_POSITIONER_GRAVITY_BOTTOM_LEFT
pub const POSITIONER_GRAVITY_BOTTOM_LEFT: u32 = 6;

/// XDG_POSITIONER_GRAVITY_TOP_RIGHT
pub const POSITIONER_GRAVITY_TOP_RIGHT: u32 = 7;

/// XDG_POSITIONER_GRAVITY_BOTTOM_RIGHT
pub const POSITIONER_GRAVITY_BOTTOM_RIGHT: u32 = 8;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_NONE
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_NONE: u32 = 0;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_X
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_X: u32 = 0x1;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_Y
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_SLIDE_Y: u32 = 0x2;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_X
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_X: u32 = 0x4;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_Y
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_FLIP_Y: u32 = 0x8;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_X
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_X: u32 = 0x10;

/// XDG_POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_Y
pub const POSITIONER_CONSTRAINT_ADJUSTMENT_RESIZE_Y: u32 = 0x20;

const POSITIONER_DESTROY: u32 = 0;
const POSITIONER_SET_SIZE: u32 = 1;
const POSITIONER_SET_ANCHOR_RECT: u32 = 2;
const POSITIONER_SET_ANCHOR: u32 = 3;
const POSITIONER_SET_GRAVITY: u32 = 4;
const POSITIONER_SET_CONSTRAINT_ADJUSTMENT: u32 = 5;
const POSITIONER_SET_OFFSET: u32 = 6;
const POSITIONER_SET_REACTIVE: u32 = 7;
const POSITIONER_SET_PARENT_SIZE: u32 = 8;
const POSITIONER_SET_PARENT_CONFIGURE: u32 = 9;

/// xdg_positioner_destroy
pub unsafe fn positioner_destroy(positioner: *mut Positioner) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_DESTROY,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// xdg_positioner_set_size
pub unsafe fn positioner_set_size(positioner: *mut Positioner, width: i32, height: i32) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_SIZE,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        width,
        height
    );
}

/// xdg_positioner_set_anchor_rect
pub unsafe fn positioner_set_anchor_rect(
    positioner: *mut Positioner,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_ANCHOR_RECT,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        x,
        y,
        width,
        height
    );
}

/// xdg_positioner_set_anchor
pub unsafe fn positioner_set_anchor(positioner: *mut Positioner, anchor: u32) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_ANCHOR,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        anchor
    );
}

/// xdg_positioner_set_gravity
pub unsafe fn positioner_set_gravity(positioner: *mut Positioner, gravity: u32) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_GRAVITY,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        gravity
    );
}

/// xdg_positioner_set_constraint_adjustment
pub unsafe fn positioner_set_constraint_adjustment(
    positioner: *mut Positioner,
    constraint_adjustment: u32,
) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_CONSTRAINT_ADJUSTMENT,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        constraint_adjustment
    );
}

/// xdg_positioner_set_offset
pub unsafe fn positioner_set_offset(positioner: *mut Positioner, x: i32, y: i32) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_OFFSET,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        x,
        y
    );
}

/// xdg_positioner_set_reactive
pub unsafe fn positioner_set_reactive(positioner: *mut Positioner) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_REACTIVE,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
    );
}

/// xdg_positioner_set_parent_size
pub unsafe fn positioner_set_parent_size(
    positioner: *mut Positioner,
    parent_width: i32,
    parent_height: i32,
) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_PARENT_SIZE,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        parent_width,
        parent_height
    );
}

/// xdg_positioner_set_parent_configure
pub unsafe fn positioner_set_parent_configure(positioner: *mut Positioner, serial: u32) {
    proxy_marshal_flags!(
        positioner.cast(),
        POSITIONER_SET_PARENT_CONFIGURE,
        ptr::null(),
        proxy_get_version(positioner.cast()),
        0,
        serial
    );
}

/// struct xdg_surface
#[repr(C)]
pub struct XdgSurface {
    _opaque: [u8; 0],
}

/// xdg_surface_interface
pub static XDG_SURFACE_INTERFACE: Interface = Interface {
    name: b"xdg_surface\0".as_ptr().cast(),
    version: 4,
    method_count: 5,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"get_toplevel\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&TOPLEVEL_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"get_popup\0".as_ptr().cast(),
            signature: b"n?oo\0".as_ptr().cast(),
            types: [
                &POPUP_INTERFACE as *const Interface,
                &XDG_SURFACE_INTERFACE,
                &POSITIONER_INTERFACE,
            ]
            .as_ptr(),
        },
        Message {
            name: b"set_window_geometry\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"ack_configure\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 1,
    events: [Message {
        name: b"configure\0".as_ptr().cast(),
        signature: b"u\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
};

/// XDG_SURFACE_ERROR_NOT_CONSTRUCTED
pub const XDG_SURFACE_ERROR_NOT_CONSTRUCTED: c_int = 1;

/// XDG_SURFACE_ERROR_ALREADY_CONSTRUCTED
pub const XDG_SURFACE_ERROR_ALREADY_CONSTRUCTED: c_int = 2;

/// XDG_SURFACE_ERROR_UNCONFIGURED_BUFFER
pub const XDG_SURFACE_ERROR_UNCONFIGURED_BUFFER: c_int = 3;

/// struct xdg_surface_listener
#[repr(C)]
pub struct XdgSurfaceListener {
    pub configure:
        unsafe extern "C" fn(data: *mut c_void, xdg_surface: *mut XdgSurface, serial: u32),
}

/// xdg_surface_add_listener
pub unsafe fn xdg_surface_add_listener(
    xdg_surface: *mut XdgSurface,
    listener: *const XdgSurfaceListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(xdg_surface.cast(), listener as *mut _, data)
}

const XDG_SURFACE_DESTROY: u32 = 0;
const XDG_SURFACE_GET_TOPLEVEL: u32 = 1;
const XDG_SURFACE_GET_POPUP: u32 = 2;
const XDG_SURFACE_SET_WINDOW_GEOMETRY: u32 = 3;
const XDG_SURFACE_ACK_CONFIGURE: u32 = 4;

/// xdg_surface_destroy
pub unsafe fn xdg_surface_destroy(xdg_surface: *mut XdgSurface) {
    proxy_marshal_flags!(
        xdg_surface.cast(),
        XDG_SURFACE_DESTROY,
        ptr::null(),
        proxy_get_version(xdg_surface.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// xdg_surface_get_toplevel
pub unsafe fn xdg_surface_get_toplevel(xdg_surface: *mut XdgSurface) -> *mut Toplevel {
    proxy_marshal_flags!(
        xdg_surface.cast(),
        XDG_SURFACE_GET_TOPLEVEL,
        &TOPLEVEL_INTERFACE,
        proxy_get_version(xdg_surface.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// xdg_surface_get_popup
pub unsafe fn xdg_surface_get_popup(
    xdg_surface: *mut XdgSurface,
    parent: *mut XdgSurface,
    positioner: *mut Positioner,
) -> *mut Popup {
    proxy_marshal_flags!(
        xdg_surface.cast(),
        XDG_SURFACE_GET_POPUP,
        &POPUP_INTERFACE,
        proxy_get_version(xdg_surface.cast()),
        0,
        ptr::null::<usize>(), // XXX
        parent,
        positioner
    )
    .cast()
}

/// xdg_surface_set_window_geometry
pub unsafe fn xdg_surface_set_window_geometry(
    xdg_surface: *mut XdgSurface,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    proxy_marshal_flags!(
        xdg_surface.cast(),
        XDG_SURFACE_SET_WINDOW_GEOMETRY,
        ptr::null(),
        proxy_get_version(xdg_surface.cast()),
        0,
        x,
        y,
        width,
        height
    );
}

/// xdg_surface_ack_configure
pub unsafe fn xdg_surface_ack_configure(xdg_surface: *mut XdgSurface, serial: u32) {
    proxy_marshal_flags!(
        xdg_surface.cast(),
        XDG_SURFACE_ACK_CONFIGURE,
        ptr::null(),
        proxy_get_version(xdg_surface.cast()),
        0,
        serial
    );
}

/// struct xdg_toplevel
#[repr(C)]
pub struct Toplevel {
    _opaque: [u8; 0],
}

/// xdg_toplevel_interface
pub static TOPLEVEL_INTERFACE: Interface = Interface {
    name: b"xdg_toplevel\0".as_ptr().cast(),
    version: 4,
    method_count: 14,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_parent\0".as_ptr().cast(),
            signature: b"?o\0".as_ptr().cast(),
            types: [&TOPLEVEL_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"set_title\0".as_ptr().cast(),
            signature: b"s\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_app_id\0".as_ptr().cast(),
            signature: b"s\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"show_window_menu\0".as_ptr().cast(),
            signature: b"ouii\0".as_ptr().cast(),
            types: [&SEAT_INTERFACE, ptr::null(), ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: b"move\0".as_ptr().cast(),
            signature: b"ou\0".as_ptr().cast(),
            types: [&SEAT_INTERFACE, ptr::null()].as_ptr(),
        },
        Message {
            name: b"resize\0".as_ptr().cast(),
            signature: b"ouu\0".as_ptr().cast(),
            types: [&SEAT_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: b"set_max_size\0".as_ptr().cast(),
            signature: b"ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_min_size\0".as_ptr().cast(),
            signature: b"ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_maximized\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"unset_maximized\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_fullscreen\0".as_ptr().cast(),
            signature: b"?o\0".as_ptr().cast(),
            types: [&OUTPUT_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"unset_fullscreen\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_minimized\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 3,
    events: [
        Message {
            name: b"configure\0".as_ptr().cast(),
            signature: b"iia\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"close\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"configure_bounds\0".as_ptr().cast(),
            signature: b"4ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// XDG_TOPLEVEL_ERROR_INVALID_RESIZE_EDGE
pub const TOPLEVEL_ERROR_INVALID_RESIZE_EDGE: c_int = 0;

/// XDG_TOPLEVEL_RESIZE_EDGE_NONE
pub const TOPLEVEL_RESIZE_EDGE_NONE: u32 = 0;

/// XDG_TOPLEVEL_RESIZE_EDGE_TOP
pub const TOPLEVEL_RESIZE_EDGE_TOP: u32 = 1;

/// XDG_TOPLEVEL_RESIZE_EDGE_BOTTOM
pub const TOPLEVEL_RESIZE_EDGE_BOTTOM: u32 = 2;

/// XDG_TOPLEVEL_RESIZE_EDGE_LEFT
pub const TOPLEVEL_RESIZE_EDGE_LEFT: u32 = 4;

/// XDG_TOPLEVEL_RESIZE_EDGE_TOP_LEFT
pub const TOPLEVEL_RESIZE_EDGE_TOP_LEFT: u32 = 5;

/// XDG_TOPLEVEL_RESIZE_EDGE_BOTTOM_LEFT
pub const TOPLEVEL_RESIZE_EDGE_BOTTOM_LEFT: u32 = 6;

/// XDG_TOPLEVEL_RESIZE_EDGE_RIGHT
pub const TOPLEVEL_RESIZE_EDGE_RIGHT: u32 = 8;

/// XDG_TOPLEVEL_RESIZE_EDGE_TOP_RIGHT
pub const TOPLEVEL_RESIZE_EDGE_TOP_RIGHT: u32 = 9;

/// XDG_TOPLEVEL_RESIZE_EDGE_BOTTOM_RIGHT
pub const TOPLEVEL_RESIZE_EDGE_BOTTOM_RIGHT: u32 = 10;

/// XDG_TOPLEVEL_STATE_MAXIMIZED
pub const TOPLEVEL_STATE_MAXIMIZED: u32 = 1;

/// XDG_TOPLEVEL_STATE_FULLSCREEN
pub const TOPLEVEL_STATE_FULLSCREEN: u32 = 2;

/// XDG_TOPLEVEL_STATE_RESIZING
pub const TOPLEVEL_STATE_RESIZING: u32 = 3;

/// XDG_TOPLEVEL_STATE_ACTIVATED
pub const TOPLEVEL_STATE_ACTIVATED: u32 = 4;

/// XDG_TOPLEVEL_STATE_TILED_LEFT
pub const TOPLEVEL_STATE_TILED_LEFT: u32 = 5;

/// XDG_TOPLEVEL_STATE_TILED_RIGHT
pub const TOPLEVEL_STATE_TILED_RIGHT: u32 = 6;

/// XDG_TOPLEVEL_STATE_TILED_TOP
pub const TOPLEVEL_STATE_TILED_TOP: u32 = 7;

/// XDG_TOPLEVEL_STATE_TILED_BOTTOM
pub const TOPLEVEL_STATE_TILED_BOTTOM: u32 = 8;

/// struct xdg_toplevel_listener
#[repr(C)]
pub struct ToplevelListener {
    pub configure: unsafe extern "C" fn(
        data: *mut c_void,
        toplevel: *mut Toplevel,
        width: i32,
        height: i32,
        states: *mut c_void, // TODO: struct wl_array *
    ),
    pub close: unsafe extern "C" fn(data: *mut c_void, toplevel: *mut Toplevel),
    pub configure_bounds:
        unsafe extern "C" fn(data: *mut c_void, toplevel: *mut Toplevel, width: i32, height: i32),
}

/// xdg_toplevel_add_listener
pub unsafe fn toplevel_add_listener(
    toplevel: *mut Toplevel,
    listener: *const ToplevelListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(toplevel.cast(), listener as *mut _, data)
}

const TOPLEVEL_DESTROY: u32 = 0;
const TOPLEVEL_SET_PARENT: u32 = 1;
const TOPLEVEL_SET_TITLE: u32 = 2;
const TOPLEVEL_SET_APP_ID: u32 = 3;
const TOPLEVEL_SHOW_WINDOW_MENU: u32 = 4;
const TOPLEVEL_MOVE: u32 = 5;
const TOPLEVEL_RESIZE: u32 = 6;
const TOPLEVEL_SET_MAX_SIZE: u32 = 7;
const TOPLEVEL_SET_MIN_SIZE: u32 = 8;
const TOPLEVEL_SET_MAXIMIZED: u32 = 9;
const TOPLEVEL_UNSET_MAXIMIZED: u32 = 10;
const TOPLEVEL_SET_FULLSCREEN: u32 = 11;
const TOPLEVEL_UNSET_FULLSCREEN: u32 = 12;
const TOPLEVEL_SET_MINIMIZED: u32 = 13;

/// xdg_toplevel_destroy
pub unsafe fn toplevel_destroy(toplevel: *mut Toplevel) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_DESTROY,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// xdg_toplevel_set_parent
pub unsafe fn toplevel_set_parent(toplevel: *mut Toplevel, parent: *mut Toplevel) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_PARENT,
        &TOPLEVEL_INTERFACE,
        proxy_get_version(toplevel.cast()),
        0,
        parent
    );
}

/// xdg_toplevel_set_title
pub unsafe fn toplevel_set_title(toplevel: *mut Toplevel, title: *const c_char) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_TITLE,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        title
    );
}

/// xdg_toplevel_set_app_id
pub unsafe fn toplevel_set_app_id(toplevel: *mut Toplevel, app_id: *const c_char) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_APP_ID,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        app_id
    );
}

/// xdg_toplevel_show_window_menu
pub unsafe fn toplevel_show_window_menu(
    toplevel: *mut Toplevel,
    seat: *mut Seat,
    serial: u32,
    x: i32,
    y: i32,
) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SHOW_WINDOW_MENU,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        seat,
        serial,
        x,
        y
    );
}

/// xdg_toplevel_move
pub unsafe fn toplevel_move(toplevel: *mut Toplevel, seat: *mut Seat, serial: u32) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_MOVE,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        seat,
        serial
    );
}

/// xdg_toplevel_resize
pub unsafe fn toplevel_resize(toplevel: *mut Toplevel, seat: *mut Seat, serial: u32, edges: u32) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_RESIZE,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        seat,
        serial,
        edges
    );
}

/// xdg_toplevel_set_max_size
pub unsafe fn toplevel_set_max_size(toplevel: *mut Toplevel, width: i32, height: i32) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_MAX_SIZE,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        width,
        height
    );
}

/// xdg_toplevel_set_min_size
pub unsafe fn toplevel_set_min_size(toplevel: *mut Toplevel, width: i32, height: i32) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_MIN_SIZE,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        width,
        height
    );
}

/// xdg_toplevel_set_maximized
pub unsafe fn toplevel_set_maximized(toplevel: *mut Toplevel) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_MAXIMIZED,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
    );
}

/// xdg_toplevel_unset_maximized
pub unsafe fn toplevel_unset_maximized(toplevel: *mut Toplevel) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_UNSET_MAXIMIZED,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
    );
}

/// xdg_toplevel_set_fullscreen
pub unsafe fn toplevel_set_fullscreen(toplevel: *mut Toplevel, output: *mut Output) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_FULLSCREEN,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
        output
    );
}

/// xdg_toplevel_unset_fullscreen
pub unsafe fn toplevel_unset_fullscreen(toplevel: *mut Toplevel) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_UNSET_FULLSCREEN,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
    );
}

/// xdg_toplevel_set_minimized
pub unsafe fn toplevel_set_minimized(toplevel: *mut Toplevel) {
    proxy_marshal_flags!(
        toplevel.cast(),
        TOPLEVEL_SET_MINIMIZED,
        ptr::null(),
        proxy_get_version(toplevel.cast()),
        0,
    );
}

/// struct xdg_popup
#[repr(C)]
pub struct Popup {
    _opaque: [u8; 0],
}

/// xdg_popup_interface
pub static POPUP_INTERFACE: Interface = Interface {
    name: b"xdg_popup\0".as_ptr().cast(),
    version: 4,
    method_count: 3,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"grab\0".as_ptr().cast(),
            signature: b"ou\0".as_ptr().cast(),
            types: [&SEAT_INTERFACE, ptr::null()].as_ptr(),
        },
        Message {
            name: b"reposition\0".as_ptr().cast(),
            signature: b"3ou\0".as_ptr().cast(),
            types: [&POSITIONER_INTERFACE, ptr::null()].as_ptr(),
        },
    ]
    .as_ptr(),
    event_count: 3,
    events: [
        Message {
            name: b"configure\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"popup_done\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"repositioned\0".as_ptr().cast(),
            signature: b"3u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// XDG_POPUP_ERROR_INVALID_GRAB
pub const POPUP_ERROR_INVALID_GRAB: c_int = 0;

/// struct xdg_popup_listener
#[repr(C)]
pub struct PopupListener {
    pub configure: unsafe extern "C" fn(
        data: *mut c_void,
        popup: *mut Popup,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ),
    pub popup_done: unsafe extern "C" fn(data: *mut c_void, popup: *mut Popup),
    pub repositioned: unsafe extern "C" fn(data: *mut c_void, popup: *mut Popup, token: u32),
}

/// xdg_popup_add_listener
pub unsafe fn popup_add_listener(
    popup: *mut Popup,
    listener: *const PopupListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(popup.cast(), listener as *mut _, data)
}

const POPUP_DESTROY: u32 = 0;
const POPUP_GRAB: u32 = 1;
const POPUP_REPOSITION: u32 = 2;

/// xdg_popup_destroy
pub unsafe fn popup_destroy(popup: *mut Popup) {
    proxy_marshal_flags!(
        popup.cast(),
        POPUP_DESTROY,
        ptr::null(),
        proxy_get_version(popup.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// xdg_popup_grab
pub unsafe fn popup_grab(popup: *mut Popup, seat: *mut Seat, serial: u32) {
    proxy_marshal_flags!(
        popup.cast(),
        POPUP_GRAB,
        ptr::null(),
        proxy_get_version(popup.cast()),
        0,
        seat,
        serial
    );
}

/// xdg_popup_reposition
pub unsafe fn popup_reposition(popup: *mut Popup, positioner: *mut Positioner, token: u32) {
    proxy_marshal_flags!(
        popup.cast(),
        POPUP_REPOSITION,
        ptr::null(),
        proxy_get_version(popup.cast()),
        0,
        positioner,
        token
    );
}
