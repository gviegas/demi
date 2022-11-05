// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::{
    proxy_add_listener, proxy_get_version, Display, Interface, Message, MARSHAL_FLAG_DESTROY,
};

const NULL_TYPES: *const *const Interface = [ptr::null(); 8].as_ptr();

const DISPLAY_SYNC: u32 = 0;
const DISPLAY_GET_REGISTRY: u32 = 1;

/// wl_display_sync
pub unsafe fn display_sync(display: *mut Display) -> *mut Callback {
    proxy_marshal_flags!(
        display.cast(),
        DISPLAY_SYNC,
        &CALLBACK_INTERFACE,
        proxy_get_version(display.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// wl_display_get_registry
pub unsafe fn display_get_registry(display: *mut Display) -> *mut Registry {
    proxy_marshal_flags!(
        display.cast(),
        DISPLAY_GET_REGISTRY,
        &REGISTRY_INTERFACE,
        proxy_get_version(display.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// struct wl_callback
#[repr(C)]
pub struct Callback {
    _opaque: [u8; 0],
}

/// wl_callback_interface
pub static CALLBACK_INTERFACE: Interface = Interface {
    name: b"wl_callback\0".as_ptr().cast(),
    version: 1,
    method_count: 0,
    methods: ptr::null(),
    event_count: 1,
    events: [Message {
        name: b"done\0".as_ptr().cast(),
        signature: b"u\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
};

/// struct wl_callback_listener
#[repr(C)]
pub struct CallbackListener {
    pub done: unsafe extern "C" fn(data: *mut c_void, callback: *mut Callback, callback_data: u32),
}

/// wl_callback_add_listener
pub unsafe fn callback_add_listener(
    callback: *mut Callback,
    listener: *const CallbackListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(callback.cast(), listener as *mut _, data)
}

/// struct wl_registry
#[repr(C)]
pub struct Registry {
    _opaque: [u8; 0],
}

/// wl_registry_interface
pub static REGISTRY_INTERFACE: Interface = Interface {
    name: b"wl_registry\0".as_ptr().cast(),
    version: 1,
    method_count: 1,
    methods: [Message {
        name: b"bind\0".as_ptr().cast(),
        signature: b"usun\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 2,
    events: [
        Message {
            name: b"global\0".as_ptr().cast(),
            signature: b"usu\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"global_remove\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// struct wl_registry_listener
#[repr(C)]
pub struct RegistryListener {
    pub global: unsafe extern "C" fn(
        data: *mut c_void,
        registry: *mut Registry,
        name: u32,
        interface: *const c_char,
        version: u32,
    ),
    pub global_remove: unsafe extern "C" fn(data: *mut c_void, registry: *mut Registry, name: u32),
}

/// wl_registry_add_listener
pub unsafe fn registry_add_listener(
    registry: *mut Registry,
    listener: *const RegistryListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(registry.cast(), listener as *mut _, data)
}

const REGISTRY_BIND: u32 = 0;

/// wl_registry_bind
pub unsafe fn registry_bind(
    registry: *mut Registry,
    name: u32,
    interface: *const Interface,
    version: u32,
) -> *mut c_void {
    proxy_marshal_flags!(
        registry.cast(),
        REGISTRY_BIND,
        interface,
        version,
        0,
        name,
        (*interface).name,
        version,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// struct wl_compositor
#[repr(C)]
pub struct Compositor {
    _opaque: [u8; 0],
}

/// wl_compositor_interface
pub static COMPOSITOR_INTERFACE: Interface = Interface {
    name: b"wl_compositor\0".as_ptr().cast(),
    version: 5,
    method_count: 2,
    methods: [
        Message {
            name: b"create_surface\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&SURFACE_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"create_region\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&REGION_INTERFACE as *const Interface].as_ptr(),
        },
    ]
    .as_ptr(),
    event_count: 0,
    events: ptr::null(),
};

const COMPOSITOR_CREATE_SURFACE: u32 = 0;
const COMPOSITOR_CREATE_REGION: u32 = 1;

/// wl_compositor_create_surface
pub unsafe fn compositor_create_surface(compositor: *mut Compositor) -> *mut Surface {
    proxy_marshal_flags!(
        compositor.cast(),
        COMPOSITOR_CREATE_SURFACE,
        &SURFACE_INTERFACE,
        proxy_get_version(compositor.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// wl_compositor_create_region
pub unsafe fn compositor_create_region(compositor: *mut Compositor) -> *mut Region {
    proxy_marshal_flags!(
        compositor.cast(),
        COMPOSITOR_CREATE_REGION,
        &REGION_INTERFACE,
        proxy_get_version(compositor.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// struct wl_surface
#[repr(C)]
pub struct Surface {
    _opaque: [u8; 0],
}

/// wl_surface_interface
pub static SURFACE_INTERFACE: Interface = Interface {
    name: b"wl_surface\0".as_ptr().cast(),
    version: 5,
    method_count: 11,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"attach\0".as_ptr().cast(),
            signature: b"?oii\0".as_ptr().cast(),
            types: [&BUFFER_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: b"damage\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"frame\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&CALLBACK_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"set_opaque_region\0".as_ptr().cast(),
            signature: b"?o\0".as_ptr().cast(),
            types: [&REGION_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"set_input_region\0".as_ptr().cast(),
            signature: b"?o\0".as_ptr().cast(),
            types: [&REGION_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"commit\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_buffer_transform\0".as_ptr().cast(),
            signature: b"2i\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"set_buffer_scale\0".as_ptr().cast(),
            signature: b"3i\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"damage_buffer\0".as_ptr().cast(),
            signature: b"4iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"offset\0".as_ptr().cast(),
            signature: b"5ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 2,
    events: [
        Message {
            name: b"enter\0".as_ptr().cast(),
            signature: b"o\0".as_ptr().cast(),
            types: [&OUTPUT_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"leave\0".as_ptr().cast(),
            signature: b"o\0".as_ptr().cast(),
            types: [&OUTPUT_INTERFACE as *const Interface].as_ptr(),
        },
    ]
    .as_ptr(),
};

/// struct wl_surface_listener
#[repr(C)]
pub struct SurfaceListener {
    pub enter: unsafe extern "C" fn(data: *mut c_void, surface: *mut Surface, output: *mut Output),
    pub leave: unsafe extern "C" fn(data: *mut c_void, surface: *mut Surface, output: *mut Output),
}

/// wl_surface_add_listener
pub unsafe fn surface_add_listener(
    surface: *mut Surface,
    listener: *const SurfaceListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(surface.cast(), listener as *mut _, data)
}

const SURFACE_DESTROY: u32 = 0;
const SURFACE_ATTACH: u32 = 1;
const SURFACE_DAMAGE: u32 = 2;
const SURFACE_FRAME: u32 = 3;
const SURFACE_SET_OPAQUE_REGION: u32 = 4;
const SURFACE_SET_INPUT_REGION: u32 = 5;
const SURFACE_COMMIT: u32 = 6;
const SURFACE_SET_BUFFER_TRANSFORM: u32 = 7;
const SURFACE_SET_BUFFER_SCALE: u32 = 8;
const SURFACE_DAMAGE_BUFFER: u32 = 9;
const SURFACE_OFFSET: u32 = 10;

/// wl_surface_destroy
pub unsafe fn surface_destroy(surface: *mut Surface) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_DESTROY,
        ptr::null(),
        proxy_get_version(surface.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// wl_surface_attach
pub unsafe fn surface_attach(surface: *mut Surface, buffer: *mut Buffer, x: i32, y: i32) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_ATTACH,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        buffer,
        x,
        y
    );
}

/// wl_surface_damage
pub unsafe fn surface_damage(surface: *mut Surface, x: i32, y: i32, width: i32, height: i32) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_DAMAGE,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        x,
        y,
        width,
        height
    );
}

/// wl_surface_frame
pub unsafe fn surface_frame(surface: *mut Surface) -> *mut Callback {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_FRAME,
        &CALLBACK_INTERFACE,
        proxy_get_version(surface.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// wl_surface_set_opaque_region
pub unsafe fn surface_set_opaque_region(surface: *mut Surface, region: *mut Region) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_SET_OPAQUE_REGION,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        region
    );
}

/// wl_surface_set_input_region
pub unsafe fn surface_set_input_region(surface: *mut Surface, region: *mut Region) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_SET_INPUT_REGION,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        region
    );
}

/// wl_surface_commit
pub unsafe fn surface_commit(surface: *mut Surface) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_COMMIT,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
    );
}

/// wl_surface_set_buffer_transform
pub unsafe fn surface_set_buffer_transform(surface: *mut Surface, transform: i32) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_SET_BUFFER_TRANSFORM,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        transform
    );
}

/// wl_surface_set_buffer_scale
pub unsafe fn surface_set_buffer_scale(surface: *mut Surface, scale: i32) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_SET_BUFFER_SCALE,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        scale
    );
}

/// wl_surface_damage_buffer
pub unsafe fn surface_damage_buffer(
    surface: *mut Surface,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_DAMAGE_BUFFER,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        x,
        y,
        width,
        height
    );
}

/// wl_surface_offset
pub unsafe fn surface_offset(surface: *mut Surface, x: i32, y: i32) {
    proxy_marshal_flags!(
        surface.cast(),
        SURFACE_OFFSET,
        ptr::null(),
        proxy_get_version(surface.cast()),
        0,
        x,
        y
    );
}

/// struct wl_region
#[repr(C)]
pub struct Region {
    _opaque: [u8; 0],
}

/// wl_region_interface
pub static REGION_INTERFACE: Interface = Interface {
    name: b"wl_region\0".as_ptr().cast(),
    version: 1,
    method_count: 3,
    methods: [
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"add\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"subtract\0".as_ptr().cast(),
            signature: b"iiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 0,
    events: ptr::null(),
};

const REGION_DESTROY: u32 = 0;
const REGION_ADD: u32 = 1;
const REGION_SUBTRACT: u32 = 2;

/// wl_region_destroy
pub unsafe fn region_destroy(region: *mut Region) {
    proxy_marshal_flags!(
        region.cast(),
        REGION_DESTROY,
        ptr::null(),
        proxy_get_version(region.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// wl_region_add
pub unsafe fn region_add(region: *mut Region, x: i32, y: i32, width: i32, height: i32) {
    proxy_marshal_flags!(
        region.cast(),
        REGION_ADD,
        ptr::null(),
        proxy_get_version(region.cast()),
        0,
        x,
        y,
        width,
        height
    );
}

/// wl_region_subtract
pub unsafe fn region_subtract(region: *mut Region, x: i32, y: i32, width: i32, height: i32) {
    proxy_marshal_flags!(
        region.cast(),
        REGION_SUBTRACT,
        ptr::null(),
        proxy_get_version(region.cast()),
        0,
        x,
        y,
        width,
        height
    );
}

/// struct wl_shm
#[repr(C)]
pub struct Shm {
    _opaque: [u8; 0],
}

/// wl_shm_interface
pub static SHM_INTERFACE: Interface = Interface {
    name: b"wl_shm\0".as_ptr().cast(),
    version: 1,
    method_count: 1,
    methods: [Message {
        name: b"create_pool\0".as_ptr().cast(),
        signature: b"nhi\0".as_ptr().cast(),
        types: [&SHM_POOL_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
    }]
    .as_ptr(),
    event_count: 1,
    events: [Message {
        name: b"format\0".as_ptr().cast(),
        signature: b"u\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
};

/// struct wl_shm_listener
#[repr(C)]
pub struct ShmListener {
    pub format: unsafe extern "C" fn(data: *mut c_void, shm: *mut Shm, format: u32),
}

/// wl_shm_add_listener
pub unsafe fn shm_add_listener(
    shm: *mut Shm,
    listener: *const ShmListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(shm.cast(), listener as *mut _, data)
}

const SHM_CREATE_POOL: u32 = 0;

/// wl_shm_create_pool
pub unsafe fn shm_create_pool(shm: *mut Shm, fd: i32, size: i32) -> *mut ShmPool {
    proxy_marshal_flags!(
        shm.cast(),
        SHM_CREATE_POOL,
        &SHM_POOL_INTERFACE,
        proxy_get_version(shm.cast()),
        0,
        ptr::null::<usize>(), // XXX
        fd,
        size
    )
    .cast()
}

/// struct wl_shm_pool
#[repr(C)]
pub struct ShmPool {
    _opaque: [u8; 0],
}

/// wl_shm_pool_interface
pub static SHM_POOL_INTERFACE: Interface = Interface {
    name: b"wl_shm_pool\0".as_ptr().cast(),
    version: 1,
    method_count: 3,
    methods: [
        Message {
            name: b"create_buffer\0".as_ptr().cast(),
            signature: b"niiiiu\0".as_ptr().cast(),
            types: [
                &BUFFER_INTERFACE,
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
            ]
            .as_ptr(),
        },
        Message {
            name: b"destroy\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"resize\0".as_ptr().cast(),
            signature: b"i\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 0,
    events: ptr::null(),
};

const SHM_POOL_CREATE_BUFFER: u32 = 0;
const SHM_POOL_DESTROY: u32 = 1;
const SHM_POOL_RESIZE: u32 = 2;

/// wl_shm_pool_create_buffer
pub unsafe fn shm_pool_create_buffer(
    shm_pool: *mut ShmPool,
    offset: i32,
    width: i32,
    height: i32,
    stride: i32,
    format: u32,
) -> *mut Buffer {
    proxy_marshal_flags!(
        shm_pool.cast(),
        SHM_POOL_CREATE_BUFFER,
        &BUFFER_INTERFACE,
        proxy_get_version(shm_pool.cast()),
        0,
        ptr::null::<usize>(), // XXX
        offset,
        width,
        height,
        stride,
        format
    )
    .cast()
}

/// wl_shm_pool_destroy
pub unsafe fn shm_pool_destroy(shm_pool: *mut ShmPool) {
    proxy_marshal_flags!(
        shm_pool.cast(),
        SHM_POOL_DESTROY,
        ptr::null(),
        proxy_get_version(shm_pool.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// wl_shm_pool_resize
pub unsafe fn shm_pool_resize(shm_pool: *mut ShmPool, size: i32) {
    proxy_marshal_flags!(
        shm_pool.cast(),
        SHM_POOL_RESIZE,
        ptr::null(),
        proxy_get_version(shm_pool.cast()),
        0,
        size
    );
}

/// struct wl_buffer
#[repr(C)]
pub struct Buffer {
    _opaque: [u8; 0],
}

/// wl_buffer_interface
pub static BUFFER_INTERFACE: Interface = Interface {
    name: b"wl_buffer\0".as_ptr().cast(),
    version: 1,
    method_count: 1,
    methods: [Message {
        name: b"destroy\0".as_ptr().cast(),
        signature: b"\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 1,
    events: [Message {
        name: b"release\0".as_ptr().cast(),
        signature: b"\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
};

/// struct wl_buffer_listener
#[repr(C)]
pub struct BufferListener {
    pub release: unsafe extern "C" fn(data: *mut c_void, buffer: *mut Buffer),
}

/// wl_buffer_add_listener
pub unsafe fn buffer_add_listener(
    buffer: *mut Buffer,
    listener: *const BufferListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(buffer.cast(), listener as *mut _, data)
}

const BUFFER_DESTROY: u32 = 0;

/// wl_buffer_destroy
pub unsafe fn buffer_destroy(buffer: *mut Buffer) {
    proxy_marshal_flags!(
        buffer.cast(),
        BUFFER_DESTROY,
        ptr::null(),
        proxy_get_version(buffer.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// struct wl_seat
#[repr(C)]
pub struct Seat {
    _opaque: [u8; 0],
}

/// wl_seat_interface
pub static SEAT_INTERFACE: Interface = Interface {
    name: b"wl_seat\0".as_ptr().cast(),
    version: 7,
    method_count: 4,
    methods: [
        Message {
            name: b"get_pointer\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&POINTER_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"get_keyboard\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&KEYBOARD_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"get_touch\0".as_ptr().cast(),
            signature: b"n\0".as_ptr().cast(),
            types: [&TOUCH_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: b"release\0".as_ptr().cast(),
            signature: b"5\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 2,
    events: [
        Message {
            name: b"capabilities\0".as_ptr().cast(),
            signature: b"u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"name\0".as_ptr().cast(),
            signature: b"2s\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// struct wl_seat_listener
#[repr(C)]
pub struct SeatListener {
    pub capabilities: unsafe extern "C" fn(data: *mut c_void, seat: *mut Seat, capabilities: u32),
    pub name: unsafe extern "C" fn(data: *mut c_void, seat: *mut Seat, name: *const c_char),
}

/// wl_seat_add_listener
pub unsafe fn seat_add_listener(
    seat: *mut Seat,
    listener: *const SeatListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(seat.cast(), listener as *mut _, data)
}

const SEAT_GET_POINTER: u32 = 0;
const SEAT_GET_KEYBOARD: u32 = 1;
const SEAT_GET_TOUCH: u32 = 2;
const SEAT_RELEASE: u32 = 3;

/// wl_seat_get_pointer
pub unsafe fn seat_get_pointer(seat: *mut Seat) -> *mut Pointer {
    proxy_marshal_flags!(
        seat.cast(),
        SEAT_GET_POINTER,
        &POINTER_INTERFACE,
        proxy_get_version(seat.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// wl_seat_get_keyboard
pub unsafe fn seat_get_keyboard(seat: *mut Seat) -> *mut Keyboard {
    proxy_marshal_flags!(
        seat.cast(),
        SEAT_GET_KEYBOARD,
        &KEYBOARD_INTERFACE,
        proxy_get_version(seat.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// wl_seat_get_touch
pub unsafe fn seat_get_touch(seat: *mut Seat) -> *mut Touch {
    proxy_marshal_flags!(
        seat.cast(),
        SEAT_GET_TOUCH,
        &TOUCH_INTERFACE,
        proxy_get_version(seat.cast()),
        0,
        ptr::null::<usize>() // XXX
    )
    .cast()
}

/// wl_seat_release
pub unsafe fn seat_release(seat: *mut Seat) {
    proxy_marshal_flags!(
        seat.cast(),
        SEAT_RELEASE,
        ptr::null(),
        proxy_get_version(seat.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// struct wl_pointer
#[repr(C)]
pub struct Pointer {
    _opaque: [u8; 0],
}

/// wl_pointer_interface
pub static POINTER_INTERFACE: Interface = Interface {
    name: b"wl_pointer\0".as_ptr().cast(),
    version: 7,
    method_count: 2,
    methods: [
        Message {
            name: b"set_cursor\0".as_ptr().cast(),
            signature: b"u?oii\0".as_ptr().cast(),
            types: [ptr::null(), &SURFACE_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: b"release\0".as_ptr().cast(),
            signature: b"3\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 9,
    events: [
        Message {
            name: b"enter\0".as_ptr().cast(),
            signature: b"uoff\0".as_ptr().cast(),
            types: [ptr::null(), &SURFACE_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: b"leave\0".as_ptr().cast(),
            signature: b"uo\0".as_ptr().cast(),
            types: [ptr::null(), &SURFACE_INTERFACE].as_ptr(),
        },
        Message {
            name: b"motion\0".as_ptr().cast(),
            signature: b"uff\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"button\0".as_ptr().cast(),
            signature: b"uuuu\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"axis\0".as_ptr().cast(),
            signature: b"uuf\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"frame\0".as_ptr().cast(),
            signature: b"5\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"axis_source\0".as_ptr().cast(),
            signature: b"5u\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"axis_stop\0".as_ptr().cast(),
            signature: b"5uu\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"axis_discrete\0".as_ptr().cast(),
            signature: b"5ui\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// struct wl_pointer_listener
#[repr(C)]
pub struct PointerListener {
    pub enter: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: *mut Pointer,
        serial: u32,
        surface: *mut Surface,
        surface_x: i32, // 24.8
        surface_y: i32, // 24.8
    ),
    pub leave: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: *mut Pointer,
        serial: u32,
        surface: *mut Surface,
    ),
    pub motion: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: *mut Pointer,
        time: u32,
        surface_x: i32, // 24.8
        surface_y: i32, // 24.8
    ),
    pub button: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: *mut Pointer,
        serial: u32,
        time: u32,
        button: u32,
        state: u32,
    ),
    pub axis: unsafe extern "C" fn(
        data: *mut c_void,
        pointer: *mut Pointer,
        time: u32,
        axis: u32,
        value: i32, // 24.8
    ),
    pub frame: unsafe extern "C" fn(data: *mut c_void, pointer: *mut Pointer),
    pub axis_source:
        unsafe extern "C" fn(data: *mut c_void, pointer: *mut Pointer, axis_source: u32),
    pub axis_stop:
        unsafe extern "C" fn(data: *mut c_void, pointer: *mut Pointer, time: u32, axis: u32),
    pub axis_discrete:
        unsafe extern "C" fn(data: *mut c_void, pointer: *mut Pointer, axis: u32, discrete: i32),
}

/// wl_pointer_add_listener
pub unsafe fn pointer_add_listener(
    pointer: *mut Pointer,
    listener: *const PointerListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(pointer.cast(), listener as *mut _, data)
}

const POINTER_SET_CURSOR: u32 = 0;
const POINTER_RELEASE: u32 = 1;

/// wl_pointer_set_cursor
pub unsafe fn pointer_set_cursor(
    pointer: *mut Pointer,
    serial: u32,
    surface: *mut Surface,
    hotspot_x: i32,
    hotspot_y: i32,
) {
    proxy_marshal_flags!(
        pointer.cast(),
        POINTER_SET_CURSOR,
        ptr::null(),
        proxy_get_version(pointer.cast()),
        0,
        serial,
        surface,
        hotspot_x,
        hotspot_y
    );
}

/// wl_pointer_release
pub unsafe fn pointer_release(pointer: *mut Pointer) {
    proxy_marshal_flags!(
        pointer.cast(),
        POINTER_RELEASE,
        ptr::null(),
        proxy_get_version(pointer.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// struct wl_keyboard
#[repr(C)]
pub struct Keyboard {
    _opaque: [u8; 0],
}

/// wl_keyboard_interface
pub static KEYBOARD_INTERFACE: Interface = Interface {
    name: b"wl_keyboard\0".as_ptr().cast(),
    version: 7,
    method_count: 1,
    methods: [Message {
        name: b"release\0".as_ptr().cast(),
        signature: b"3\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 6,
    events: [
        Message {
            name: b"keymap\0".as_ptr().cast(),
            signature: b"uhu\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"enter\0".as_ptr().cast(),
            signature: b"uoa\0".as_ptr().cast(),
            types: [ptr::null(), &SURFACE_INTERFACE, ptr::null()].as_ptr(),
        },
        Message {
            name: b"leave\0".as_ptr().cast(),
            signature: b"uo\0".as_ptr().cast(),
            types: [ptr::null(), &SURFACE_INTERFACE].as_ptr(),
        },
        Message {
            name: b"key\0".as_ptr().cast(),
            signature: b"uuuu\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"modifiers\0".as_ptr().cast(),
            signature: b"uuuuu\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"repeat_info\0".as_ptr().cast(),
            signature: b"4ii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// struct wl_keyboard_listener
#[repr(C)]
pub struct KeyboardListener {
    pub keymap: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: *mut Keyboard,
        format: u32,
        fd: i32,
        size: u32,
    ),
    pub enter: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: *mut Keyboard,
        serial: u32,
        surface: *mut Surface,
        keys: *mut c_void, // TODO
    ),
    pub leave: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: *mut Keyboard,
        serial: u32,
        surface: *mut Surface,
    ),
    pub key: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: *mut Keyboard,
        serial: u32,
        time: u32,
        key: u32,
        state: u32,
    ),
    pub modifiers: unsafe extern "C" fn(
        data: *mut c_void,
        keyboard: *mut Keyboard,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ),
    pub repeat_info:
        unsafe extern "C" fn(data: *mut c_void, keyboard: *mut Keyboard, rate: i32, delay: i32),
}

/// wl_keyboard_add_listener
pub unsafe fn keyboard_add_listener(
    keyboard: *mut Keyboard,
    listener: *const KeyboardListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(keyboard.cast(), listener as *mut _, data)
}

const KEYBOARD_RELEASE: u32 = 0;

/// wl_keyboard_release
pub unsafe fn keyboard_release(keyboard: *mut Keyboard) {
    proxy_marshal_flags!(
        keyboard.cast(),
        KEYBOARD_RELEASE,
        ptr::null(),
        proxy_get_version(keyboard.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// struct wl_touch
#[repr(C)]
pub struct Touch {
    _opaque: [u8; 0],
}

/// wl_touch_interface
pub static TOUCH_INTERFACE: Interface = Interface {
    name: b"wl_touch\0".as_ptr().cast(),
    version: 7,
    method_count: 1,
    methods: [Message {
        name: b"release\0".as_ptr().cast(),
        signature: b"3\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 7,
    events: [
        Message {
            name: b"down\0".as_ptr().cast(),
            signature: b"uuoiff\0".as_ptr().cast(),
            types: [
                ptr::null(),
                ptr::null(),
                &SURFACE_INTERFACE,
                ptr::null(),
                ptr::null(),
                ptr::null(),
            ]
            .as_ptr(),
        },
        Message {
            name: b"up\0".as_ptr().cast(),
            signature: b"uui\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"motion\0".as_ptr().cast(),
            signature: b"uiff\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"frame\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"cancel\0".as_ptr().cast(),
            signature: b"\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"shape\0".as_ptr().cast(),
            signature: b"6iff\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"orientation\0".as_ptr().cast(),
            signature: b"6if\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// struct wl_touch_listener
#[repr(C)]
pub struct TouchListener {
    pub down: unsafe extern "C" fn(
        data: *mut c_void,
        touch: *mut Touch,
        serial: u32,
        time: u32,
        surface: *mut Surface,
        x: i32, // 24.8
        y: i32, // 24.8
    ),
    pub up:
        unsafe extern "C" fn(data: *mut c_void, touch: *mut Touch, serial: u32, time: u32, id: i32),
    pub motion: unsafe extern "C" fn(
        data: *mut c_void,
        touch: *mut Touch,
        time: u32,
        id: i32,
        x: i32, // 24.8
        y: i32, // 24.8
    ),
    pub frame: unsafe extern "C" fn(data: *mut c_void, touch: *mut Touch),
    pub cancel: unsafe extern "C" fn(data: *mut c_void, touch: *mut Touch),
    pub shape: unsafe extern "C" fn(
        data: *mut c_void,
        touch: *mut Touch,
        id: i32,
        major: i32, // 24.8
        minor: i32, // 24.8
    ),
    pub orientation: unsafe extern "C" fn(
        data: *mut c_void,
        touch: *mut Touch,
        id: i32,
        orientation: i32, // 24.8
    ),
}

/// wl_touch_add_listener
pub unsafe fn touch_add_listener(
    touch: *mut Touch,
    listener: *const TouchListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(touch.cast(), listener as *mut _, data)
}

const TOUCH_RELEASE: u32 = 0;

/// wl_touch_release
pub unsafe fn touch_release(touch: *mut Touch) {
    proxy_marshal_flags!(
        touch.cast(),
        TOUCH_RELEASE,
        ptr::null(),
        proxy_get_version(touch.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}

/// struct wl_output
#[repr(C)]
pub struct Output {
    _opaque: [u8; 0],
}

/// wl_output_interface
pub static OUTPUT_INTERFACE: Interface = Interface {
    name: b"wl_output\0".as_ptr().cast(),
    version: 4,
    method_count: 1,
    methods: [Message {
        name: b"release\0".as_ptr().cast(),
        signature: b"3\0".as_ptr().cast(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 6,
    events: [
        Message {
            name: b"geometry\0".as_ptr().cast(),
            signature: b"iiiiissi\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"mode\0".as_ptr().cast(),
            signature: b"uiii\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"done\0".as_ptr().cast(),
            signature: b"2\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"scale\0".as_ptr().cast(),
            signature: b"2i\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"name\0".as_ptr().cast(),
            signature: b"4s\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
        Message {
            name: b"description\0".as_ptr().cast(),
            signature: b"4s\0".as_ptr().cast(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// struct wl_output_listener
#[repr(C)]
pub struct OutputListener {
    pub geometry: unsafe extern "C" fn(
        data: *mut c_void,
        output: *mut Output,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: i32,
        make: *const c_char,
        model: *const c_char,
        transform: i32,
    ),
    pub mode: unsafe extern "C" fn(
        data: *mut c_void,
        output: *mut Output,
        flags: u32,
        width: i32,
        height: i32,
        refresh: i32,
    ),
    pub done: unsafe extern "C" fn(data: *mut c_void, output: *mut Output),
    pub scale: unsafe extern "C" fn(data: *mut c_void, output: *mut Output, factor: i32),
    pub name: unsafe extern "C" fn(data: *mut c_void, output: *mut Output, name: *const c_char),
    pub description:
        unsafe extern "C" fn(data: *mut c_void, output: *mut Output, description: *const c_char),
}

/// wl_output_add_listener
pub unsafe fn output_add_listener(
    output: *mut Output,
    listener: *const OutputListener,
    data: *mut c_void,
) -> c_int {
    proxy_add_listener(output.cast(), listener as *mut _, data)
}

const OUTPUT_RELEASE: u32 = 0;

/// wl_output_release
pub unsafe fn output_release(output: *mut Output) {
    proxy_marshal_flags!(
        output.cast(),
        OUTPUT_RELEASE,
        ptr::null(),
        proxy_get_version(output.cast()),
        MARSHAL_FLAG_DESTROY,
    );
}
