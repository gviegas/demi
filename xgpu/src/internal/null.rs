use crate::Result;

use crate::internal::NAdapter;

struct Null;

impl NAdapter for Null {
    // TODO
}

pub(super) fn new_adapter() -> Result<Box<dyn NAdapter>> {
    Ok(Box::new(Null))
}
