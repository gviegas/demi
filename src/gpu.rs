// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

use std::sync::Once;

#[cfg(test)]
mod tests;

mod vk;

static mut IMPL: Option<Box<dyn Gpu>> = None;

/// Initializes the underlying `Gpu` implementation.
///
/// Panics if all back-ends fail to initialize.
#[cfg(any(target_os = "linux", windows))]
pub fn init() {
    static INIT: Once = Once::new();
    INIT.call_once(|| unsafe {
        IMPL = vk::init();
        IMPL.is_none()
            .then(|| panic!("no graphics back-end that we can use"));
    });
}

/// Graphics back-end interface.
pub trait Gpu {}
