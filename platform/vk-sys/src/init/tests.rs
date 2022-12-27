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
    assert_eq!(RC.load(Ordering::Acquire), 2 + N - 1);
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
    assert_eq!(RC.load(Ordering::Acquire), 2 + N);
    for _ in 0..N {
        join.push(thread::spawn(maybe_fini));
    }
    while let Some(x) = join.pop() {
        x.join().unwrap();
    }
    assert_eq!(RC.load(Ordering::Acquire), 2);
    do_fini();
}

fn init_and_fini_once() {
    unsafe {
        assert!(PROC.is_none());
        assert!(GLOBAL_FP.is_none());
        assert_eq!(RC.load(Ordering::SeqCst), 0);
        init::init().unwrap();
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert_eq!(RC.load(Ordering::SeqCst), 2);
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
        assert!(RC.load(Ordering::SeqCst) >= 2);
    }
}

fn maybe_init() {
    unsafe {
        init::init().unwrap();
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert!(RC.load(Ordering::SeqCst) >= 2);
    }
}

fn do_fini() {
    unsafe {
        assert!(PROC.is_some());
        assert!(GLOBAL_FP.is_some());
        assert_eq!(RC.load(Ordering::SeqCst), 2);
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
        assert!(RC.load(Ordering::SeqCst) >= 2);
        init::fini();
    }
}
