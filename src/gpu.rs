// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

#[cfg(test)]
mod tests;

mod vk;

static mut IMPL: Option<Box<dyn Gpu>> = None;

/// Initializes the underlying implementation.
///
/// Panics if all back-ends fail to initialize.
///
/// NOTE: One must ensure this function is called exactly once,
/// before any `gpu` functionality is used. It is not safe to
/// call it from multiple threads.
#[cfg(any(target_os = "linux", windows))]
pub fn init() {
    unsafe {
        IMPL = vk::Impl::new();
        IMPL.is_none()
            .then(|| panic!("no graphics back-end that we can use"));
    }
}

/// Drops the underlying implementation.
///
/// NOTE: One must ensure this function is called exactly once,
/// after all uses of `gpu`. It is not safe to call it from
/// multiple threads.
pub fn shutdown() {
    unsafe {
        IMPL = None;
    }
}

/// Graphics back-end interface.
// TODO
trait Gpu {}
