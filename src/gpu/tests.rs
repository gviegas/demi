// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::thread;

use crate::gpu::{self, IMPL};

#[test]
fn init() {
    assert!(unsafe { IMPL.is_none() });
    let mut join = vec![];
    for _ in 0..5 {
        join.push(thread::spawn(gpu::init));
    }
    for i in join {
        i.join().unwrap();
    }
    assert!(unsafe { IMPL.is_some() });
    gpu::shutdown();
    assert!(unsafe { IMPL.is_none() });
}
