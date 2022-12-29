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
    match RC.swap(usize::MAX, Ordering::AcqRel) {
        0 => {
            // NOTE: Initialization of global data must
            // be done here.
            gpu::init();
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
}

/// Finalizes the crate after use.
///
/// NOTE: The crate must not be used after calling this function.
pub fn shutdown() {
    match RC.swap(usize::MAX, Ordering::AcqRel) {
        0 => {
            RC.store(0, Ordering::Release);
        }
        1 => {
            // NOTE: Finalization of global data must
            // be done here.
            gpu::shutdown();
            RC.store(0, Ordering::Release);
        }
        usize::MAX => {
            while RC.load(Ordering::Acquire) == usize::MAX {
                hint::spin_loop();
            }
            return shutdown();
        }
        x => {
            RC.store(x - 1, Ordering::Release);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;
    use std::thread;

    use super::RC;

    #[test]
    #[ignore]
    // NOTE: This cannot run in parallel with other tests.
    fn crate_init_and_shutdown() {
        assert_eq!(0, RC.load(Ordering::Relaxed));
        crate::init();
        assert_eq!(1, RC.load(Ordering::Relaxed));
        crate::shutdown();
        assert_eq!(0, RC.load(Ordering::Relaxed));
        crate::init();
        crate::init();
        assert_eq!(2, RC.load(Ordering::Relaxed));
        crate::shutdown();
        assert_eq!(1, RC.load(Ordering::Relaxed));
        crate::shutdown();
        assert_eq!(0, RC.load(Ordering::Relaxed));

        const N: usize = 15;
        let mut join = Vec::with_capacity(N);

        for _ in 0..N {
            join.push(thread::spawn(crate::init));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        assert_eq!(N, RC.load(Ordering::Acquire));

        for _ in 0..N {
            join.push(thread::spawn(crate::shutdown));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        assert_eq!(0, RC.load(Ordering::Acquire));

        for _ in 0..N {
            join.push(thread::spawn(|| {
                crate::init();
                crate::shutdown();
            }));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        assert_eq!(0, RC.load(Ordering::Acquire));
    }
}
