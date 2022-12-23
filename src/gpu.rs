// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Interface to the graphics back-end.

#[cfg(test)]
mod tests;

mod vk;

/// Initializes the underlying `Gpu` implementation.
pub fn init() -> Option<Box<dyn Gpu>> {
    // TODO: This won't compile on platforms where
    // `vk_sys` isn't supported.
    if cfg!(any(target_os = "linux", windows)) {
        vk::init()
    } else {
        None
    }
}

/// Graphics back-end interface.
pub trait Gpu {}
