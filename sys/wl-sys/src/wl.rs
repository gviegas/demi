use std::ffi::{c_char, c_int, c_void};
use std::ptr;

use crate::{
    proxy_add_listener, proxy_get_version, Display, Interface, Message, MARSHAL_FLAG_DESTROY,
};

const NULL_TYPES: *const *const Interface = [ptr::null(); 8].as_ptr();

/// WL_DISPLAY_ERROR_INVALID_OBJECT
pub const DISPLAY_ERROR_INVALID_OBJECT: c_int = 0;

/// WL_DISPLAY_ERROR_INVALID_METHOD
pub const DISPLAY_ERROR_INVALID_METHOD: c_int = 1;

/// WL_DISPLAY_ERROR_NO_MEMORY
pub const DISPLAY_ERROR_NO_MEMORY: c_int = 2;

/// WL_DISPLAY_ERROR_IMPLEMENTATION
pub const DISPLAY_ERROR_IMPLEMENTATION: c_int = 3;

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
    name: c"wl_callback".as_ptr(),
    version: 1,
    method_count: 0,
    methods: ptr::null(),
    event_count: 1,
    events: [Message {
        name: c"done".as_ptr(),
        signature: c"u".as_ptr(),
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
    name: c"wl_registry".as_ptr(),
    version: 1,
    method_count: 1,
    methods: [Message {
        name: c"bind".as_ptr(),
        signature: c"usun".as_ptr(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 2,
    events: [
        Message {
            name: c"global".as_ptr(),
            signature: c"usu".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"global_remove".as_ptr(),
            signature: c"u".as_ptr(),
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
    name: c"wl_compositor".as_ptr(),
    version: 5,
    method_count: 2,
    methods: [
        Message {
            name: c"create_surface".as_ptr(),
            signature: c"n".as_ptr(),
            types: [&SURFACE_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"create_region".as_ptr(),
            signature: c"n".as_ptr(),
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
    name: c"wl_surface".as_ptr(),
    version: 5,
    method_count: 11,
    methods: [
        Message {
            name: c"destroy".as_ptr(),
            signature: c"".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"attach".as_ptr(),
            signature: c"?oii".as_ptr(),
            types: [&BUFFER_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: c"damage".as_ptr(),
            signature: c"iiii".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"frame".as_ptr(),
            signature: c"n".as_ptr(),
            types: [&CALLBACK_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"set_opaque_region".as_ptr(),
            signature: c"?o".as_ptr(),
            types: [&REGION_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"set_input_region".as_ptr(),
            signature: c"?o".as_ptr(),
            types: [&REGION_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"commit".as_ptr(),
            signature: c"".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"set_buffer_transform".as_ptr(),
            signature: c"2i".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"set_buffer_scale".as_ptr(),
            signature: c"3i".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"damage_buffer".as_ptr(),
            signature: c"4iiii".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"offset".as_ptr(),
            signature: c"5ii".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 2,
    events: [
        Message {
            name: c"enter".as_ptr(),
            signature: c"o".as_ptr(),
            types: [&OUTPUT_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"leave".as_ptr(),
            signature: c"o".as_ptr(),
            types: [&OUTPUT_INTERFACE as *const Interface].as_ptr(),
        },
    ]
    .as_ptr(),
};

/// WL_SURFACE_ERROR_INVALID_SCALE
pub const SURFACE_ERROR_INVALID_SCALE: c_int = 0;

/// WL_SURFACE_ERROR_INVALID_TRANSFORM
pub const SURFACE_ERROR_INVALID_TRANSFORM: c_int = 1;

/// WL_SURFACE_ERROR_INVALID_SIZE
pub const SURFACE_ERROR_INVALID_SIZE: c_int = 2;

/// WL_SURFACE_ERROR_INVALID_OFFSET
pub const SURFACE_ERROR_INVALID_OFFSET: c_int = 3;

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
    name: c"wl_region".as_ptr(),
    version: 1,
    method_count: 3,
    methods: [
        Message {
            name: c"destroy".as_ptr(),
            signature: c"".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"add".as_ptr(),
            signature: c"iiii".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"subtract".as_ptr(),
            signature: c"iiii".as_ptr(),
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
    name: c"wl_shm".as_ptr(),
    version: 1,
    method_count: 1,
    methods: [Message {
        name: c"create_pool".as_ptr(),
        signature: c"nhi".as_ptr(),
        types: [&SHM_POOL_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
    }]
    .as_ptr(),
    event_count: 1,
    events: [Message {
        name: c"format".as_ptr(),
        signature: c"u".as_ptr(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
};

/// WL_SHM_ERROR_INVALID_FORMAT
pub const SHM_ERROR_INVALID_FORMAT: c_int = 0;

/// WL_SHM_ERROR_INVALID_STRIDE
pub const SHM_ERROR_INVALID_STRIDE: c_int = 1;

/// WL_SHM_ERROR_INVALID_FD
pub const SHM_ERROR_INVALID_FD: c_int = 2;

/// WL_SHM_FORMAT_ARGB8888
pub const SHM_FORMAT_ARGB8888: u32 = 0;

/// WL_SHM_FORMAT_XRGB8888
pub const SHM_FORMAT_XRGB8888: u32 = 1;

/// WL_SHM_FORMAT_C8
pub const SHM_FORMAT_C8: u32 = 0x20203843;

/// WL_SHM_FORMAT_RGB332
pub const SHM_FORMAT_RGB332: u32 = 0x38424752;

/// WL_SHM_FORMAT_BGR233
pub const SHM_FORMAT_BGR233: u32 = 0x38524742;

/// WL_SHM_FORMAT_XRGB4444
pub const SHM_FORMAT_XRGB4444: u32 = 0x32315258;

/// WL_SHM_FORMAT_XBGR4444
pub const SHM_FORMAT_XBGR4444: u32 = 0x32314258;

/// WL_SHM_FORMAT_RGBX4444
pub const SHM_FORMAT_RGBX4444: u32 = 0x32315852;

/// WL_SHM_FORMAT_BGRX4444
pub const SHM_FORMAT_BGRX4444: u32 = 0x32315842;

/// WL_SHM_FORMAT_ARGB4444
pub const SHM_FORMAT_ARGB4444: u32 = 0x32315241;

/// WL_SHM_FORMAT_ABGR4444
pub const SHM_FORMAT_ABGR4444: u32 = 0x32314241;

/// WL_SHM_FORMAT_RGBA4444
pub const SHM_FORMAT_RGBA4444: u32 = 0x32314152;

/// WL_SHM_FORMAT_BGRA4444
pub const SHM_FORMAT_BGRA4444: u32 = 0x32314142;

/// WL_SHM_FORMAT_XRGB1555
pub const SHM_FORMAT_XRGB1555: u32 = 0x35315258;

/// WL_SHM_FORMAT_XBGR1555
pub const SHM_FORMAT_XBGR1555: u32 = 0x35314258;

/// WL_SHM_FORMAT_RGBX5551
pub const SHM_FORMAT_RGBX5551: u32 = 0x35315852;

/// WL_SHM_FORMAT_BGRX5551
pub const SHM_FORMAT_BGRX5551: u32 = 0x35315842;

/// WL_SHM_FORMAT_ARGB1555
pub const SHM_FORMAT_ARGB1555: u32 = 0x35315241;

/// WL_SHM_FORMAT_ABGR1555
pub const SHM_FORMAT_ABGR1555: u32 = 0x35314241;

/// WL_SHM_FORMAT_RGBA5551
pub const SHM_FORMAT_RGBA5551: u32 = 0x35314152;

/// WL_SHM_FORMAT_BGRA5551
pub const SHM_FORMAT_BGRA5551: u32 = 0x35314142;

/// WL_SHM_FORMAT_RGB565
pub const SHM_FORMAT_RGB565: u32 = 0x36314752;

/// WL_SHM_FORMAT_BGR565
pub const SHM_FORMAT_BGR565: u32 = 0x36314742;

/// WL_SHM_FORMAT_RGB888
pub const SHM_FORMAT_RGB888: u32 = 0x34324752;

/// WL_SHM_FORMAT_BGR888
pub const SHM_FORMAT_BGR888: u32 = 0x34324742;

/// WL_SHM_FORMAT_XBGR8888
pub const SHM_FORMAT_XBGR8888: u32 = 0x34324258;

/// WL_SHM_FORMAT_RGBX8888
pub const SHM_FORMAT_RGBX8888: u32 = 0x34325852;

/// WL_SHM_FORMAT_BGRX8888
pub const SHM_FORMAT_BGRX8888: u32 = 0x34325842;

/// WL_SHM_FORMAT_ABGR8888
pub const SHM_FORMAT_ABGR8888: u32 = 0x34324241;

/// WL_SHM_FORMAT_RGBA8888
pub const SHM_FORMAT_RGBA8888: u32 = 0x34324152;

/// WL_SHM_FORMAT_BGRA8888
pub const SHM_FORMAT_BGRA8888: u32 = 0x34324142;

/// WL_SHM_FORMAT_XRGB2101010
pub const SHM_FORMAT_XRGB2101010: u32 = 0x30335258;

/// WL_SHM_FORMAT_XBGR2101010
pub const SHM_FORMAT_XBGR2101010: u32 = 0x30334258;

/// WL_SHM_FORMAT_RGBX1010102
pub const SHM_FORMAT_RGBX1010102: u32 = 0x30335852;

/// WL_SHM_FORMAT_BGRX1010102
pub const SHM_FORMAT_BGRX1010102: u32 = 0x30335842;

/// WL_SHM_FORMAT_ARGB2101010
pub const SHM_FORMAT_ARGB2101010: u32 = 0x30335241;

/// WL_SHM_FORMAT_ABGR2101010
pub const SHM_FORMAT_ABGR2101010: u32 = 0x30334241;

/// WL_SHM_FORMAT_RGBA1010102
pub const SHM_FORMAT_RGBA1010102: u32 = 0x30334152;

/// WL_SHM_FORMAT_BGRA1010102
pub const SHM_FORMAT_BGRA1010102: u32 = 0x30334142;

/// WL_SHM_FORMAT_YUYV
pub const SHM_FORMAT_YUYV: u32 = 0x56595559;

/// WL_SHM_FORMAT_YVYU
pub const SHM_FORMAT_YVYU: u32 = 0x55595659;

/// WL_SHM_FORMAT_UYVY
pub const SHM_FORMAT_UYVY: u32 = 0x59565955;

/// WL_SHM_FORMAT_VYUY
pub const SHM_FORMAT_VYUY: u32 = 0x59555956;

/// WL_SHM_FORMAT_AYUV
pub const SHM_FORMAT_AYUV: u32 = 0x56555941;

/// WL_SHM_FORMAT_NV12
pub const SHM_FORMAT_NV12: u32 = 0x3231564e;

/// WL_SHM_FORMAT_NV21
pub const SHM_FORMAT_NV21: u32 = 0x3132564e;

/// WL_SHM_FORMAT_NV16
pub const SHM_FORMAT_NV16: u32 = 0x3631564e;

/// WL_SHM_FORMAT_NV61
pub const SHM_FORMAT_NV61: u32 = 0x3136564e;

/// WL_SHM_FORMAT_YUV410
pub const SHM_FORMAT_YUV410: u32 = 0x39565559;

/// WL_SHM_FORMAT_YVU410
pub const SHM_FORMAT_YVU410: u32 = 0x39555659;

/// WL_SHM_FORMAT_YUV411
pub const SHM_FORMAT_YUV411: u32 = 0x31315559;

/// WL_SHM_FORMAT_YVU411
pub const SHM_FORMAT_YVU411: u32 = 0x31315659;

/// WL_SHM_FORMAT_YUV420
pub const SHM_FORMAT_YUV420: u32 = 0x32315559;

/// WL_SHM_FORMAT_YVU420
pub const SHM_FORMAT_YVU420: u32 = 0x32315659;

/// WL_SHM_FORMAT_YUV422
pub const SHM_FORMAT_YUV422: u32 = 0x36315559;

/// WL_SHM_FORMAT_YVU422
pub const SHM_FORMAT_YVU422: u32 = 0x36315659;

/// WL_SHM_FORMAT_YUV444
pub const SHM_FORMAT_YUV444: u32 = 0x34325559;

/// WL_SHM_FORMAT_YVU444
pub const SHM_FORMAT_YVU444: u32 = 0x34325659;

/// WL_SHM_FORMAT_R8
pub const SHM_FORMAT_R8: u32 = 0x20203852;

/// WL_SHM_FORMAT_R16
pub const SHM_FORMAT_R16: u32 = 0x20363152;

/// WL_SHM_FORMAT_RG88
pub const SHM_FORMAT_RG88: u32 = 0x38384752;

/// WL_SHM_FORMAT_GR88
pub const SHM_FORMAT_GR88: u32 = 0x38385247;

/// WL_SHM_FORMAT_RG1616
pub const SHM_FORMAT_RG1616: u32 = 0x32334752;

/// WL_SHM_FORMAT_GR1616
pub const SHM_FORMAT_GR1616: u32 = 0x32335247;

/// WL_SHM_FORMAT_XRGB16161616F
pub const SHM_FORMAT_XRGB16161616F: u32 = 0x48345258;

/// WL_SHM_FORMAT_XBGR16161616F
pub const SHM_FORMAT_XBGR16161616F: u32 = 0x48344258;

/// WL_SHM_FORMAT_ARGB16161616F
pub const SHM_FORMAT_ARGB16161616F: u32 = 0x48345241;

/// WL_SHM_FORMAT_ABGR16161616F
pub const SHM_FORMAT_ABGR16161616F: u32 = 0x48344241;

/// WL_SHM_FORMAT_XYUV8888
pub const SHM_FORMAT_XYUV8888: u32 = 0x56555958;

/// WL_SHM_FORMAT_VUY888
pub const SHM_FORMAT_VUY888: u32 = 0x34325556;

/// WL_SHM_FORMAT_VUY101010
pub const SHM_FORMAT_VUY101010: u32 = 0x30335556;

/// WL_SHM_FORMAT_Y210
pub const SHM_FORMAT_Y210: u32 = 0x30313259;

/// WL_SHM_FORMAT_Y212
pub const SHM_FORMAT_Y212: u32 = 0x32313259;

/// WL_SHM_FORMAT_Y216
pub const SHM_FORMAT_Y216: u32 = 0x36313259;

/// WL_SHM_FORMAT_Y410
pub const SHM_FORMAT_Y410: u32 = 0x30313459;

/// WL_SHM_FORMAT_Y412
pub const SHM_FORMAT_Y412: u32 = 0x32313459;

/// WL_SHM_FORMAT_Y416
pub const SHM_FORMAT_Y416: u32 = 0x36313459;

/// WL_SHM_FORMAT_XVYU2101010
pub const SHM_FORMAT_XVYU2101010: u32 = 0x30335658;

/// WL_SHM_FORMAT_XVYU12_16161616
pub const SHM_FORMAT_XVYU12_16161616: u32 = 0x36335658;

/// WL_SHM_FORMAT_XVYU16161616
pub const SHM_FORMAT_XVYU16161616: u32 = 0x38345658;

/// WL_SHM_FORMAT_Y0L0
pub const SHM_FORMAT_Y0L0: u32 = 0x304c3059;

/// WL_SHM_FORMAT_X0L0
pub const SHM_FORMAT_X0L0: u32 = 0x304c3058;

/// WL_SHM_FORMAT_Y0L2
pub const SHM_FORMAT_Y0L2: u32 = 0x324c3059;

/// WL_SHM_FORMAT_X0L2
pub const SHM_FORMAT_X0L2: u32 = 0x324c3058;

/// WL_SHM_FORMAT_YUV420_8BIT
pub const SHM_FORMAT_YUV420_8BIT: u32 = 0x38305559;

/// WL_SHM_FORMAT_YUV420_10BIT
pub const SHM_FORMAT_YUV420_10BIT: u32 = 0x30315559;

/// WL_SHM_FORMAT_XRGB8888_A8
pub const SHM_FORMAT_XRGB8888_A8: u32 = 0x38415258;

/// WL_SHM_FORMAT_XBGR8888_A8
pub const SHM_FORMAT_XBGR8888_A8: u32 = 0x38414258;

/// WL_SHM_FORMAT_RGBX8888_A8
pub const SHM_FORMAT_RGBX8888_A8: u32 = 0x38415852;

/// WL_SHM_FORMAT_BGRX8888_A8
pub const SHM_FORMAT_BGRX8888_A8: u32 = 0x38415842;

/// WL_SHM_FORMAT_RGB888_A8
pub const SHM_FORMAT_RGB888_A8: u32 = 0x38413852;

/// WL_SHM_FORMAT_BGR888_A8
pub const SHM_FORMAT_BGR888_A8: u32 = 0x38413842;

/// WL_SHM_FORMAT_RGB565_A8
pub const SHM_FORMAT_RGB565_A8: u32 = 0x38413552;

/// WL_SHM_FORMAT_BGR565_A8
pub const SHM_FORMAT_BGR565_A8: u32 = 0x38413542;

/// WL_SHM_FORMAT_NV24
pub const SHM_FORMAT_NV24: u32 = 0x3432564e;

/// WL_SHM_FORMAT_NV42
pub const SHM_FORMAT_NV42: u32 = 0x3234564e;

/// WL_SHM_FORMAT_P210
pub const SHM_FORMAT_P210: u32 = 0x30313250;

/// WL_SHM_FORMAT_P010
pub const SHM_FORMAT_P010: u32 = 0x30313050;

/// WL_SHM_FORMAT_P012
pub const SHM_FORMAT_P012: u32 = 0x32313050;

/// WL_SHM_FORMAT_P016
pub const SHM_FORMAT_P016: u32 = 0x36313050;

/// WL_SHM_FORMAT_AXBXGXRX106106106106
pub const SHM_FORMAT_AXBXGXRX106106106106: u32 = 0x30314241;

/// WL_SHM_FORMAT_NV15
pub const SHM_FORMAT_NV15: u32 = 0x3531564e;

/// WL_SHM_FORMAT_Q410
pub const SHM_FORMAT_Q410: u32 = 0x30313451;

/// WL_SHM_FORMAT_Q401
pub const SHM_FORMAT_Q401: u32 = 0x31303451;

/// WL_SHM_FORMAT_XRGB16161616
pub const SHM_FORMAT_XRGB16161616: u32 = 0x38345258;

/// WL_SHM_FORMAT_XBGR16161616
pub const SHM_FORMAT_XBGR16161616: u32 = 0x38344258;

/// WL_SHM_FORMAT_ARGB16161616
pub const SHM_FORMAT_ARGB16161616: u32 = 0x38345241;

/// WL_SHM_FORMAT_ABGR16161616
pub const SHM_FORMAT_ABGR16161616: u32 = 0x38344241;

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
    name: c"wl_shm_pool".as_ptr(),
    version: 1,
    method_count: 3,
    methods: [
        Message {
            name: c"create_buffer".as_ptr(),
            signature: c"niiiiu".as_ptr(),
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
            name: c"destroy".as_ptr(),
            signature: c"".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"resize".as_ptr(),
            signature: c"i".as_ptr(),
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
    name: c"wl_buffer".as_ptr(),
    version: 1,
    method_count: 1,
    methods: [Message {
        name: c"destroy".as_ptr(),
        signature: c"".as_ptr(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 1,
    events: [Message {
        name: c"release".as_ptr(),
        signature: c"".as_ptr(),
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
    name: c"wl_seat".as_ptr(),
    version: 7,
    method_count: 4,
    methods: [
        Message {
            name: c"get_pointer".as_ptr(),
            signature: c"n".as_ptr(),
            types: [&POINTER_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"get_keyboard".as_ptr(),
            signature: c"n".as_ptr(),
            types: [&KEYBOARD_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"get_touch".as_ptr(),
            signature: c"n".as_ptr(),
            types: [&TOUCH_INTERFACE as *const Interface].as_ptr(),
        },
        Message {
            name: c"release".as_ptr(),
            signature: c"5".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 2,
    events: [
        Message {
            name: c"capabilities".as_ptr(),
            signature: c"u".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"name".as_ptr(),
            signature: c"2s".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// WL_SEAT_CAPABILITY_POINTER
pub const SEAT_CAPABILITY_POINTER: u32 = 0x1;

/// WL_SEAT_CAPABILITY_KEYBOARD
pub const SEAT_CAPABILITY_KEYBOARD: u32 = 0x2;

/// WL_SEAT_CAPABILITY_TOUCH
pub const SEAT_CAPABILITY_TOUCH: u32 = 0x4;

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
    name: c"wl_pointer".as_ptr(),
    version: 7,
    method_count: 2,
    methods: [
        Message {
            name: c"set_cursor".as_ptr(),
            signature: c"u?oii".as_ptr(),
            types: [ptr::null(), &SURFACE_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: c"release".as_ptr(),
            signature: c"3".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
    event_count: 9,
    events: [
        Message {
            name: c"enter".as_ptr(),
            signature: c"uoff".as_ptr(),
            types: [ptr::null(), &SURFACE_INTERFACE, ptr::null(), ptr::null()].as_ptr(),
        },
        Message {
            name: c"leave".as_ptr(),
            signature: c"uo".as_ptr(),
            types: [ptr::null(), &SURFACE_INTERFACE].as_ptr(),
        },
        Message {
            name: c"motion".as_ptr(),
            signature: c"uff".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"button".as_ptr(),
            signature: c"uuuu".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"axis".as_ptr(),
            signature: c"uuf".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"frame".as_ptr(),
            signature: c"5".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"axis_source".as_ptr(),
            signature: c"5u".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"axis_stop".as_ptr(),
            signature: c"5uu".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"axis_discrete".as_ptr(),
            signature: c"5ui".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// WL_POINTER_ERROR_ROLE
pub const POINTER_ERROR_ROLE: c_int = 0;

/// WL_POINTER_BUTTON_STATE_RELEASED
pub const POINTER_BUTTON_STATE_RELEASED: u32 = 0;

/// WL_POINTER_BUTTON_STATE_PRESSED
pub const POINTER_BUTTON_STATE_PRESSED: u32 = 1;

/// WL_POINTER_AXIS_VERTICAL_SCROLL
pub const POINTER_AXIS_VERTICAL_SCROLL: u32 = 0;

/// WL_POINTER_AXIS_HORIZONTAL_SCROLL
pub const POINTER_AXIS_HORIZONTAL_SCROLL: u32 = 1;

/// WL_POINTER_AXIS_SOURCE_WHEEL
pub const POINTER_AXIS_SOURCE_WHEEL: u32 = 0;

/// WL_POINTER_AXIS_SOURCE_FINGER
pub const POINTER_AXIS_SOURCE_FINGER: u32 = 1;

/// WL_POINTER_AXIS_SOURCE_CONTINUOUS
pub const POINTER_AXIS_SOURCE_CONTINUOUS: u32 = 2;

/// WL_POINTER_AXIS_SOURCE_WHEEL_TILT
pub const POINTER_AXIS_SOURCE_WHEEL_TILT: u32 = 3;

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
    name: c"wl_keyboard".as_ptr(),
    version: 7,
    method_count: 1,
    methods: [Message {
        name: c"release".as_ptr(),
        signature: c"3".as_ptr(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 6,
    events: [
        Message {
            name: c"keymap".as_ptr(),
            signature: c"uhu".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"enter".as_ptr(),
            signature: c"uoa".as_ptr(),
            types: [ptr::null(), &SURFACE_INTERFACE, ptr::null()].as_ptr(),
        },
        Message {
            name: c"leave".as_ptr(),
            signature: c"uo".as_ptr(),
            types: [ptr::null(), &SURFACE_INTERFACE].as_ptr(),
        },
        Message {
            name: c"key".as_ptr(),
            signature: c"uuuu".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"modifiers".as_ptr(),
            signature: c"uuuuu".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"repeat_info".as_ptr(),
            signature: c"4ii".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// WL_KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP
pub const KEYBOARD_KEYMAP_FORMAT_NO_KEYMAP: u32 = 0;

/// WL_KEYBOARD_KEYMAP_FORMAT_XKB_V1
pub const KEYBOARD_KEYMAP_FORMAT_XKB_V1: u32 = 1;

/// WL_KEYBOARD_KEY_STATE_RELEASED
pub const KEYBOARD_KEY_STATE_RELEASED: u32 = 0;

/// WL_KEYBOARD_KEY_STATE_PRESSED
pub const KEYBOARD_KEY_STATE_PRESSED: u32 = 1;

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
    name: c"wl_touch".as_ptr(),
    version: 7,
    method_count: 1,
    methods: [Message {
        name: c"release".as_ptr(),
        signature: c"3".as_ptr(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 7,
    events: [
        Message {
            name: c"down".as_ptr(),
            signature: c"uuoiff".as_ptr(),
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
            name: c"up".as_ptr(),
            signature: c"uui".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"motion".as_ptr(),
            signature: c"uiff".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"frame".as_ptr(),
            signature: c"".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"cancel".as_ptr(),
            signature: c"".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"shape".as_ptr(),
            signature: c"6iff".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"orientation".as_ptr(),
            signature: c"6if".as_ptr(),
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
    name: c"wl_output".as_ptr(),
    version: 4,
    method_count: 1,
    methods: [Message {
        name: c"release".as_ptr(),
        signature: c"3".as_ptr(),
        types: NULL_TYPES,
    }]
    .as_ptr(),
    event_count: 6,
    events: [
        Message {
            name: c"geometry".as_ptr(),
            signature: c"iiiiissi".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"mode".as_ptr(),
            signature: c"uiii".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"done".as_ptr(),
            signature: c"2".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"scale".as_ptr(),
            signature: c"2i".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"name".as_ptr(),
            signature: c"4s".as_ptr(),
            types: NULL_TYPES,
        },
        Message {
            name: c"description".as_ptr(),
            signature: c"4s".as_ptr(),
            types: NULL_TYPES,
        },
    ]
    .as_ptr(),
};

/// WL_OUTPUT_SUBPIXEL_UNKNOWN
pub const OUTPUT_SUBPIXEL_UNKNOWN: i32 = 0;

/// WL_OUTPUT_SUBPIXEL_NONE
pub const OUTPUT_SUBPIXEL_NONE: i32 = 1;

/// WL_OUTPUT_SUBPIXEL_HORIZONTAL_RGB
pub const OUTPUT_SUBPIXEL_HORIZONTAL_RGB: i32 = 2;

/// WL_OUTPUT_SUBPIXEL_HORIZONTAL_BGR
pub const OUTPUT_SUBPIXEL_HORIZONTAL_BGR: i32 = 3;

/// WL_OUTPUT_SUBPIXEL_VERTICAL_RGB
pub const OUTPUT_SUBPIXEL_VERTICAL_RGB: i32 = 4;

/// WL_OUTPUT_SUBPIXEL_VERTICAL_BGR
pub const OUTPUT_SUBPIXEL_VERTICAL_BGR: i32 = 5;

/// WL_OUTPUT_TRANSFORM_NORMAL
pub const OUTPUT_TRANSFORM_NORMAL: i32 = 0;

/// WL_OUTPUT_TRANSFORM_90
pub const OUTPUT_TRANSFORM_90: i32 = 1;

/// WL_OUTPUT_TRANSFORM_180
pub const OUTPUT_TRANSFORM_180: i32 = 2;

/// WL_OUTPUT_TRANSFORM_270
pub const OUTPUT_TRANSFORM_270: i32 = 3;

/// WL_OUTPUT_TRANSFORM_FLIPPED
pub const OUTPUT_TRANSFORM_FLIPPED: i32 = 4;

/// WL_OUTPUT_TRANSFORM_FLIPPED_90
pub const OUTPUT_TRANSFORM_FLIPPED_90: i32 = 5;

/// WL_OUTPUT_TRANSFORM_FLIPPED_180
pub const OUTPUT_TRANSFORM_FLIPPED_180: i32 = 6;

/// WL_OUTPUT_TRANSFORM_FLIPPED_270
pub const OUTPUT_TRANSFORM_FLIPPED_270: i32 = 7;

/// WL_OUTPUT_MODE_CURRENT
pub const OUTPUT_MODE_CURRENT: u32 = 0x1;

/// WL_OUTPUT_MODE_PREFERRED
pub const OUTPUT_MODE_PREFERRED: u32 = 0x2;

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
