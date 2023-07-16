use crate::{PowerPreference, Result};

use crate::internal::NAdapter;

mod adapter;
use adapter::*;

pub(super) fn new_adapter(power_pref: PowerPreference) -> Result<Box<dyn NAdapter>> {
    Ok(Box::new(Adapter::new(power_pref)?))
}
