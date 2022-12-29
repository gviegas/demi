// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use ::std::sync::atomic::Ordering;
use std::thread;

use crate::init::{self, GLOBAL_FP, PROC, RC};

#[test]
#[ignore]
// NOTE: This cannot run in parallel with other tests.
fn mt_init_and_fini() {
    const N: usize = 3;

    thread::spawn(init_and_fini_once).join().unwrap();
    init_and_fini_once();
    thread::spawn(do_init).join().unwrap();
    do_fini();

    let mut join = Vec::with_capacity(N);
    for _ in 0..N {
        join.push(thread::spawn(maybe_init));
    }
    while let Some(x) = join.pop() {
        x.join().unwrap();
    }
    assert_eq!(RC.load(Ordering::Acquire), N);
    for _ in 0..N {
        join.push(thread::spawn(maybe_fini));
    }
    while let Some(x) = join.pop() {
        x.join().unwrap();
    }
    assert_eq!(RC.load(Ordering::Acquire), 0);

    do_init();
    maybe_init();
    for _ in 0..N {
        join.push(thread::spawn(maybe_init));
    }
    join.push(thread::spawn(maybe_fini));
    while let Some(x) = join.pop() {
        x.join().unwrap();
    }
    assert_eq!(RC.load(Ordering::Acquire), 1 + N);
    for _ in 0..N {
        join.push(thread::spawn(maybe_fini));
    }
    while let Some(x) = join.pop() {
        x.join().unwrap();
    }
    assert_eq!(RC.load(Ordering::Acquire), 1);
    do_fini();

    const M: usize = N * 8;

    let mut join = Vec::with_capacity(M);
    for _ in 0..M {
        join.push(thread::spawn(|| {
            maybe_init();
            maybe_fini();
        }));
    }
    for i in join {
        i.join().unwrap();
    }
    unsafe {
        assert!(PROC.is_none());
        assert!(GLOBAL_FP.is_none());
    }
    assert_eq!(RC.load(Ordering::Acquire), 0);
}

fn init_and_fini_once() {
    unsafe {
        assert!(PROC.is_none());
        assert!(GLOBAL_FP.is_none());
        assert_eq!(RC.load(Ordering::SeqCst), 0);
        init::init().unwrap();
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert_eq!(RC.load(Ordering::SeqCst), 1);
        init::fini();
        assert!(PROC.is_none());
        assert!(GLOBAL_FP.is_none());
        assert_eq!(RC.load(Ordering::SeqCst), 0);
    }
}

fn do_init() {
    unsafe {
        assert!(PROC.is_none());
        assert!(GLOBAL_FP.is_none());
        assert_eq!(RC.load(Ordering::SeqCst), 0);
        init::init().unwrap();
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert!(RC.load(Ordering::SeqCst) >= 1);
    }
}

fn maybe_init() {
    unsafe {
        init::init().unwrap();
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert!(RC.load(Ordering::SeqCst) >= 1);
    }
}

fn do_fini() {
    unsafe {
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert_eq!(RC.load(Ordering::SeqCst), 1);
        init::fini();
        assert!(PROC.is_none());
        assert!(GLOBAL_FP.is_none());
        assert_eq!(RC.load(Ordering::SeqCst), 0);
    }
}

fn maybe_fini() {
    unsafe {
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert!(RC.load(Ordering::SeqCst) >= 1);
        init::fini();
    }
}
