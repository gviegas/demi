use crate::{DeviceLostReason, Error, PowerPreference, Result};

use crate::internal::NAdapter;

mod adapter;
use adapter::*;

pub(super) fn new_adapter(power_pref: PowerPreference) -> Result<Box<dyn NAdapter>> {
    Ok(Box::new(Adapter::new(power_pref)?))
}

impl From<vk::Error> for Error {
    fn from(vk_err: vk::Error) -> Self {
        match vk_err {
            vk::Error::DeviceLost => Error::DeviceLost(DeviceLostReason::Unknown, "Device lost"),
            vk::Error::OutOfHostMemory => Error::OutOfMemory("Out of host memory"),
            vk::Error::OutOfDeviceMemory => Error::OutOfMemory("Out of device memory"),
            _ => Error::Internal("Unknown"), // TODO: String from Error.
        }
    }
}
