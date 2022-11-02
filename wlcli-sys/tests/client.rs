// Copyright 2022 Gustavo C. Viegas. All rights reserved.

#![cfg(unix)]

use std::ffi::{c_char, c_void, CStr};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::pin::Pin;
use std::ptr;

use wlcli_sys::{
    self, Buffer, Compositor, Display, Keyboard, Output, Pointer, Registry, RegistryListener, Seat,
    SeatListener, Shm, ShmPool, Surface, Touch, COMPOSITOR_INTERFACE, OUTPUT_INTERFACE,
    SEAT_INTERFACE, SHM_INTERFACE,
};

#[test]
fn test_client() {
    match wlcli_sys::init() {
        Ok(()) => (),
        Err(e) => panic!("{e}"),
    }

    let display = connect();

    let global = bind(display);
    println!("{:#?}", global);

    let surfaces = vec![
        global.create_surface(),
        global.create_surface(),
        global.create_surface(),
    ];
    println!("{:#?}", surfaces);

    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;
    const BPP: i32 = 8 + 8 + 8 + 8;
    const SIZE: i32 = WIDTH * HEIGHT * BPP / 8;

    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("/var/tmp/demi.wlcli-sys.test.0")
        .unwrap();
    let data = vec![0u8; SIZE as usize];
    file.write(&data).unwrap();
    let fd = file.as_raw_fd();
    let shm_pool = global.create_pool(fd, SIZE);
    println!("{:#?}", shm_pool);

    // TODO: Define formats in `wl.rs`.
    const SHM_FORMAT_ARGB8888: u32 = 0;
    let buffer = create_buffer(
        shm_pool,
        0,
        WIDTH,
        HEIGHT,
        WIDTH * BPP / 8,
        SHM_FORMAT_ARGB8888,
    );
    println!("{:#?}", buffer);

    attach(surfaces[1], buffer, 0, 0);
    commit(surfaces[1]);

    let input = global.get_input(display);
    println!("{:#?}", input);

    roundtrip(display);

    disconnect(display);
}

fn connect() -> *mut Display {
    unsafe {
        let display = wlcli_sys::display_connect(ptr::null());
        assert!(!display.is_null());
        display
    }
}

fn disconnect(display: *mut Display) {
    unsafe {
        wlcli_sys::display_disconnect(display);
    }
}

fn roundtrip(display: *mut Display) {
    unsafe {
        wlcli_sys::display_roundtrip(display);
    }
}

fn get_registry(display: *mut Display) -> *mut Registry {
    unsafe {
        let registry = wlcli_sys::display_get_registry(display);
        assert!(!registry.is_null());
        registry
    }
}

fn bind(display: *mut Display) -> Pin<Box<Global>> {
    unsafe {
        let registry = get_registry(display);
        let mut global = Box::pin(Global {
            compositor: (ptr::null_mut(), u32::MAX),
            shm: (ptr::null_mut(), u32::MAX),
            seat: (ptr::null_mut(), u32::MAX),
            output: (ptr::null_mut(), u32::MAX),
        });

        assert_eq!(
            wlcli_sys::registry_add_listener(
                registry,
                &REGISTRY_LISTENER,
                &mut *global as *mut _ as *mut _
            ),
            0
        );

        wlcli_sys::display_roundtrip(display);

        // These globals are expected to be present.
        assert!(!global.compositor.0.is_null());
        assert!(!global.shm.0.is_null());
        assert!(!global.seat.0.is_null());
        assert!(!global.output.0.is_null());

        global
    }
}

#[derive(Debug)]
struct Global {
    compositor: (*mut Compositor, u32),
    shm: (*mut Shm, u32),
    seat: (*mut Seat, u32),
    output: (*mut Output, u32),
}

impl Global {
    fn create_surface(&self) -> *mut Surface {
        unsafe {
            let sf = wlcli_sys::compositor_create_surface(self.compositor.0);
            assert!(!sf.is_null());
            sf
        }
    }

    fn create_pool(&self, fd: i32, size: i32) -> *mut ShmPool {
        unsafe {
            let pool = wlcli_sys::shm_create_pool(self.shm.0, fd, size);
            assert!(!pool.is_null());
            pool
        }
    }

    fn get_input(&self, display: *mut Display) -> Pin<Box<Input>> {
        unsafe {
            let mut input = Box::pin(Input {
                pointer: ptr::null_mut(),
                keyboard: ptr::null_mut(),
                touch: ptr::null_mut(),
            });

            assert_eq!(
                wlcli_sys::seat_add_listener(
                    self.seat.0,
                    &SEAT_LISTENER,
                    &mut *input as *mut _ as *mut _
                ),
                0
            );

            wlcli_sys::display_roundtrip(display);

            // It is ok for input to be missing.
            if (*input).pointer.is_null() {
                println!("[!] no pointer device!");
            }
            if (*input).keyboard.is_null() {
                println!("[!] no keyboard device!");
            }
            if (*input).touch.is_null() {
                println!("[!] no touch device!");
            }

            input
        }
    }
}

#[derive(Debug)]
struct Input {
    pointer: *mut Pointer,
    keyboard: *mut Keyboard,
    touch: *mut Touch,
}

unsafe extern "C" fn rty_global(
    data: *mut c_void,
    registry: *mut Registry,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    println!(
        "\tregistry.global: {:?}  {:?}  {:?}  {:?}  {:?}",
        data,
        registry,
        name,
        CStr::from_ptr(interface),
        version
    );

    let data: &mut Global = &mut *data.cast();

    match CStr::from_ptr(interface).to_str().unwrap() {
        "wl_compositor" => {
            let cpt = wlcli_sys::registry_bind(registry, name, &COMPOSITOR_INTERFACE, version);
            assert!(!cpt.is_null());
            (*data).compositor = (cpt.cast(), name);
        }
        "wl_shm" => {
            let shm = wlcli_sys::registry_bind(registry, name, &SHM_INTERFACE, version);
            assert!(!shm.is_null());
            (*data).shm = (shm.cast(), name);
        }
        "wl_seat" => {
            let seat = wlcli_sys::registry_bind(registry, name, &SEAT_INTERFACE, version);
            assert!(!seat.is_null());
            (*data).seat = (seat.cast(), name);
        }
        "wl_output" => {
            let out = wlcli_sys::registry_bind(registry, name, &OUTPUT_INTERFACE, version);
            assert!(!out.is_null());
            (*data).output = (out.cast(), name);
        }
        _ => (),
    }
}

static REGISTRY_LISTENER: RegistryListener = RegistryListener {
    global: rty_global,
    global_remove: rty_global_remove,
};

unsafe extern "C" fn rty_global_remove(data: *mut c_void, registry: *mut Registry, name: u32) {
    println!(
        "\tregistry.global_remove: {:?}  {:?}  {:?}",
        data, registry, name
    );

    let data: *mut Global = data.cast();

    assert_ne!(name, (*data).compositor.1);
    assert_ne!(name, (*data).shm.1);
    assert_ne!(name, (*data).seat.1);
    assert_ne!(name, (*data).output.1);
}

fn create_buffer(
    shm_pool: *mut ShmPool,
    offset: i32,
    width: i32,
    height: i32,
    stride: i32,
    format: u32,
) -> *mut Buffer {
    unsafe {
        let buffer =
            wlcli_sys::shm_pool_create_buffer(shm_pool, offset, width, height, stride, format);
        assert!(!buffer.is_null());
        buffer
    }
}

fn attach(surface: *mut Surface, buffer: *mut Buffer, x: i32, y: i32) {
    unsafe {
        wlcli_sys::surface_attach(surface, buffer, x, y);
    }
}

fn commit(surface: *mut Surface) {
    unsafe {
        wlcli_sys::surface_commit(surface);
    }
}

static SEAT_LISTENER: SeatListener = SeatListener {
    capabilities: seat_capabilities,
    name: seat_name,
};

unsafe extern "C" fn seat_capabilities(data: *mut c_void, seat: *mut Seat, capabilities: u32) {
    println!(
        "seat.capabilities: {:?}  {:?}  {:?}",
        data, seat, capabilities
    );

    // TODO: Define capabilities in `wl.rs`.
    const SEAT_CAPABILITY_POINTER: u32 = 1 << 0;
    const SEAT_CAPABILITY_KEYBOARD: u32 = 1 << 1;
    const SEAT_CAPABILITY_TOUCH: u32 = 1 << 2;

    let data: &mut Input = &mut *data.cast();

    if capabilities & SEAT_CAPABILITY_POINTER != 0 {
        data.pointer = wlcli_sys::seat_get_pointer(seat);
        assert!(!data.pointer.is_null());
    }
    if capabilities & SEAT_CAPABILITY_KEYBOARD != 0 {
        data.keyboard = wlcli_sys::seat_get_keyboard(seat);
        assert!(!data.keyboard.is_null());
    }
    if capabilities & SEAT_CAPABILITY_TOUCH != 0 {
        data.touch = wlcli_sys::seat_get_touch(seat);
        assert!(!data.touch.is_null());
    }
}

unsafe extern "C" fn seat_name(data: *mut c_void, seat: *mut Seat, name: *const c_char) {
    println!(
        "seat.name: {:?}  {:?}  {:?}",
        data,
        seat,
        CStr::from_ptr(name)
    );
}
