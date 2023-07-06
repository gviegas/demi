//! GPU adapter.

use crate::{Device, DeviceDescriptor, Result};

pub struct Adapter {
    features: SupportedFeatures,
    limits: SupportedLimits,
    fallback: bool,
    _info: Option<AdapterInfo>,
    // TODO
}

impl Adapter {
    pub fn features(&self) -> &SupportedFeatures {
        &self.features
    }

    pub fn limits(&self) -> &SupportedLimits {
        &self.limits
    }

    pub fn is_fallback_adapter(&self) -> bool {
        self.fallback
    }

    // async
    pub fn request_adapter_info(&self) -> AdapterInfo {
        panic!("not yet implemented");
    }

    // async
    pub fn request_device(self, _desc: Option<&DeviceDescriptor>) -> Result<Device> {
        panic!("not yet implemented");
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Feature {
    DepthClipControl,
    Depth32FloatStencil8,
    TextureCompressionBc,
    TextureCompressionEtc2,
    TextureCompressionAstc,
    TimestampQuery,
    IndirectFirstInstance,
    ShaderFloat16,
    Rg11b10UfloatRenderable,
    Bgra8UnormStorage,
    Float32Filterable,
}
pub(crate) const MAX_FEATURE: usize = 1 + Feature::Float32Filterable as usize;

#[cfg_attr(test, derive(Default))] // TODO: Remove.
#[derive(Clone)]
pub struct SupportedFeatures([bool; MAX_FEATURE]);

impl SupportedFeatures {
    pub fn is_supported(&self, feature: Feature) -> bool {
        self.0[feature as usize]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Limit {
    MaxTextureDimension1d(u32),
    MaxTextureDimension2d(u32),
    MaxTextureDimension3d(u32),
    MaxTextureArrayLayers(u32),
    MaxBindGroups(u32),
    MaxBindGroupsPlusVertexBuffers(u32),
    MaxBindingsPerBindGroup(u32),
    MaxDynamicUniformBuffersPerPipelineLayout(u32),
    MaxDynamicStorageBuffersPerPipelineLayout(u32),
    MaxSampledTexturesPerShaderStage(u32),
    MaxSamplersPerShaderStage(u32),
    MaxStorageBuffersPerShaderStage(u32),
    MaxStorageTexturesPerShaderStage(u32),
    MaxUniformBuffersPerShaderStage(u32),
    MaxUniformBufferBindingSize(u64),
    MaxStorageBufferBindingSize(u64),
    MinUniformBufferOffsetAlignment(u32),
    MinStorageBufferOffsetAlignment(u32),
    MaxVertexBuffers(u32),
    MaxBufferSize(u64),
    MaxVertexAttributes(u32),
    MaxVertexBufferArrayStride(u32),
    MaxInterStageShaderComponents(u32),
    MaxInterStageShaderVariables(u32),
    MaxColorAttachments(u32),
    MaxColorAttachmentBytesPerSample(u32),
    MaxComputeWorkgroupStorageSize(u32),
    MaxComputeInvocationsPerWorkgroup(u32),
    MaxComputeWorkgroupSizeX(u32),
    MaxComputeWorkgroupSizeY(u32),
    MaxComputeWorkgroupSizeZ(u32),
    MaxComputeWorkgroupsPerDimension(u32),
}
pub(crate) const MAX_LIMIT: usize =
    1 + Limit::MaxComputeWorkgroupsPerDimension(0).discriminant() as usize;

impl Limit {
    pub(crate) const fn discriminant(&self) -> u8 {
        unsafe { *(self as *const Self as *const u8) }
    }
}

#[derive(Clone)]
pub struct SupportedLimits([u64; MAX_LIMIT]);

impl SupportedLimits {
    pub fn max_texture_dimension_1d(&self) -> u32 {
        const I: u8 = Limit::MaxTextureDimension1d(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_texture_dimension_2d(&self) -> u32 {
        const I: u8 = Limit::MaxTextureDimension2d(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_texture_dimension_3d(&self) -> u32 {
        const I: u8 = Limit::MaxTextureDimension3d(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_texture_array_layers(&self) -> u32 {
        const I: u8 = Limit::MaxTextureArrayLayers(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_bind_groups(&self) -> u32 {
        const I: u8 = Limit::MaxBindGroups(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_bind_groups_plus_vertex_buffers(&self) -> u32 {
        const I: u8 = Limit::MaxBindGroupsPlusVertexBuffers(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_bindings_per_bind_group(&self) -> u32 {
        const I: u8 = Limit::MaxBindingsPerBindGroup(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_dynamic_uniform_buffers_per_pipeline_layout(&self) -> u32 {
        const I: u8 = Limit::MaxDynamicUniformBuffersPerPipelineLayout(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_dynamic_storage_buffers_per_pipeline_layout(&self) -> u32 {
        const I: u8 = Limit::MaxDynamicStorageBuffersPerPipelineLayout(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_sampled_textures_per_shader_stage(&self) -> u32 {
        const I: u8 = Limit::MaxSampledTexturesPerShaderStage(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_samplers_per_shader_stage(&self) -> u32 {
        const I: u8 = Limit::MaxSamplersPerShaderStage(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_storage_buffers_per_shader_stage(&self) -> u32 {
        const I: u8 = Limit::MaxStorageBuffersPerShaderStage(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_storage_textures_per_shader_stage(&self) -> u32 {
        const I: u8 = Limit::MaxStorageTexturesPerShaderStage(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_uniform_buffers_per_shader_stage(&self) -> u32 {
        const I: u8 = Limit::MaxUniformBuffersPerShaderStage(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_uniform_buffer_binding_size(&self) -> u64 {
        const I: u8 = Limit::MaxUniformBufferBindingSize(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_storage_buffer_binding_size(&self) -> u64 {
        const I: u8 = Limit::MaxStorageBufferBindingSize(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn min_uniform_buffer_offset_alignment(&self) -> u32 {
        const I: u8 = Limit::MinUniformBufferOffsetAlignment(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn min_storage_buffer_offset_alignment(&self) -> u32 {
        const I: u8 = Limit::MinStorageBufferOffsetAlignment(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_vertex_buffers(&self) -> u32 {
        const I: u8 = Limit::MaxVertexBuffers(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_buffer_size(&self) -> u64 {
        const I: u8 = Limit::MaxBufferSize(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_vertex_attributes(&self) -> u32 {
        const I: u8 = Limit::MaxVertexAttributes(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_vertex_buffer_array_stride(&self) -> u32 {
        const I: u8 = Limit::MaxVertexBufferArrayStride(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_inter_stage_shader_components(&self) -> u32 {
        const I: u8 = Limit::MaxInterStageShaderComponents(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_inter_stage_shader_variables(&self) -> u32 {
        const I: u8 = Limit::MaxInterStageShaderVariables(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_color_attachments(&self) -> u32 {
        const I: u8 = Limit::MaxColorAttachments(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_color_attachment_bytes_per_sample(&self) -> u32 {
        const I: u8 = Limit::MaxColorAttachmentBytesPerSample(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_compute_workgroup_storage_size(&self) -> u32 {
        const I: u8 = Limit::MaxComputeWorkgroupStorageSize(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_compute_invocations_per_workgroup(&self) -> u32 {
        const I: u8 = Limit::MaxComputeInvocationsPerWorkgroup(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_compute_workgroup_size_x(&self) -> u32 {
        const I: u8 = Limit::MaxComputeWorkgroupSizeX(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_compute_workgroup_size_y(&self) -> u32 {
        const I: u8 = Limit::MaxComputeWorkgroupSizeY(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_compute_workgroup_size_z(&self) -> u32 {
        const I: u8 = Limit::MaxComputeWorkgroupSizeZ(0).discriminant();
        self.0[I as usize] as _
    }

    pub fn max_compute_workgroups_per_dimension(&self) -> u32 {
        const I: u8 = Limit::MaxComputeWorkgroupsPerDimension(0).discriminant();
        self.0[I as usize] as _
    }
}

impl Default for SupportedLimits {
    fn default() -> Self {
        Self([
            8192,      // MaxTextureDimension1d(u32)
            8192,      // MaxTextureDimension2d(u32)
            2048,      // MaxTextureDimension3d(u32)
            256,       // MaxTextureArrayLayers(u32)
            4,         // MaxBindGroups(u32)
            24,        // MaxBindGroupsPlusVertexBuffers(u32)
            1000,      // MaxBindingsPerBindGroup(u32)
            8,         // MaxDynamicUniformBuffersPerPipelineLayout(u32)
            4,         // MaxDynamicStorageBuffersPerPipelineLayout(u32)
            16,        // MaxSampledTexturesPerShaderStage(u32)
            16,        // MaxSamplersPerShaderStage(u32)
            8,         // MaxStorageBuffersPerShaderStage(u32)
            4,         // MaxStorageTexturesPerShaderStage(u32)
            12,        // MaxUniformBuffersPerShaderStage(u32)
            65536,     // MaxUniformBufferBindingSize(u64)
            134217728, // MaxStorageBufferBindingSize(u64)
            256,       // MinUniformBufferOffsetAlignment(u32)
            256,       // MinStorageBufferOffsetAlignment(u32)
            8,         // MaxVertexBuffers(u32)
            268435456, // MaxBufferSize(u64)
            16,        // MaxVertexAttributes(u32)
            2048,      // MaxVertexBufferArrayStride(u32)
            60,        // MaxInterStageShaderComponents(u32)
            16,        // MaxInterStageShaderVariables(u32)
            8,         // MaxColorAttachments(u32)
            32,        // MaxColorAttachmentBytesPerSample(u32)
            16384,     // MaxComputeWorkgroupStorageSize(u32)
            256,       // MaxComputeInvocationsPerWorkgroup(u32)
            256,       // MaxComputeWorkgroupSizeX(u32)
            256,       // MaxComputeWorkgroupSizeY(u32)
            64,        // MaxComputeWorkgroupSizeZ(u32)
            65535,     // MaxComputeWorkgroupsPerDimension(u32)
        ])
    }
}

#[derive(Clone)]
pub struct AdapterInfo {
    pub vendor: String,
    pub architecture: String,
    pub device: String,
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter() {
        let info = AdapterInfo {
            vendor: "unknown".to_string(),
            architecture: "unknown".to_string(),
            device: "unknown".to_string(),
            description: Default::default(),
        };

        // TODO: `Adapter::new`.
        let adap = Adapter {
            features: Default::default(), // NOTE: This is invalid.
            limits: Default::default(),
            fallback: false,
            _info: Some(info.clone()),
        };
        _ = adap.features();
        _ = adap.limits();
        _ = adap.is_fallback_adapter();
        _ = adap.request_adapter_info();
        _ = adap.request_device(Some(&DeviceDescriptor {
            required_features: &[Feature::TextureCompressionBc, Feature::TimestampQuery],
            required_limits: &[Limit::MaxColorAttachments(10)],
            ..Default::default()
        }));

        // TODO: `Adapter::new`.
        let adap = Adapter {
            features: Default::default(), // NOTE: This is invalid.
            limits: Default::default(),
            fallback: false,
            _info: Some(info),
        };
        _ = adap.request_device(None);
    }
}
