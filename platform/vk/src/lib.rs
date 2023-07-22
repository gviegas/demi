//! Wrapper exposing a subset of the Vulkan API.

#![cfg(any(target_os = "linux", windows))]

use std::result;
use vk_sys as vks;

mod instance;
pub use instance::*;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub status: Option<Status>,
    pub description: &'static str,
}

impl Error {
    pub fn new(status: Option<Status>, description: &'static str) -> Self {
        Self {
            status,
            description,
        }
    }

    pub fn status(status: Status, description: &'static str) -> Self {
        Self::new(Some(status), description)
    }
}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Self {
            status: Some(status),
            description: "",
        }
    }
}

impl From<&'static str> for Error {
    fn from(description: &'static str) -> Self {
        Self {
            status: None,
            description,
        }
    }
}

impl From<vks::Result> for Error {
    fn from(result: vks::Result) -> Self {
        <vks::Result as Into<Status>>::into(result).into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Status {
    Success = 0,
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
}

impl From<vks::Result> for Status {
    fn from(result: vks::Result) -> Self {
        match result {
            vks::SUCCESS => Status::Success,
            vks::NOT_READY => Status::NotReady,
            vks::TIMEOUT => Status::Timeout,
            vks::EVENT_SET => Status::EventSet,
            vks::EVENT_RESET => Status::EventReset,
            vks::INCOMPLETE => Status::Incomplete,
            vks::ERROR_OUT_OF_HOST_MEMORY => Status::OutOfHostMemory,
            vks::ERROR_OUT_OF_DEVICE_MEMORY => Status::OutOfDeviceMemory,
            vks::ERROR_INITIALIZATION_FAILED => Status::InitializationFailed,
            vks::ERROR_DEVICE_LOST => Status::DeviceLost,
            vks::ERROR_MEMORY_MAP_FAILED => Status::MemoryMapFailed,
            vks::ERROR_LAYER_NOT_PRESENT => Status::LayerNotPresent,
            vks::ERROR_EXTENSION_NOT_PRESENT => Status::ExtensionNotPresent,
            vks::ERROR_FEATURE_NOT_PRESENT => Status::FeatureNotPresent,
            vks::ERROR_INCOMPATIBLE_DRIVER => Status::IncompatibleDriver,
            vks::ERROR_TOO_MANY_OBJECTS => Status::TooManyObjects,
            vks::ERROR_FORMAT_NOT_SUPPORTED => Status::FormatNotSupported,
            vks::ERROR_FRAGMENTED_POOL => Status::FragmentedPool,
            vks::ERROR_UNKNOWN => Status::Unknown,
            vks::ERROR_OUT_OF_POOL_MEMORY => Status::OutOfPoolMemory,
            vks::ERROR_INVALID_EXTERNAL_HANDLE => Status::InvalidExternalHandle,
            vks::ERROR_FRAGMENTATION => Status::Fragmentation,
            vks::ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS => Status::InvalidOpaqueCaptureAddress,
            vks::PIPELINE_COMPILE_REQUIRED => Status::PipelineCompileRequired,
            vks::ERROR_SURFACE_LOST_KHR => Status::SurfaceLost,
            vks::ERROR_NATIVE_WINDOW_IN_USE_KHR => Status::NativeWindowInUse,
            vks::SUBOPTIMAL_KHR => Status::Suboptimal,
            vks::ERROR_OUT_OF_DATE_KHR => Status::OutOfDate,
            vks::ERROR_INCOMPATIBLE_DISPLAY_KHR => Status::IncompatibleDriver,
            vks::ERROR_VALIDATION_FAILED_EXT => Status::ValidationFailed,
            vks::ERROR_INVALID_SHADER_NV => Status::InvalidShader,
            vks::ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT => {
                Status::InvalidDrmFormatModifierPlaneLayout
            }
            vks::ERROR_NOT_PERMITTED_KHR => Status::NotPermitted,
            vks::ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT => Status::FullScreenExclusiveModeLost,
            vks::THREAD_IDLE_KHR => Status::ThreadIdle,
            vks::THREAD_DONE_KHR => Status::ThreadDone,
            vks::OPERATION_DEFERRED_KHR => Status::OperationDeferred,
            vks::OPERATION_NOT_DEFERRED_KHR => Status::OperationNotDeferred,
            vks::ERROR_COMPRESSION_EXHAUSTED_EXT => Status::CompressionExhausted,
            _ => Status::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result() {
        let e = err().unwrap_err();
        assert_eq!(e.status, Some(Status::OutOfDeviceMemory));
        assert_eq!(e.description, "OODM!");

        let e = err_status().unwrap_err();
        assert_eq!(e.status, Some(Status::DeviceLost));
        assert_eq!(e.description, "");

        let e = err_description().unwrap_err();
        assert_eq!(e.status, None);
        assert_eq!(e.description, "!fAiLeD!");

        let e = err_result().unwrap_err();
        assert_eq!(e.status, Some(Status::Timeout));
        assert_eq!(e.description, "");

        fn err() -> Result<()> {
            Err(Error::status(Status::OutOfDeviceMemory, "OODM!"))
        }

        fn err_status() -> Result<()> {
            Err(Status::DeviceLost.into())
        }

        fn err_description() -> Result<()> {
            Err("!fAiLeD!".into())
        }

        fn err_result() -> Result<()> {
            Err(vks::TIMEOUT.into())
        }
    }
}
