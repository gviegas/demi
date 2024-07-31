use std::ffi::c_void;

use crate::{Bool32, PhysicalDevice, StructureType};

/// VkPhysicalDeviceFeatures
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tesselation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_bc: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image_2d: Bool32,
    pub sparse_residency_image_3d: Bool32,
    pub sparse_residency_2_samples: Bool32,
    pub sparse_residency_4_samples: Bool32,
    pub sparse_residency_8_samples: Bool32,
    pub sparse_residency_16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

/// PFN_vkGetPhysicalDeviceFeatures
pub(crate) type GetPhysicalDeviceFeatures =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, features: *mut PhysicalDeviceFeatures);

/// VkPhysicalDeviceFeatures2 (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}

/// PFN_vkGetPhysicalDeviceFeatures2 (v1.1)
pub(crate) type GetPhysicalDeviceFeatures2 =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, features: *mut PhysicalDeviceFeatures2);

/// VkPhysicalDeviceVulkan11Features (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceVulkan11Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub storage_buffer_16_bit_access: Bool32,
    pub uniform_and_storage_buffer_16_bit_access: Bool32,
    pub storage_push_constant_16: Bool32,
    pub storage_input_output_16: Bool32,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
    pub protected_memory: Bool32,
    pub sampler_ycbcr_conversion: Bool32,
    pub shader_draw_parameters: Bool32,
}

/// VkPhysicalDeviceVulkan12Features (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceVulkan12Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub sampler_mirror_clamp_to_edge: Bool32,
    pub draw_indirect_count: Bool32,
    pub storage_buffer_8_bit_access: Bool32,
    pub uniform_and_storage_buffer_8_bit_access: Bool32,
    pub storage_push_constant_8: Bool32,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
    pub descriptor_indexing: Bool32,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
    pub sampler_filter_minmax: Bool32,
    pub scalar_block_layout: Bool32,
    pub imageless_framebuffer: Bool32,
    pub uniform_buffer_standard_layout: Bool32,
    pub shader_subgroup_extended_types: Bool32,
    pub separate_depth_stencil_layouts: Bool32,
    pub host_query_reset: Bool32,
    pub timeline_semaphore: Bool32,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
    pub shader_output_viewport_index: Bool32,
    pub shader_output_layer: Bool32,
    pub subgroup_broadcast_dynamic_id: Bool32,
}

/// VkPhysicalDeviceVulkan13Features (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceVulkan13Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub robust_image_access: Bool32,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
    pub pipeline_creation_cache_control: Bool32,
    pub private_data: Bool32,
    pub shader_demote_to_helper_invocation: Bool32,
    pub shader_terminate_invocation: Bool32,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
    pub synchronization_2: Bool32,
    pub texture_compression_astc_hdr: Bool32,
    pub shader_zero_initialize_workgroup_memory: Bool32,
    pub dynamic_rendering: Bool32,
    pub shader_integer_dot_product: Bool32,
    pub maintenance_4: Bool32,
}

/// VkPhysicalDeviceVariablePointersFeatures (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceVariablePointersFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub variable_pointers_storage_buffer: Bool32,
    pub variable_pointers: Bool32,
}

/// VkPhysicalDeviceProtectedMemoryFeatures (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub protected_memory: Bool32,
}

/// VkPhysicalDeviceMultiviewFeatures (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub multiview: Bool32,
    pub multiview_geometry_shader: Bool32,
    pub multiview_tessellation_shader: Bool32,
}

/// VkPhysicalDeviceShaderAtomicInt64Features (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderAtomicInt64Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_buffer_int64_atomics: Bool32,
    pub shader_shared_int64_atomics: Bool32,
}

/// VkPhysicalDeviceShaderFloat16Int8Features (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderFloat16Int8Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_float16: Bool32,
    pub shader_int8: Bool32,
}

/// VkPhysicalDevice16BitStorageFeatures (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDevice16BitStorageFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub storage_buffer_16_bit_access: Bool32,
    pub uniform_and_storage_buffer_16_bit_access: Bool32,
    pub storage_push_constant_16: Bool32,
    pub storage_input_output_16: Bool32,
}

/// VkPhysicalDevice8BitStorageFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDevice8BitStorageFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub storage_buffer_8_bit_access: Bool32,
    pub uniform_and_storage_buffer_8_bit_access: Bool32,
    pub storage_push_constant_8: Bool32,
}

/// VkPhysicalDeviceShaderDrawParametersFeatures (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderDrawParametersFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_draw_parameters: Bool32,
}

/// VkPhysicalDeviceSamplerYcbcrConversionFeatures (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub sampler_ycbcr_conversion: Bool32,
}

/// VkPhysicalDeviceDescriptorIndexingFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_input_attachment_array_dynamic_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_texel_buffer_array_dynamic_indexing: Bool32,
    pub shader_uniform_buffer_array_non_uniform_indexing: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_image_array_non_uniform_indexing: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing: Bool32,
    pub shader_uniform_texel_buffer_array_non_uniform_indexing: Bool32,
    pub shader_storage_texel_buffer_array_non_uniform_indexing: Bool32,
    pub descriptor_binding_uniform_buffer_update_after_bind: Bool32,
    pub descriptor_binding_sampled_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_image_update_after_bind: Bool32,
    pub descriptor_binding_storage_buffer_update_after_bind: Bool32,
    pub descriptor_binding_uniform_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_storage_texel_buffer_update_after_bind: Bool32,
    pub descriptor_binding_update_unused_while_pending: Bool32,
    pub descriptor_binding_partially_bound: Bool32,
    pub descriptor_binding_variable_descriptor_count: Bool32,
    pub runtime_descriptor_array: Bool32,
}

/// VkPhysicalDeviceVulkanMemoryModelFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceVulkanMemoryModelFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub vulkan_memory_model: Bool32,
    pub vulkan_memory_model_device_scope: Bool32,
    pub vulkan_memory_model_availability_visibility_chains: Bool32,
}

/// VkPhysicalDeviceUniformBufferStandardLayoutFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub uniform_buffer_standard_layout: Bool32,
}

/// VkPhysicalDeviceScalarBlockLayoutFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceScalarBlockLayoutFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub scalar_block_layout: Bool32,
}

/// VkPhysicalDeviceBufferDeviceAddressFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceBufferDeviceAddressFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub buffer_device_address: Bool32,
    pub buffer_device_address_capture_replay: Bool32,
    pub buffer_device_address_multi_device: Bool32,
}

/// VkPhysicalDeviceImagelessFramebufferFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceImagelessFramebufferFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub imageless_framebuffer: Bool32,
}

/// VkPhysicalDeviceInlineUniformBlockFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub inline_uniform_block: Bool32,
    pub descriptor_binding_inline_uniform_block_update_after_bind: Bool32,
}

/// VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_subgroup_extended_types: Bool32,
}

/// VkPhysicalDeviceHostQueryResetFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceHostQueryResetFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub host_query_reset: Bool32,
}

/// VkPhysicalDeviceTimelineSemaphoreFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub timeline_semaphore: Bool32,
}

/// VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub separate_depth_stencil_layouts: Bool32,
}

/// VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_demote_to_helper_invocation: Bool32,
}

/// VkPhysicalDeviceTextureCompressionASTCHDRFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceTextureCompressionAstcHdrFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub texture_compression_astc_hdr: Bool32,
}

/// VkPhysicalDeviceSubgroupSizeControlFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub subgroup_size_control: Bool32,
    pub compute_full_subgroups: Bool32,
}

/// VkPhysicalDevicePipelineCreationCacheControlFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDevicePipelineCreationCacheControlFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub pipeline_creation_cache_control: Bool32,
}

/// VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_zero_initialize_workgroup_memory: Bool32,
}

/// VkPhysicalDevicePrivateDataFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDevicePrivateDataFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub private_data: Bool32,
}

/// VkPhysicalDeviceImageRobustnessFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceImageRobustnessFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub robust_image_access: Bool32,
}

/// VkPhysicalDeviceShaderTerminateInvocationFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderTerminateInvocationFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_terminate_invocation: Bool32,
}

/// VkPhysicalDeviceSynchronization2Features (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSynchronization2Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub synchronization_2: Bool32,
}

/// VkPhysicalDeviceShaderIntegerDotProductFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceShaderIntegerDotProductFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub shader_integer_dot_product: Bool32,
}

/// VkPhysicalDeviceMaintenance4Features (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4Features {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub maintenance_4: Bool32,
}

/// VkPhysicalDeviceDynamicRenderingFeatures (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceDynamicRenderingFeatures {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub dynamic_rendering: Bool32,
}

/// VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR (VK_KHR_global_priority)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKhr {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub global_priority_query: Bool32,
}

/// VkPhysicalDevicePortabilitySubsetFeaturesKHR (VK_KHR_portability_subset)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDevicePortabilitySubsetFeaturesKhr {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub constant_alpha_color_blend_factors: Bool32,
    pub events: Bool32,
    pub image_view_format_reinterpretation: Bool32,
    pub image_view_format_swizzle: Bool32,
    pub image_view_2d_on_3d_image: Bool32,
    pub multisample_array_image: Bool32,
    pub mutable_comparison_samplers: Bool32,
    pub point_polygons: Bool32,
    pub sampler_mip_lod_bias: Bool32,
    pub separate_stencil_mask_ref: Bool32,
    pub shader_sample_rate_interpolation_functions: Bool32,
    pub tessellation_isolines: Bool32,
    pub tessellation_point_mode: Bool32,
    pub triangle_fans: Bool32,
    pub vertex_attribute_access_beyond_stride: Bool32,
}
