use crate::{DeviceLostReason, Error, PowerPreference, Result};

use crate::internal::NAdapter;

mod adapter;
use adapter::*;

pub(super) fn new_adapter(power_pref: PowerPreference) -> Result<Box<dyn NAdapter>> {
    Ok(Box::new(Adapter::new(power_pref)?))
}

impl From<vk::Error> for Error {
    fn from(vk_err: vk::Error) -> Self {
        match vk_err.status {
            Some(vk::Status::DeviceLost) => {
                Error::DeviceLost(DeviceLostReason::Unknown, vk_err.description)
            }
            Some(vk::Status::OutOfHostMemory) | Some(vk::Status::OutOfDeviceMemory) => {
                Error::OutOfMemory(vk_err.description)
            }
            _ => Error::Internal(vk_err.description),
        }
    }
}
