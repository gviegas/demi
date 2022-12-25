// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

use std::sync::Once;

#[cfg(test)]
mod tests;

mod vk;

static mut IMPL: Option<Box<dyn Gpu>> = None;

/// Initializes the underlying implementation.
///
/// Panics if all back-ends fail to initialize.
#[cfg(any(target_os = "linux", windows))]
pub fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        IMPL = vk::Impl::new();
        IMPL.is_none()
            .then(|| panic!("no graphics back-end that we can use"));
    });
}

/// Drops the underlying implementation.
///
/// NOTE: One must ensure this function is called only once,
/// just before the program terminates.
pub fn shutdown() {
    unsafe {
        IMPL = None;
    }
}

/// Graphics back-end interface.
// TODO
trait Gpu {}
