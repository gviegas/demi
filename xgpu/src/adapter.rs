//! GPU adapter.

pub struct Adapter {
    // TODO
}

impl Adapter {
    pub fn features(&self) -> &SupportedFeatures {
        panic!("not yet implemented");
    }

    pub fn limits(&self) -> &SupportedLimits {
        panic!("not yet implemented");
    }

    pub fn info(&self) -> &AdapterInfo {
        panic!("not yet implemented");
    }

    // async
    pub fn request_device(self /*,desc: DeviceDescriptor*/) /* -> Device */
    {
        panic!("not yet implemented");
    }
}

// TODO
pub struct SupportedFeatures;

// TODO
pub struct SupportedLimits;

// TODO
pub struct AdapterInfo;
