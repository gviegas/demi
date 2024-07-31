use crate::c_size_t;
//use core::ffi::c_size_t;
use std::ffi::c_void;

use crate::{Bool32, ResolveModeFlags, SampleCountFlags, ShaderStageFlags, StructureType};

/// VkPhysicalDeviceLimits
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: u64,
    pub sparse_address_space_size: u64,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: c_size_t,
    pub min_texel_buffer_offset_alignment: u64,
    pub min_uniform_buffer_offset_alignment: u64,
    pub min_storage_buffer_offset_alignment: u64,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: Bool32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: Bool32,
    pub standard_sample_locations: Bool32,
    pub optimal_buffer_copy_offset_alignment: u64,
    pub optimal_buffer_copy_row_pitch_alignment: u64,
    pub non_coherent_atom_size: u64,
}

/// VkPhysicalDeviceSparseProperties
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: Bool32,
    pub residency_standard_2d_multisample_block_shape: Bool32,
    pub residency_standard_3d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

/// VkPhysicalDeviceMaintenance3Properties (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance3Properties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_per_set_descriptors: u32,
    pub max_memory_allocation_size: u64,
}

/// VkPhysicalDeviceMaintenance4Properties (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceMaintenance4Properties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_buffer_size: u64,
}

/// VkPhysicalDeviceMultiviewProperties (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_multiview_view_count: u32,
    pub max_multiview_instance_index: u32,
}

/// VkPhysicalDeviceProtectedMemoryProperties (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceProtectedMemoryProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub protected_no_fault: Bool32,
}

/// VkPhysicalDeviceTimelineSemaphoreProperties (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceTimelineSemaphoreProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_timeline_semaphore_value_difference: u64,
}

/// VkPhysicalDeviceSubgroupProperties (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub subgroup_size: u32,
    pub supported_stages: ShaderStageFlags,
    pub supported_operations: SubgroupFeatureFlags,
    pub quad_operations_in_all_stages: Bool32,
}

def_flags!(
    SubgroupFeatureFlags,
    SubgroupFeatureFlagBits,
    SUBGROUP_FEATURE_BASIC_BIT = 0x00000001,
    SUBGROUP_FEATURE_VOTE_BIT = 0x00000002,
    SUBGROUP_FEATURE_ARITHMETIC_BIT = 0x00000004,
    SUBGROUP_FEATURE_BALLOT_BIT = 0x00000008,
    SUBGROUP_FEATURE_SHUFFLE_BIT = 0x00000010,
    SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = 0x00000020,
    SUBGROUP_FEATURE_CLUSTERED_BIT = 0x00000040,
    SUBGROUP_FEATURE_QUAD_BIT = 0x00000080,
    SUBGROUP_FEATURE_PARTITIONED_BIT_NV = 0x00000100
);

/// VkPhysicalDeviceSubgroupSizeControlProperties (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSubgroupSizeControlProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub min_subgroup_size: u32,
    pub max_subgroup_size: u32,
    pub max_compute_workgroup_subgroups: u32,
    pub required_subgroup_size_stages: ShaderStageFlags,
}

/// VkPhysicalDeviceFloatControlsProperties (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceFloatControlsProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub denorm_behavior_independence: ShaderFloatControlsIndependence,
    pub rounding_mode_independence: ShaderFloatControlsIndependence,
    pub shader_signed_zero_inf_nan_preserve_float16: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float32: Bool32,
    pub shader_signed_zero_inf_nan_preserve_float64: Bool32,
    pub shader_denorm_preserve_float16: Bool32,
    pub shader_denorm_preserve_float32: Bool32,
    pub shader_denorm_preserve_float64: Bool32,
    pub shader_denorm_flush_to_zero_float16: Bool32,
    pub shader_denorm_flush_to_zero_float32: Bool32,
    pub shader_denorm_flush_to_zero_float64: Bool32,
    pub shader_rounding_mode_rte_float16: Bool32,
    pub shader_rounding_mode_rte_float32: Bool32,
    pub shader_rounding_mode_rte_float64: Bool32,
    pub shader_rounding_mode_rtz_float16: Bool32,
    pub shader_rounding_mode_rtz_float32: Bool32,
    pub shader_rounding_mode_rtz_float64: Bool32,
}

def_ids!(
    ShaderFloatControlsIndependence,
    SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY = 0,
    SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL = 1,
    SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE = 2,
    SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY_KHR =
        SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY,
    SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL_KHR = SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL,
    SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE_KHR = SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE
);

/// VkPhysicalDeviceSamplerFilterMinmaxProperties (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceSamplerFilterMinmaxProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub filter_minmax_single_component_formats: Bool32,
    pub filter_minmax_image_component_mapping: Bool32,
}

/// VkPhysicalDeviceDescriptorIndexingProperties (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceDescriptorIndexingProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_update_after_bind_descriptors_in_all_pools: u32,
    pub shader_uniform_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_sampled_image_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_buffer_array_non_uniform_indexing_native: Bool32,
    pub shader_storage_image_array_non_uniform_indexing_native: Bool32,
    pub shader_input_attachment_array_non_uniform_indexing_native: Bool32,
    pub robust_buffer_access_update_after_bind: Bool32,
    pub quad_divergent_implicit_lod: Bool32,
    pub max_per_stage_descriptor_update_after_bind_samplers: u32,
    pub max_per_stage_descriptor_update_after_bind_uniform_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_buffers: u32,
    pub max_per_stage_descriptor_update_after_bind_sampled_images: u32,
    pub max_per_stage_descriptor_update_after_bind_storage_images: u32,
    pub max_per_stage_descriptor_update_after_bind_input_attachments: u32,
    pub max_per_stage_update_after_bind_resources: u32,
    pub max_descriptor_set_update_after_bind_samplers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers: u32,
    pub max_descriptor_set_update_after_bind_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers: u32,
    pub max_descriptor_set_update_after_bind_storage_buffers_dynamic: u32,
    pub max_descriptor_set_update_after_bind_sampled_images: u32,
    pub max_descriptor_set_update_after_bind_storage_images: u32,
    pub max_descriptor_set_update_after_bind_input_attachments: u32,
}

/// VkPhysicalDeviceInlineUniformBlockProperties (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceInlineUniformBlockProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_inline_uniform_block_size: u32,
    pub max_per_stage_descriptor_inline_uniform_blocks: u32,
    pub max_per_stage_descriptor_update_after_bind_inline_uniform_blocks: u32,
    pub max_descriptor_set_inline_uniform_blocks: u32,
    pub max_descriptor_set_update_after_bind_inline_uniform_blocks: u32,
}

/// VkPhysicalDeviceDepthStencilResolveProperties (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceDepthStencilResolveProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub supported_depth_resolve_modes: ResolveModeFlags,
    pub supported_stencil_resolve_modes: ResolveModeFlags,
    pub independent_resolve_none: Bool32,
    pub independent_resolve: Bool32,
}

/// VkPhysicalDeviceTexelBufferAlignmentProperties (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceTexelBufferAlignmentProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub storage_texel_buffer_offset_alignment_bytes: u64,
    pub storage_texel_buffer_offset_single_texel_alignment: Bool32,
    pub uniform_texel_buffer_offset_alignment_bytes: u64,
    pub uniform_texel_buffer_offset_single_texel_alignment: Bool32,
}
