//! Wayland client API.

#![cfg(unix)]

use std::ffi::{c_char, c_int, c_void};
use std::hint;
use std::mem;
use std::sync::atomic::{AtomicUsize, Ordering};

use dl::Dl;

const MARSHAL_FLAG_DESTROY: u32 = 1 << 0;

// wl_proxy_marshal_flags
macro_rules! proxy_marshal_flags {
    ($proxy:expr, $opcode:expr, $iface:expr, $vers:expr, $flags:expr, $( $x:expr ),*) => {
        (crate::LIB.as_ref().unwrap().1.proxy_marshal_flags)(
            $proxy,
            $opcode,
            $iface,
            $vers,
            $flags,
            $( $x ),*
        )
    };
}

mod wl;
pub use crate::wl::*;

mod xdg;
pub use crate::xdg::*;

/// struct wl_interface
#[repr(C)]
pub struct Interface {
    name: *const c_char,
    version: c_int,
    method_count: c_int,
    methods: *const Message,
    event_count: c_int,
    events: *const Message,
}

unsafe impl Send for Interface {}
unsafe impl Sync for Interface {}

/// struct wl_message
#[repr(C)]
pub struct Message {
    name: *const c_char,
    signature: *const c_char,
    types: *const *const Interface,
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

/// struct wl_proxy
#[repr(C)]
pub struct Proxy {
    _opaque: [u8; 0],
}

/// struct wl_display
#[repr(C)]
pub struct Display {
    _opaque: [u8; 0],
}

type ProxyMarshalFlags = unsafe extern "C" fn(
    proxy: *mut Proxy,
    opcode: u32,
    interface: *const Interface,
    version: u32,
    flags: u32,
    ...
) -> *mut Proxy;

type ProxyDestroy = unsafe extern "C" fn(proxy: *mut Proxy);

type ProxyAddListener = unsafe extern "C" fn(
    proxy: *mut Proxy,
    implementation: *mut unsafe extern "C" fn(),
    data: *mut c_void,
) -> c_int;

type ProxyGetListener = unsafe extern "C" fn(proxy: *mut Proxy) -> *const c_void;

type ProxySetUserData = unsafe extern "C" fn(proxy: *mut Proxy, data: *mut c_void);

type ProxyGetUserData = unsafe extern "C" fn(proxy: *mut Proxy) -> *mut c_void;

type ProxyGetVersion = unsafe extern "C" fn(proxy: *mut Proxy) -> u32;

type ProxyGetId = unsafe extern "C" fn(proxy: *mut Proxy) -> u32;

type DisplayConnect = unsafe extern "C" fn(name: *const c_char) -> *mut Display;

type DisplayDisconnect = unsafe extern "C" fn(display: *mut Display);

type DisplayDispatch = unsafe extern "C" fn(display: *mut Display) -> c_int;

type DisplayDispatchPending = unsafe extern "C" fn(display: *mut Display) -> c_int;

type DisplayFlush = unsafe extern "C" fn(display: *mut Display) -> c_int;

type DisplayRoundtrip = unsafe extern "C" fn(display: *mut Display) -> c_int;

type DisplayGetError = unsafe extern "C" fn(display: *mut Display) -> c_int;

#[derive(Debug)]
struct Fp {
    proxy_marshal_flags: ProxyMarshalFlags,
    proxy_destroy: ProxyDestroy,
    proxy_add_listener: ProxyAddListener,
    proxy_get_listener: ProxyGetListener,
    proxy_set_user_data: ProxySetUserData,
    proxy_get_user_data: ProxyGetUserData,
    proxy_get_version: ProxyGetVersion,
    proxy_get_id: ProxyGetId,
    display_connect: DisplayConnect,
    display_disconnect: DisplayDisconnect,
    display_dispatch: DisplayDispatch,
    display_dispatch_pending: DisplayDispatchPending,
    display_flush: DisplayFlush,
    display_roundtrip: DisplayRoundtrip,
    display_get_error: DisplayGetError,
}

fn get_fp(lib: &Dl) -> Result<Fp, String> {
    unsafe {
        let proxy_marshal_flags: ProxyMarshalFlags =
            mem::transmute(lib.get("wl_proxy_marshal_flags")?);

        let proxy_destroy: ProxyDestroy = mem::transmute(lib.get("wl_proxy_destroy")?);

        let proxy_add_listener: ProxyAddListener =
            mem::transmute(lib.get("wl_proxy_add_listener")?);

        let proxy_get_listener: ProxyGetListener =
            mem::transmute(lib.get("wl_proxy_get_listener")?);

        let proxy_set_user_data: ProxySetUserData =
            mem::transmute(lib.get("wl_proxy_set_user_data")?);

        let proxy_get_user_data: ProxyGetUserData =
            mem::transmute(lib.get("wl_proxy_get_user_data")?);

        let proxy_get_version: ProxyGetVersion = mem::transmute(lib.get("wl_proxy_get_version")?);

        let proxy_get_id: ProxyGetId = mem::transmute(lib.get("wl_proxy_get_id")?);

        let display_connect: DisplayConnect = mem::transmute(lib.get("wl_display_connect")?);

        let display_disconnect: DisplayDisconnect =
            mem::transmute(lib.get("wl_display_disconnect")?);

        let display_dispatch: DisplayDispatch = mem::transmute(lib.get("wl_display_dispatch")?);

        let display_dispatch_pending: DisplayDispatchPending =
            mem::transmute(lib.get("wl_display_dispatch_pending")?);

        let display_flush: DisplayFlush = mem::transmute(lib.get("wl_display_flush")?);

        let display_roundtrip: DisplayRoundtrip = mem::transmute(lib.get("wl_display_roundtrip")?);

        let display_get_error: DisplayGetError = mem::transmute(lib.get("wl_display_get_error")?);

        Ok(Fp {
            proxy_marshal_flags,
            proxy_destroy,
            proxy_add_listener,
            proxy_get_listener,
            proxy_set_user_data,
            proxy_get_user_data,
            proxy_get_version,
            proxy_get_id,
            display_connect,
            display_disconnect,
            display_dispatch,
            display_dispatch_pending,
            display_flush,
            display_roundtrip,
            display_get_error,
        })
    }
}

const LIB_NAME: &str = "libwayland-client.so.0";
static mut LIB: Option<(Dl, Fp)> = None;
static RC: AtomicUsize = AtomicUsize::new(0);

/// Initializes the library.
///
/// NOTE: It should be paired with a subsequent [`fini`] call.
pub fn init() -> Result<(), String> {
    let mut err = String::new();
    match RC.swap(usize::MAX, Ordering::AcqRel) {
        0 => {
            match Dl::new(LIB_NAME, dl::LAZY | dl::LOCAL) {
                Ok(lib) => match get_fp(&lib) {
                    Ok(fp) => unsafe { LIB = Some((lib, fp)) },
                    Err(e) => err = e,
                },
                Err(e) => err = e,
            }
            RC.store(1, Ordering::Release);
        }
        usize::MAX => {
            while RC.load(Ordering::Acquire) == usize::MAX {
                hint::spin_loop();
            }
            return init();
        }
        x => {
            assert!(x < isize::MAX as _, "RC overflow");
            RC.store(x + 1, Ordering::Release);
        }
    }
    unsafe {
        if LIB.is_some() {
            Ok(())
        } else {
            Err(err)
        }
    }
}

/// Finalizes the library.
///
/// NOTE: It should be paired with a previous [`init`] call.
pub fn fini() {
    match RC.swap(usize::MAX, Ordering::AcqRel) {
        0 => RC.store(0, Ordering::Release),
        1 => {
            unsafe {
                LIB = None;
            }
            RC.store(0, Ordering::Release);
        }
        usize::MAX => {
            while RC.load(Ordering::Acquire) == usize::MAX {
                hint::spin_loop();
            }
            fini();
        }
        x => RC.store(x - 1, Ordering::Release),
    }
}

/// wl_proxy_destroy
pub unsafe fn proxy_destroy(proxy: *mut Proxy) {
    (LIB.as_ref().unwrap().1.proxy_destroy)(proxy);
}

/// wl_proxy_add_listener
pub unsafe fn proxy_add_listener(
    proxy: *mut Proxy,
    implementation: *mut unsafe extern "C" fn(),
    data: *mut c_void,
) -> c_int {
    (LIB.as_ref().unwrap().1.proxy_add_listener)(proxy, implementation, data)
}

/// wl_proxy_get_listener
pub unsafe fn proxy_get_listener(proxy: *mut Proxy) -> *const c_void {
    (LIB.as_ref().unwrap().1.proxy_get_listener)(proxy)
}

/// wl_proxy_set_user_data
pub unsafe fn proxy_set_user_data(proxy: *mut Proxy, data: *mut c_void) {
    (LIB.as_ref().unwrap().1.proxy_set_user_data)(proxy, data)
}

/// wl_proxy_get_user_data
pub unsafe fn proxy_get_user_data(proxy: *mut Proxy) -> *mut c_void {
    (LIB.as_ref().unwrap().1.proxy_get_user_data)(proxy)
}

/// wl_proxy_get_version
pub unsafe fn proxy_get_version(proxy: *mut Proxy) -> u32 {
    (LIB.as_ref().unwrap().1.proxy_get_version)(proxy)
}

/// wl_proxy_get_id
pub unsafe fn proxy_get_id(proxy: *mut Proxy) -> u32 {
    (LIB.as_ref().unwrap().1.proxy_get_id)(proxy)
}

/// wl_display_connect
pub unsafe fn display_connect(name: *const c_char) -> *mut Display {
    (LIB.as_ref().unwrap().1.display_connect)(name)
}

/// wl_display_disconnect
pub unsafe fn display_disconnect(display: *mut Display) {
    (LIB.as_ref().unwrap().1.display_disconnect)(display);
}

/// wl_display_dispatch
pub unsafe fn display_dispatch(display: *mut Display) -> c_int {
    (LIB.as_ref().unwrap().1.display_dispatch)(display)
}

/// wl_display_dispatch_pending
pub unsafe fn display_dispatch_pending(display: *mut Display) -> c_int {
    (LIB.as_ref().unwrap().1.display_dispatch_pending)(display)
}

/// wl_display_flush
pub unsafe fn display_flush(display: *mut Display) -> c_int {
    (LIB.as_ref().unwrap().1.display_flush)(display)
}

/// wl_display_roundtrip
pub unsafe fn display_roundtrip(display: *mut Display) -> c_int {
    (LIB.as_ref().unwrap().1.display_roundtrip)(display)
}

/// wl_display_get_error
pub unsafe fn display_get_error(display: *mut Display) -> c_int {
    (LIB.as_ref().unwrap().1.display_get_error)(display)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;
    use std::thread;

    use crate::{fini, init, LIB, RC};

    #[test]
    #[ignore]
    // NOTE: This cannot run in parallel with other tests.
    fn crate_init_and_fini() {
        let eq = |x| {
            assert_eq!(x, RC.load(Ordering::Acquire));
            unsafe {
                if x > 0 {
                    assert!(LIB.is_some());
                } else {
                    assert!(LIB.is_none());
                }
            }
        };

        eq(0);
        init().unwrap();
        eq(1);
        fini();
        eq(0);
        init().unwrap();
        init().unwrap();
        eq(2);
        fini();
        eq(1);
        fini();
        eq(0);

        const N: usize = 15;
        let mut join = Vec::with_capacity(N);

        for _ in 0..N {
            join.push(thread::spawn(|| init().unwrap()));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        eq(N);

        for _ in 0..N {
            join.push(thread::spawn(fini));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        eq(0);

        for _ in 0..N {
            join.push(thread::spawn(|| {
                init().unwrap();
                fini();
            }));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        eq(0);
    }
}
