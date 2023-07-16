use std::ptr;

use crate::internal::NAdapter;
use crate::{Error, PowerPreference, Result};

pub(super) struct Adapter {
    inst: vk::Instance,
    // TODO
}

impl NAdapter for Adapter {
    // TODO
}

impl Adapter {
    pub(super) fn new(power_pref: PowerPreference) -> Result<Self> {
        let inst = match vk::Instance::new(/*...*/) {
            Ok(x) => x,
            Err(s) => return Err(Error::Internal(s)),
        };

        // TODO: Device selection.

        Ok(Self { inst })
    }
}
