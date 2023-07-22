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
    pub(super) fn new(_power_pref: PowerPreference) -> Result<Self> {
        let inst = match vk::Instance::new(/*...*/) {
            Ok(x) => x,
            Err(err) => return Err(err.into()),
        };

        // TODO: Device selection.

        Ok(Self { inst })
    }
}
