// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::thread;

use crate::gpu;

#[test]
fn init() {
    let mut join = vec![];
    for _ in 0..5 {
        join.push(thread::spawn(gpu::init));
    }
    for i in join {
        i.join().unwrap();
    }
}
