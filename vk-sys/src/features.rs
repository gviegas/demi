// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_void;

use crate::{Bool32, PhysicalDevice, StructureType};

// TODO: v1.1+ feature structs.

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

/// VkPhysicalDeviceFeatures2
/// [v1.1]
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceFeatures2 {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub features: PhysicalDeviceFeatures,
}

/// PFN_vkGetPhysicalDeviceFeatures2
/// [v1.1]
pub(crate) type GetPhysicalDeviceFeatures2 =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, features: *mut PhysicalDeviceFeatures2);
