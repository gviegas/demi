use crate::{internal, RequestAdapterOptions, Result};

pub struct Adapter {
    inner: Box<dyn NAdapter>,
}

impl Adapter {
    pub fn new(options: &RequestAdapterOptions) -> Result<Self> {
        Ok(Self {
            inner: internal::new_nadapter(options)?,
        })
    }
}

pub(super) trait NAdapter {
    // TODO
}
