//! Wrapper exposing a subset of the Vulkan API.

#![cfg(any(target_os = "linux", windows))]

mod instance;
pub use instance::*;
