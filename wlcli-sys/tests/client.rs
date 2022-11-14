// Copyright 2022 Gustavo C. Viegas. All rights reserved.

#![cfg(unix)]

use std::ffi::{c_char, c_void, CStr};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::AsRawFd;
use std::pin::Pin;
use std::ptr;
use std::thread;
use std::time::{Duration, Instant};

use wlcli_sys::{
    self, Buffer, BufferListener, Compositor, Display, Keyboard, KeyboardListener, Output, Pointer,
    PointerListener, Registry, RegistryListener, Seat, SeatListener, Shm, ShmPool, Surface,
    SurfaceListener, Toplevel, ToplevelListener, Touch, WmBase, WmBaseListener, XdgSurface,
    XdgSurfaceListener,
};

#[test]
fn test_client() {
    for _ in 0..3 {
        thread::spawn(|| wlcli_sys::init().unwrap());
    }
    wlcli_sys::init().unwrap();

    let display = connect();

    let global = bind(display);
    println!("global: {:#?}", global);

    let input = global.get_input(display);
    println!("input: {:#?}", input);

    let surface = global.create_surface();
    println!("surface: {:#?}", surface);

    global.set_wm();
    let xdg_surface = global.get_xdg_surface(surface);
    let toplevel = get_toplevel(xdg_surface);
    println!("xdg_surface: {:#?}\ntoplevel: {:#?}", xdg_surface, toplevel);

    // Need to commit with no buffer first.
    commit(surface);
    roundtrip(display);

    const WIDTH: i32 = 480;
    const HEIGHT: i32 = 300;
    const BPP: i32 = 8 + 8 + 8 + 8;
    const SIZE: i32 = WIDTH * HEIGHT * BPP / 8;

    let mut file = File::options()
        .read(true)
        .write(true)
        .create(true)
        .open("/var/tmp/demi.wlcli-sys.test.0")
        .unwrap();
    let mut data = vec![0u8; SIZE as usize];
    data.chunks_exact_mut((BPP / 8 * 4) as usize)
        .for_each(|x| x[..(BPP / 8 * 2) as usize].copy_from_slice(&[255; (BPP / 8 * 2) as usize]));
    file.write(&data).unwrap();
    file.flush().unwrap();
    let fd = file.as_raw_fd();
    let shm_pool = global.create_pool(fd, SIZE);
    println!("shm_pool: {:#?}", shm_pool);

    let buffer = create_buffer(
        shm_pool,
        0,
        WIDTH,
        HEIGHT,
        WIDTH * BPP / 8,
        wlcli_sys::SHM_FORMAT_XRGB8888,
    );
    println!("buffer: {:#?}", buffer);

    // Commit a second time, now with a buffer attached.
    attach(surface, buffer, 0, 0);
    commit(surface);
    flush(display);

    while !quit() {
        dispatch(display);
    }

    //wait_wm_ping(display, Duration::new(30, 0));

    disconnect(display);

    for _ in 0..3 {
        thread::spawn(wlcli_sys::fini);
    }
    wlcli_sys::fini();
}

static mut QUIT: bool = false;

fn quit() -> bool {
    unsafe { QUIT }
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

fn dispatch(display: *mut Display) {
    unsafe {
        wlcli_sys::display_dispatch(display);
    }
}

fn dispatch_pending(display: *mut Display) {
    unsafe {
        wlcli_sys::display_dispatch_pending(display);
    }
}

fn flush(display: *mut Display) {
    unsafe {
        wlcli_sys::display_flush(display);
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
            wm_base: (ptr::null_mut(), u32::MAX),
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
        assert!(!global.wm_base.0.is_null());
        assert!(!global.shm.0.is_null());
        assert!(!global.seat.0.is_null());
        assert!(!global.output.0.is_null());

        global
    }
}

#[derive(Debug)]
struct Global {
    compositor: (*mut Compositor, u32),
    wm_base: (*mut WmBase, u32),
    shm: (*mut Shm, u32),
    seat: (*mut Seat, u32),
    output: (*mut Output, u32),
}

impl Global {
    fn create_surface(&self) -> *mut Surface {
        unsafe {
            let surface = wlcli_sys::compositor_create_surface(self.compositor.0);
            assert!(!surface.is_null());
            assert_eq!(
                wlcli_sys::surface_add_listener(surface, &SURFACE_LISTENER, ptr::null_mut()),
                0
            );
            surface
        }
    }

    fn set_wm(&self) {
        unsafe {
            assert_eq!(
                wlcli_sys::wm_base_add_listener(
                    self.wm_base.0,
                    &WM_BASE_LISTENER,
                    &mut PING_CHECK as *mut _ as *mut _,
                ),
                0
            );
        }
    }

    fn get_xdg_surface(&self, surface: *mut Surface) -> *mut XdgSurface {
        unsafe {
            let xdg_surface = wlcli_sys::wm_base_get_xdg_surface(self.wm_base.0, surface);
            assert!(!xdg_surface.is_null());
            assert_eq!(
                wlcli_sys::xdg_surface_add_listener(
                    xdg_surface,
                    &XDG_SURFACE_LISTENER,
                    ptr::null_mut(),
                ),
                0
            );
            xdg_surface
        }
    }

    fn create_pool(&self, fd: i32, size: i32) -> *mut ShmPool {
        unsafe {
            let shm_pool = wlcli_sys::shm_create_pool(self.shm.0, fd, size);
            assert!(!shm_pool.is_null());
            shm_pool
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
            } else {
                assert_eq!(
                    wlcli_sys::pointer_add_listener(
                        (*input).pointer,
                        &POINTER_LISTENER,
                        ptr::null_mut(),
                    ),
                    0
                );
            }
            if (*input).keyboard.is_null() {
                println!("[!] no keyboard device!");
            } else {
                assert_eq!(
                    wlcli_sys::keyboard_add_listener(
                        (*input).keyboard,
                        &KEYBOARD_LISTENER,
                        ptr::null_mut(),
                    ),
                    0
                );
            }
            if (*input).touch.is_null() {
                println!("[!] no touch device!");
            }

            input
        }
    }
}

static mut PING_CHECK: isize = -50;

fn wait_wm_ping(display: *mut Display, timeout: Duration) {
    let start = Instant::now();
    unsafe {
        while PING_CHECK < 0 {
            wlcli_sys::display_roundtrip(display);
            if start.elapsed() >= timeout {
                break;
            }
            thread::sleep(Duration::from_millis(33));
        }
        println!("PING_CHECK: {}", PING_CHECK);
    }
}

fn get_toplevel(xdg_surface: *mut XdgSurface) -> *mut Toplevel {
    unsafe {
        let toplevel = wlcli_sys::xdg_surface_get_toplevel(xdg_surface);
        assert!(!toplevel.is_null());
        assert_eq!(
            wlcli_sys::toplevel_add_listener(toplevel, &TOPLEVEL_LISTENER, ptr::null_mut()),
            0
        );
        toplevel
    }
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
        assert_eq!(
            wlcli_sys::buffer_add_listener(buffer, &BUFFER_LISTENER, ptr::null_mut()),
            0
        );
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

#[derive(Debug)]
struct Input {
    pointer: *mut Pointer,
    keyboard: *mut Keyboard,
    touch: *mut Touch,
}

static REGISTRY_LISTENER: RegistryListener = RegistryListener {
    global: rty_global,
    global_remove: rty_global_remove,
};

unsafe extern "C" fn rty_global(
    data: *mut c_void,
    registry: *mut Registry,
    name: u32,
    interface: *const c_char,
    version: u32,
) {
    println!(
        "\tregistry.global: {:?} {:?} {:?} {:?} {:?}",
        data,
        registry,
        name,
        CStr::from_ptr(interface),
        version
    );

    let data: &mut Global = &mut *data.cast();

    match CStr::from_ptr(interface).to_str().unwrap() {
        "wl_compositor" => {
            let cpt =
                wlcli_sys::registry_bind(registry, name, &wlcli_sys::COMPOSITOR_INTERFACE, version);
            assert!(!cpt.is_null());
            data.compositor = (cpt.cast(), name);
        }
        "xdg_wm_base" => {
            let wm =
                wlcli_sys::registry_bind(registry, name, &wlcli_sys::WM_BASE_INTERFACE, version);
            assert!(!wm.is_null());
            data.wm_base = (wm.cast(), name);
        }
        "wl_shm" => {
            let shm = wlcli_sys::registry_bind(registry, name, &wlcli_sys::SHM_INTERFACE, version);
            assert!(!shm.is_null());
            data.shm = (shm.cast(), name);
        }
        "wl_seat" => {
            let seat =
                wlcli_sys::registry_bind(registry, name, &wlcli_sys::SEAT_INTERFACE, version);
            assert!(!seat.is_null());
            data.seat = (seat.cast(), name);
        }
        "wl_output" => {
            let out =
                wlcli_sys::registry_bind(registry, name, &wlcli_sys::OUTPUT_INTERFACE, version);
            assert!(!out.is_null());
            data.output = (out.cast(), name);
        }
        _ => (),
    }
}

unsafe extern "C" fn rty_global_remove(data: *mut c_void, registry: *mut Registry, name: u32) {
    println!(
        "\tregistry.global_remove: {:?} {:?} {:?}",
        data, registry, name
    );

    let data: &Global = &*data.cast();

    assert_ne!(name, data.compositor.1);
    assert_ne!(name, data.wm_base.1);
    assert_ne!(name, data.shm.1);
    assert_ne!(name, data.seat.1);
    assert_ne!(name, data.output.1);
}

static SURFACE_LISTENER: SurfaceListener = SurfaceListener {
    enter: sf_enter,
    leave: sf_leave,
};

unsafe extern "C" fn sf_enter(data: *mut c_void, surface: *mut Surface, output: *mut Output) {
    println!("surface.enter: {:?} {:?} {:?}", data, surface, output);
}

unsafe extern "C" fn sf_leave(data: *mut c_void, surface: *mut Surface, output: *mut Output) {
    println!("surface.leave: {:?} {:?} {:?}", data, surface, output);
}

static WM_BASE_LISTENER: WmBaseListener = WmBaseListener { ping: wm_ping };

unsafe extern "C" fn wm_ping(data: *mut c_void, wm_base: *mut WmBase, serial: u32) {
    println!("wm_base.ping: {:?} {:?} {:?}", data, wm_base, serial);
    wlcli_sys::wm_base_pong(wm_base, serial);
    *data.cast::<isize>() += 1;
}

static XDG_SURFACE_LISTENER: XdgSurfaceListener = XdgSurfaceListener {
    configure: xsf_configure,
};

unsafe extern "C" fn xsf_configure(data: *mut c_void, xdg_surface: *mut XdgSurface, serial: u32) {
    println!(
        "xdg_surface.configure: {:?} {:?} {:?}",
        data, xdg_surface, serial
    );
    wlcli_sys::xdg_surface_ack_configure(xdg_surface, serial);
}

static TOPLEVEL_LISTENER: ToplevelListener = ToplevelListener {
    configure: top_configure,
    close: top_close,
    configure_bounds: top_configure_bounds,
};

unsafe extern "C" fn top_configure(
    data: *mut c_void,
    toplevel: *mut Toplevel,
    width: i32,
    height: i32,
    states: *mut c_void,
) {
    println!(
        "toplevel.configure: {:?} {:?} {:?} {:?} {:?}",
        data, toplevel, width, height, states
    );
}

unsafe extern "C" fn top_close(data: *mut c_void, toplevel: *mut Toplevel) {
    println!("toplevel.close: {:?} {:?}", data, toplevel);
    QUIT = true;
}

unsafe extern "C" fn top_configure_bounds(
    data: *mut c_void,
    toplevel: *mut Toplevel,
    width: i32,
    height: i32,
) {
    println!(
        "toplevel.configure_bounds: {:?} {:?} {:?} {:?}",
        data, toplevel, width, height
    );
}

static BUFFER_LISTENER: BufferListener = BufferListener {
    release: buf_release,
};

unsafe extern "C" fn buf_release(data: *mut c_void, buffer: *mut Buffer) {
    println!("buffer.release: {:?} {:?}", data, buffer);
}

static SEAT_LISTENER: SeatListener = SeatListener {
    capabilities: seat_capabilities,
    name: seat_name,
};

unsafe extern "C" fn seat_capabilities(data: *mut c_void, seat: *mut Seat, capabilities: u32) {
    println!(
        "seat.capabilities: {:?} {:?} {:?}",
        data, seat, capabilities
    );

    let data: &mut Input = &mut *data.cast();

    if capabilities & wlcli_sys::SEAT_CAPABILITY_POINTER != 0 {
        data.pointer = wlcli_sys::seat_get_pointer(seat);
        assert!(!data.pointer.is_null());
    }
    if capabilities & wlcli_sys::SEAT_CAPABILITY_KEYBOARD != 0 {
        data.keyboard = wlcli_sys::seat_get_keyboard(seat);
        assert!(!data.keyboard.is_null());
    }
    if capabilities & wlcli_sys::SEAT_CAPABILITY_TOUCH != 0 {
        data.touch = wlcli_sys::seat_get_touch(seat);
        assert!(!data.touch.is_null());
    }
}

unsafe extern "C" fn seat_name(data: *mut c_void, seat: *mut Seat, name: *const c_char) {
    println!(
        "seat.name: {:?} {:?} {:?}",
        data,
        seat,
        CStr::from_ptr(name)
    );
}

static POINTER_LISTENER: PointerListener = PointerListener {
    enter: pt_enter,
    leave: pt_leave,
    motion: pt_motion,
    button: pt_button,
    axis: pt_axis,
    frame: pt_frame,
    axis_source: pt_axis_source,
    axis_stop: pt_axis_stop,
    axis_discrete: pt_axis_discrete,
};

unsafe extern "C" fn pt_enter(
    data: *mut c_void,
    pointer: *mut Pointer,
    serial: u32,
    surface: *mut Surface,
    surface_x: i32, // 24.8
    surface_y: i32, // 24.8
) {
    println!(
        "pointer.enter: {:?} {:?} {:?} {:?} {:?} {:?}",
        data,
        pointer,
        serial,
        surface,
        surface_x / 256,
        surface_y / 256
    );
    wlcli_sys::pointer_set_cursor(pointer, serial, ptr::null_mut(), 0, 0);
}

unsafe extern "C" fn pt_leave(
    data: *mut c_void,
    pointer: *mut Pointer,
    serial: u32,
    surface: *mut Surface,
) {
    println!(
        "pointer.leave: {:?} {:?} {:?} {:?}",
        data, pointer, serial, surface
    );
}

unsafe extern "C" fn pt_motion(
    data: *mut c_void,
    pointer: *mut Pointer,
    time: u32,
    surface_x: i32, // 24.8
    surface_y: i32, // 24.8
) {
    println!(
        "pointer.motion: {:?} {:?} {:?} {:?} {:?}",
        data,
        pointer,
        time,
        surface_x / 256,
        surface_y / 256
    );
}

unsafe extern "C" fn pt_button(
    data: *mut c_void,
    pointer: *mut Pointer,
    serial: u32,
    time: u32,
    button: u32,
    state: u32,
) {
    println!(
        "pointer.button: {:?} {:?} {:?} {:?} {:?} {:?}",
        data, pointer, serial, time, button, state
    );
}

unsafe extern "C" fn pt_axis(
    data: *mut c_void,
    pointer: *mut Pointer,
    time: u32,
    axis: u32,
    value: i32, // 24.8
) {
    println!(
        "pointer.axis: {:?} {:?} {:?} {:?} {:?}",
        data, pointer, time, axis, value
    );
}

unsafe extern "C" fn pt_frame(data: *mut c_void, pointer: *mut Pointer) {
    println!("pointer.frame: {:?} {:?}", data, pointer);
}

unsafe extern "C" fn pt_axis_source(data: *mut c_void, pointer: *mut Pointer, axis_source: u32) {
    println!(
        "pointer.axis_source: {:?} {:?} {:?}",
        data, pointer, axis_source
    );
}

unsafe extern "C" fn pt_axis_stop(data: *mut c_void, pointer: *mut Pointer, time: u32, axis: u32) {
    println!(
        "pointer.axis_stop: {:?} {:?} {:?} {:?}",
        data, pointer, time, axis
    );
}

unsafe extern "C" fn pt_axis_discrete(
    data: *mut c_void,
    pointer: *mut Pointer,
    axis: u32,
    discrete: i32,
) {
    println!(
        "pointer.axis_discrete: {:?} {:?} {:?} {:?}",
        data, pointer, axis, discrete
    );
}

static KEYBOARD_LISTENER: KeyboardListener = KeyboardListener {
    keymap: kb_keymap,
    enter: kb_enter,
    leave: kb_leave,
    key: kb_key,
    modifiers: kb_modifiers,
    repeat_info: kb_repeat_info,
};

unsafe extern "C" fn kb_keymap(
    data: *mut c_void,
    keyboard: *mut Keyboard,
    format: u32,
    fd: i32,
    size: u32,
) {
    println!(
        "keyboard.keymap: {:?} {:?} {:?} {:?} {:?}",
        data, keyboard, format, fd, size
    );
}

unsafe extern "C" fn kb_enter(
    data: *mut c_void,
    keyboard: *mut Keyboard,
    serial: u32,
    surface: *mut Surface,
    keys: *mut c_void, // TODO
) {
    println!(
        "keyboard.enter: {:?} {:?} {:?} {:?} {:?}",
        data, keyboard, serial, surface, keys
    );
}

unsafe extern "C" fn kb_leave(
    data: *mut c_void,
    keyboard: *mut Keyboard,
    serial: u32,
    surface: *mut Surface,
) {
    println!(
        "keyboard.leave: {:?} {:?} {:?} {:?}",
        data, keyboard, serial, surface
    );
}

unsafe extern "C" fn kb_key(
    data: *mut c_void,
    keyboard: *mut Keyboard,
    serial: u32,
    time: u32,
    key: u32,
    state: u32,
) {
    println!(
        "keyboard.key: {:?} {:?} {:?} {:?} {:?} {:?}",
        data, keyboard, serial, time, key, state
    );
}

unsafe extern "C" fn kb_modifiers(
    data: *mut c_void,
    keyboard: *mut Keyboard,
    serial: u32,
    mods_depressed: u32,
    mods_latched: u32,
    mods_locked: u32,
    group: u32,
) {
    println!(
        "keyboard.modifiers: {:?} {:?} {:?} {:?} {:?} {:?} {:?}",
        data, keyboard, serial, mods_depressed, mods_latched, mods_locked, group
    );
}

unsafe extern "C" fn kb_repeat_info(
    data: *mut c_void,
    keyboard: *mut Keyboard,
    rate: i32,
    delay: i32,
) {
    println!(
        "keyboard.repeat_info: {:?} {:?} {:?} {:?}",
        data, keyboard, rate, delay
    );
}
