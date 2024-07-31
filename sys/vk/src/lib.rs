//! Wrapper exposing a subset of the Vulkan API.

#![cfg(any(target_os = "linux", windows))]

use std::result;
use vk_sys as vks;

mod instance;
pub use instance::*;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    OutOfHostMemory = -1,
    OutOfDeviceMemory = -2,
    InitializationFailed = -3,
    DeviceLost = -4,
    MemoryMapFailed = -5,
    LayerNotPresent = -6,
    ExtensionNotPresent = -7,
    FeatureNotPresent = -8,
    IncompatibleDriver = -9,
    TooManyObjects = -10,
    FormatNotSupported = -11,
    FragmentedPool = -12,
    Unknown = -13,
    OutOfPoolMemory = -1000069000,
    InvalidExternalHandle = -1000072003,
    Fragmentation = -1000161000,
    InvalidOpaqueCaptureAddress = -1000257000,
    PipelineCompileRequired = 1000297000,
    SurfaceLost = -1000000000,
    NativeWindowInUse = -1000000001,
    Suboptimal = 1000001003,
    OutOfDate = -1000001004,
    IncompatibleDisplay = -1000003001,
    ValidationFailed = -1000011001,
    InvalidShader = -1000012000,
    InvalidDrmFormatModifierPlaneLayout = -1000158000,
    NotPermitted = -1000174001,
    FullScreenExclusiveModeLost = -1000255000,
    ThreadIdle = 1000268000,
    ThreadDone = 1000268001,
    OperationDeferred = 1000268002,
    OperationNotDeferred = 1000268003,
    CompressionExhausted = -1000338000,
    Other = 0,
}

impl Default for Error {
    fn default() -> Self {
        Error::Other
    }
}

impl TryFrom<vks::Result> for Error {
    type Error = &'static str;

    fn try_from(result: vks::Result) -> result::Result<Self, Self::Error> {
        match result {
            vks::SUCCESS => Err("`VK_SUCCESS` is not an `Error`"),
            vks::NOT_READY => Ok(Error::NotReady),
            vks::TIMEOUT => Ok(Error::Timeout),
            vks::EVENT_SET => Ok(Error::EventSet),
            vks::EVENT_RESET => Ok(Error::EventReset),
            vks::INCOMPLETE => Ok(Error::Incomplete),
            vks::ERROR_OUT_OF_HOST_MEMORY => Ok(Error::OutOfHostMemory),
            vks::ERROR_OUT_OF_DEVICE_MEMORY => Ok(Error::OutOfDeviceMemory),
            vks::ERROR_INITIALIZATION_FAILED => Ok(Error::InitializationFailed),
            vks::ERROR_DEVICE_LOST => Ok(Error::DeviceLost),
            vks::ERROR_MEMORY_MAP_FAILED => Ok(Error::MemoryMapFailed),
            vks::ERROR_LAYER_NOT_PRESENT => Ok(Error::LayerNotPresent),
            vks::ERROR_EXTENSION_NOT_PRESENT => Ok(Error::ExtensionNotPresent),
            vks::ERROR_FEATURE_NOT_PRESENT => Ok(Error::FeatureNotPresent),
            vks::ERROR_INCOMPATIBLE_DRIVER => Ok(Error::IncompatibleDriver),
            vks::ERROR_TOO_MANY_OBJECTS => Ok(Error::TooManyObjects),
            vks::ERROR_FORMAT_NOT_SUPPORTED => Ok(Error::FormatNotSupported),
            vks::ERROR_FRAGMENTED_POOL => Ok(Error::FragmentedPool),
            vks::ERROR_UNKNOWN => Ok(Error::Unknown),
            vks::ERROR_OUT_OF_POOL_MEMORY => Ok(Error::OutOfPoolMemory),
            vks::ERROR_INVALID_EXTERNAL_HANDLE => Ok(Error::InvalidExternalHandle),
            vks::ERROR_FRAGMENTATION => Ok(Error::Fragmentation),
            vks::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => Ok(Error::InvalidOpaqueCaptureAddress),
            vks::PIPELINE_COMPILE_REQUIRED => Ok(Error::PipelineCompileRequired),
            vks::ERROR_SURFACE_LOST_KHR => Ok(Error::SurfaceLost),
            vks::ERROR_NATIVE_WINDOW_IN_USE_KHR => Ok(Error::NativeWindowInUse),
            vks::SUBOPTIMAL_KHR => Ok(Error::Suboptimal),
            vks::ERROR_OUT_OF_DATE_KHR => Ok(Error::OutOfDate),
            vks::ERROR_INCOMPATIBLE_DISPLAY_KHR => Ok(Error::IncompatibleDriver),
            vks::ERROR_VALIDATION_FAILED_EXT => Ok(Error::ValidationFailed),
            vks::ERROR_INVALID_SHADER_NV => Ok(Error::InvalidShader),
            vks::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
                Ok(Error::InvalidDrmFormatModifierPlaneLayout)
            }
            vks::ERROR_NOT_PERMITTED_KHR => Ok(Error::NotPermitted),
            vks::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => {
                Ok(Error::FullScreenExclusiveModeLost)
            }
            vks::THREAD_IDLE_KHR => Ok(Error::ThreadIdle),
            vks::THREAD_DONE_KHR => Ok(Error::ThreadDone),
            vks::OPERATION_DEFERRED_KHR => Ok(Error::OperationDeferred),
            vks::OPERATION_NOT_DEFERRED_KHR => Ok(Error::OperationNotDeferred),
            vks::ERROR_COMPRESSION_EXHAUSTED_EXT => Ok(Error::CompressionExhausted),
            _ => Err("Undefined error code"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result() {
        let e = err().unwrap_err();
        assert_eq!(e, Error::OutOfDeviceMemory);

        let e = success_is_not_err();
        assert!(e.is_err());
        assert_eq!(e.unwrap_or_default(), Error::Other);

        fn err() -> Result<()> {
            Err(vks::ERROR_OUT_OF_DEVICE_MEMORY.try_into().unwrap())
        }

        fn success_is_not_err() -> result::Result<Error, &'static str> {
            vks::SUCCESS.try_into()
        }
    }
}
