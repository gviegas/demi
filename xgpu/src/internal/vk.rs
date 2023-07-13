use crate::{PowerPreference, Result};

use crate::internal::NAdapter;

struct Adapter {
    // TODO
}

impl NAdapter for Adapter {
    // TODO
}

pub(super) fn new_adapter(power_pref: PowerPreference) -> Result<Box<dyn NAdapter>> {
    panic!("not yet implemented");
}
