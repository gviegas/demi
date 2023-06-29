//! GPU errors.

use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    DeviceLost(DeviceLostReason, &'static str),
    Validation(&'static str),
    OutOfMemory(&'static str),
    Internal(&'static str),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum DeviceLostReason {
    Unknown,
    Destroyed,
}
