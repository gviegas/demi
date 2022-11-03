// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::{c_char, c_void};

use crate::{c_size_t, AllocationCallbacks, Bool32, Extent3d, Result, StructureType};

def_dh!(InstanceT, Instance);

// vkEnumerateInstanceVersion
type EnumerateInstanceVersion = unsafe extern "C" fn(api_version: *mut u32) -> Result;

/// VkInstanceCreateInfo
#[repr(C)]
pub struct InstanceCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const c_char,
}

def_flags!(InstanceCreateFlags, InstanceCreateFlagBits,);

/// VkApplicationInfo
#[repr(C)]
pub struct ApplicationInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub application_name: *const c_char,
    pub application_version: u32,
    pub engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

// vkCreateInstance
type CreateInstance = unsafe extern "C" fn(
    info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut Instance,
) -> Result;

// vkDestroyInstance
type DestroyInstance =
    unsafe extern "C" fn(instance: Instance, allocator: *const AllocationCallbacks);

def_dh!(PhysicalDeviceT, PhysicalDevice);

/// VkPhysicalDeviceProperties
#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [c_char; 256],
    pub pipeline_cache_uuid: [u8; 16],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

def_ids!(
    PhysicalDeviceType,
    PHYSICAL_DEVICE_TYPE_OTHER = 0,
    PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    PHYSICAL_DEVICE_TYPE_CPU = 4
);

/// VkPhysicalDeviceLimits
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
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: Bool32,
    pub residency_standard_2d_multisample_block_shape: Bool32,
    pub residency_standard_3d_block_shape: Bool32,
    pub residency_aligned_mip_size: Bool32,
    pub residency_non_resident_strict: Bool32,
}

/// VkQueueFamilyProperties
#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3d,
}

def_flags!(
    QueueFlags,
    QueueFlagBits,
    QUEUE_GRAPHICS_BIT = 0x00000001,
    QUEUE_COMPUTE_BIT = 0x00000002,
    QUEUE_TRANSFER_BIT = 0x00000004,
    QUEUE_SPARSE_BINDING_BIT = 0x00000008,
    QUEUE_PROTECTED_BIT = 0x00000010,
    QUEUE_OPTICAL_FLOW_BIT_NV = 0x00000100
);

/// VkPhysicalDeviceMemoryProperties
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; 16],
}

/// VkMemoryHeap
#[repr(C)]
pub struct MemoryHeap {
    pub size: u64,
    pub flags: MemoryHeapFlags,
}

/// VkMemoryType
#[repr(C)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

def_flags!(
    MemoryHeapFlags,
    MemoryHeapFlagBits,
    MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,
    MEMORY_HEAP_MULTI_INSTANCE_BIT = 0x00000002,
    MEMORY_HEAP_MULTI_INSTANCE_BIT_KHR = MEMORY_HEAP_MULTI_INSTANCE_BIT
);

def_flags!(
    MemoryPropertyFlags,
    MemoryPropertyFlagBits,
    MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
    MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
    MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
    MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
    MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
    MEMORY_PROPERTY_PROTECTED_BIT = 0x00000020,
    MEMORY_PROPERTY_DEVICE_COHERENT_BIT_AMD = 0x00000040,
    MEMORY_PROPERTY_DEVICE_UNCACHED_BIT_AMD = 0x00000080,
    MEMORY_PROPERTY_RDMA_CAPABLE_BIT_NV = 0x00000100
);

/// VkPhysicalDeviceFeatures
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

// vkEnumeratePhysicalDevices
type EnumeratePhysicalDevices = unsafe extern "C" fn(
    instance: Instance,
    count: *mut u32,
    phys_devs: *mut PhysicalDevice,
) -> Result;

// vkGetPhysicalDeviceProperties
type GetPhysicalDeviceProperties =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, properties: *const PhysicalDeviceProperties);

// vkGetPhysicalDeviceQueueFamilyProperties
type GetPhysicalDeviceQueueFamilyProperties = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    count: *mut u32,
    queue_props: *mut QueueFamilyProperties,
);

// vkGetPhysicalDeviceMemoryProperties
type GetPhysicalDeviceMemoryProperties =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, mem_props: *mut PhysicalDeviceMemoryProperties);

// vkGetPhysicalDeviceFeatures
type GetPhysicalDeviceFeatures =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, features: *mut PhysicalDeviceFeatures);

def_dh!(DeviceT, Device);

/// VkDeviceCreateInfo
#[repr(C)]
pub struct DeviceCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: DeviceCreateFlags,
    pub queue_create_info_count: u32,
    pub queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const c_char,
    pub enabled_features: *const PhysicalDeviceFeatures,
}

def_flags!(DeviceCreateFlags, DeviceCreateFlagBits,);

/// VkDeviceQueueCreateInfo
#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: DeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub queue_priorities: *const f32,
}

def_flags!(
    DeviceQueueCreateFlags,
    DeviceQueueCreateFlagBits,
    DEVICE_QUEUE_CREATE_PROTECTED_BIT = 0x00000001
);

// vkCreateDevice
type CreateDevice = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    info: *const DeviceCreateInfo,
    allocator: *const AllocationCallbacks,
    device: *mut Device,
) -> Result;

// vkDestroyDevice
type DestroyDevice = unsafe extern "C" fn(device: Device, allocator: *const AllocationCallbacks);

def_dh!(QueueT, Queue);

/// VkSubmitInfo
#[repr(C)]
pub struct SubmitInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *const Semaphore,
    pub wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub command_buffers: *const CommandBuffer,
    pub signal_semaphore_count: u32,
    pub signal_semaphores: *const Semaphore,
}

// vkGetDeviceQueue
type GetDeviceQueue =
    unsafe extern "C" fn(device: Device, fam_idx: u32, queue_idx: u32, queue: *mut Queue);

// vkQueueSubmit
type QueueSubmit = unsafe extern "C" fn(
    queue: Queue,
    submit_count: u32,
    submits: *const SubmitInfo,
    fence: Fence,
) -> Result;

def_dh!(CommandBufferT, CommandBuffer);

/// VkCommandBufferBeginInfo
#[repr(C)]
pub struct CommandBufferBeginInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: CommandBufferUsageFlags,
    pub inheritance_info: *const CommandBufferInheritanceInfo,
}

def_flags!(
    CommandBufferUsageFlags,
    CommandBufferUsageFlagBits,
    COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
    COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
    COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004
);

/// VkCommandBufferInheritanceInfo
#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub framebuffer: Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

def_flags!(
    CommandBufferResetFlags,
    CommandBufferResetFlagBits,
    COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001
);

// vkBeginCommandBuffer
type BeginCommandBuffer =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, info: *const CommandBufferBeginInfo) -> Result;

// vkCmdExecuteCommands
type CmdExecuteCommands =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, count: u32, cmd_bufs: *const CommandBuffer);

// vkEndCommandBuffer
type EndCommandBuffer = unsafe extern "C" fn(cmd_buf: CommandBuffer) -> Result;

// vkResetCommandBuffer
type ResetCommandBuffer =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, flags: CommandBufferResetFlags) -> Result;

// vkDeviceWaitIdle
type DeviceWaitIdle = unsafe extern "C" fn(device: Device) -> Result;

// vkQueueWaitIdle
type QueueWaitIdle = unsafe extern "C" fn(queue: Queue) -> Result;

def_flags!(
    PipelineStageFlags,
    PipelineStageFlagBits,
    PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
    PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
    PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
    PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
    PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
    PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
    PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
    PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
    PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
    PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
    PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
    PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
    PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
    PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
    PIPELINE_STAGE_HOST_BIT = 0x00004000,
    PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
    PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,
    PIPELINE_STAGE_NONE = 0,
    PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000,
    PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000,
    PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 0x02000000,
    PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR = 0x00200000,
    PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000,
    PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00400000,
    PIPELINE_STAGE_COMMAND_PREPROCESS_BIT_NV = 0x00020000,
    PIPELINE_STAGE_TASK_SHADER_BIT_EXT = 0x00080000,
    PIPELINE_STAGE_MESH_SHADER_BIT_EXT = 0x00100000,
    PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV =
        PIPELINE_STAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR,
    PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_NV = PIPELINE_STAGE_RAY_TRACING_SHADER_BIT_KHR,
    PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_NV =
        PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR,
    PIPELINE_STAGE_TASK_SHADER_BIT_NV = PIPELINE_STAGE_TASK_SHADER_BIT_EXT,
    PIPELINE_STAGE_MESH_SHADER_BIT_NV = PIPELINE_STAGE_MESH_SHADER_BIT_EXT,
    PIPELINE_STAGE_NONE_KHR = PIPELINE_STAGE_NONE
);

def_flags!(
    AccessFlags,
    AccessFlagBits,
    ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
    ACCESS_INDEX_READ_BIT = 0x00000002,
    ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
    ACCESS_UNIFORM_READ_BIT = 0x00000008,
    ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
    ACCESS_SHADER_READ_BIT = 0x00000020,
    ACCESS_SHADER_WRITE_BIT = 0x00000040,
    ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
    ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
    ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
    ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
    ACCESS_TRANSFER_READ_BIT = 0x00000800,
    ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
    ACCESS_HOST_READ_BIT = 0x00002000,
    ACCESS_HOST_WRITE_BIT = 0x00004000,
    ACCESS_MEMORY_READ_BIT = 0x00008000,
    ACCESS_MEMORY_WRITE_BIT = 0x00010000,
    ACCESS_NONE = 0,
    ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
    ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
    ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
    ACCESS_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
    ACCESS_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
    ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000,
    ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000,
    ACCESS_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
    ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 0x00800000,
    ACCESS_COMMAND_PREPROCESS_READ_BIT_NV = 0x00020000,
    ACCESS_COMMAND_PREPROCESS_WRITE_BIT_NV = 0x00040000,
    ACCESS_SHADING_RATE_IMAGE_READ_BIT_NV = ACCESS_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR,
    ACCESS_ACCELERATION_STRUCTURE_READ_BIT_NV = ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR,
    ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_NV = ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR,
    ACCESS_NONE_KHR = ACCESS_NONE
);

def_flags!(
    QueryControlFlags,
    QueryControlFlagBits,
    QUERY_CONTROL_PRECISE_BIT = 0x00000001
);

def_flags!(
    QueryResultFlags,
    QueryResultFlagBits,
    QUERY_RESULT_64_BIT = 0x00000001,
    QUERY_RESULT_WAIT_BIT = 0x00000002,
    QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
    QUERY_RESULT_PARTIAL_BIT = 0x00000008
);

def_ids!(
    SharingMode,
    SHARING_MODE_EXCLUSIVE = 0,
    SHARING_MODE_CONCURRENT = 1
);

def_flags!(
    QueryPipelineStatisticFlags,
    QueryPipelineStatisticFlagBits,
    QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
    QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
    QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
    QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
    QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
    QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,
    QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,
    QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
    QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
    QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
    QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
    QUERY_PIPELINE_STATISTIC_TASK_SHADER_INVOCATIONS_BIT_EXT = 0x00000800,
    QUERY_PIPELINE_STATISTIC_MESH_SHADER_INVOCATIONS_BIT_EXT = 0x00001000
);

def_flags!(
    SampleCountFlags,
    SampleCountFlagBits,
    SAMPLE_COUNT_1_BIT = 0x00000001,
    SAMPLE_COUNT_2_BIT = 0x00000002,
    SAMPLE_COUNT_4_BIT = 0x00000004,
    SAMPLE_COUNT_8_BIT = 0x00000008,
    SAMPLE_COUNT_16_BIT = 0x00000010,
    SAMPLE_COUNT_32_BIT = 0x00000020,
    SAMPLE_COUNT_64_BIT = 0x00000040
);

def_ndh!(CommandPoolT, CommandPool);
def_ndh!(FenceT, Fence);
def_ndh!(SemaphoreT, Semaphore);
def_ndh!(DeviceMemoryT, DeviceMemory);
def_ndh!(BufferT, Buffer);
def_ndh!(BufferViewT, BufferView);
def_ndh!(ImageT, Image);
def_ndh!(ImageViewT, ImageView);
def_ndh!(SamplerT, Sampler);
def_ndh!(RenderPassT, RenderPass);
def_ndh!(FramebufferT, Framebuffer);
def_ndh!(DescriptorSetLayoutT, DescriptorSetLayout);
def_ndh!(DescriptorPoolT, DescriptorPool);
def_ndh!(DescriptorSetT, DescriptorSet);
def_ndh!(ShaderModuleT, ShaderModule);
def_ndh!(PipelineLayoutT, PipelineLayout);
def_ndh!(PipelineCacheT, PipelineCache);
def_ndh!(PipelineT, Pipeline);
