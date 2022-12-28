// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::hint;
use std::sync::atomic::{AtomicUsize, Ordering};

pub mod animation;
pub mod drawable;
pub mod light;
pub mod linear;
pub mod material;
pub mod mesh;
pub mod renderer;
pub mod sampler;
pub mod scene;
pub mod shape;
pub mod skin;
pub mod texture;
pub mod transform;

mod gpu;

// NOTE: The main purpose of making this reference-counted
// is for parallel testing. Crate users are expected to
// pair one call to `init` with one call to `shutdown`.
static RC: AtomicUsize = AtomicUsize::new(0);

/// Initializes the crate for use.
///
/// Panics if initialization of any sub-module fails.
///
/// NOTE: The crate must not be used until this function completes.
pub fn init() {
    match RC.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(0) => {
            // NOTE: Initialization of global data must
            // be done here.
            gpu::init();
            RC.store(2, Ordering::SeqCst);
        }
        Err(1) => loop {
            match RC.load(Ordering::SeqCst) {
                0 => return init(),
                1 => (),
                _ => {
                    RC.fetch_add(1, Ordering::SeqCst);
                    break;
                }
            }
            hint::spin_loop();
        },
        _ => {
            RC.fetch_add(1, Ordering::SeqCst);
        }
    }
}

/// Finalizes the crate.
///
/// NOTE: The crate must not be used afterwards.
pub fn shutdown() {
    match RC.compare_exchange(2, 1, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(2) => {
            // NOTE: Finalization of global data must
            // be done here.
            gpu::shutdown();
            RC.store(0, Ordering::SeqCst);
        }
        Err(1) => loop {
            match RC.load(Ordering::SeqCst) {
                0 => break,
                1 => (),
                _ => return shutdown(),
            }
            hint::spin_loop();
        },
        Err(0) => (),
        _ => {
            RC.fetch_sub(1, Ordering::SeqCst);
        }
    }
}
