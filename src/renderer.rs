//! Core rendering.

use std::io;

/// Configuration for `Renderer` creation.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Config {
    // TODO
}

impl Default for Config {
    /// Creates a default configuration.
    fn default() -> Self {
        todo!();
    }
}

/// Renderer.
#[derive(Debug)]
pub struct Renderer {
    // TODO
}

impl Renderer {
    /// Creates a new renderer using the default `Config`.
    pub fn new() -> io::Result<Renderer> {
        Self::new_config(Config::default())
    }

    /// Creates a new renderer using a given `Config`.
    #[allow(unused_variables)] // TODO
    pub fn new_config(config: Config) -> io::Result<Renderer> {
        todo!();
    }

    // TODO: Render targets, passes' setup, ...

    /// Returns whether the back-end device is hardware-accelerated.
    pub fn is_hw_accelerated(&self) -> bool {
        todo!();
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // TODO
        println!("[!] Renderer::drop does nothing currently");
    }
}
