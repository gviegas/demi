use std::ffi::{c_char, c_void};
use std::fmt;

use crate::{
    c_size_t, AllocationCallbacks, Bool32, Extent3d, Format, Offset3d, PhysicalDeviceFeatures,
    PhysicalDeviceLimits, PhysicalDeviceSparseProperties, Rect2d, Result, StructureType,
};

// TODO: Sparse API, events, ...

/// VkLayerProperties
#[derive(Debug)]
#[repr(C)]
pub struct LayerProperties {
    pub layer_name: [c_char; 256],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; 256],
}

/// VkExtensionProperties
#[derive(Debug)]
#[repr(C)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; 256],
    pub spec_version: u32,
}

/// PFN_vkEnumerateInstanceLayerProperties
pub(crate) type EnumerateInstanceLayerProperties =
    unsafe extern "C" fn(count: *mut u32, properties: *mut LayerProperties) -> Result;

/// PFN_vkEnumerateInstanceExtensionProperties
pub(crate) type EnumerateInstanceExtensionProperties = unsafe extern "C" fn(
    layer_name: *const c_char,
    count: *mut u32,
    properties: *mut ExtensionProperties,
) -> Result;

/// PFN_vkEnumerateInstanceVersion (v1.1)
pub(crate) type EnumerateInstanceVersion = unsafe extern "C" fn(api_version: *mut u32) -> Result;

def_dh!(InstanceT, Instance);

/// VkInstanceCreateInfo
#[derive(Debug)]
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
#[derive(Debug)]
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

/// PFN_vkCreateInstance
pub(crate) type CreateInstance = unsafe extern "C" fn(
    info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut Instance,
) -> Result;

/// PFN_vkDestroyInstance
pub(crate) type DestroyInstance =
    unsafe extern "C" fn(instance: Instance, allocator: *const AllocationCallbacks);

def_dh!(PhysicalDeviceT, PhysicalDevice);

/// VkPhysicalDeviceProperties
#[derive(Debug)]
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

/// VkPhysicalDeviceProperties2 (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceProperties2 {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub properties: PhysicalDeviceProperties,
}

/// VkPhysicalDeviceGroupProperties (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceGroupProperties {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub physical_device_count: u32,
    pub physical_devices: [PhysicalDevice; 32],
    pub subset_allocation: Bool32,
}

def_ids!(
    PhysicalDeviceType,
    PHYSICAL_DEVICE_TYPE_OTHER = 0,
    PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
    PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
    PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
    PHYSICAL_DEVICE_TYPE_CPU = 4
);

/// VkQueueFamilyProperties
#[derive(Debug)]
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
#[derive(Debug)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; 32],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; 16],
}

/// VkMemoryHeap
#[derive(Debug)]
#[repr(C)]
pub struct MemoryHeap {
    pub size: u64,
    pub flags: MemoryHeapFlags,
}

/// VkMemoryType
#[derive(Debug)]
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

/// PFN_vkEnumeratePhysicalDevices
pub(crate) type EnumeratePhysicalDevices = unsafe extern "C" fn(
    instance: Instance,
    count: *mut u32,
    phys_devs: *mut PhysicalDevice,
) -> Result;

/// PFN_vkEnumeratePhysicalDeviceGroups (v1.1)
pub(crate) type EnumeratePhysicalDeviceGroups = unsafe extern "C" fn(
    instance: Instance,
    count: *mut u32,
    grp_props: *mut PhysicalDeviceGroupProperties,
) -> Result;

/// PFN_vkGetPhysicalDeviceProperties
pub(crate) type GetPhysicalDeviceProperties =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, properties: *mut PhysicalDeviceProperties);

/// PFN_vkGetPhysicalDeviceProperties2 (v1.1)
pub(crate) type GetPhysicalDeviceProperties2 =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, properties: *mut PhysicalDeviceProperties2);

/// PFN_vkGetPhysicalDeviceQueueFamilyProperties
pub(crate) type GetPhysicalDeviceQueueFamilyProperties = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    count: *mut u32,
    queue_props: *mut QueueFamilyProperties,
);

/// PFN_vkGetPhysicalDeviceMemoryProperties
pub(crate) type GetPhysicalDeviceMemoryProperties =
    unsafe extern "C" fn(phys_dev: PhysicalDevice, mem_props: *mut PhysicalDeviceMemoryProperties);

/// PFN_vkEnumerateDeviceExtensionProperties
pub(crate) type EnumerateDeviceExtensionProperties = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    layer_name: *const c_char,
    count: *mut u32,
    properties: *mut ExtensionProperties,
) -> Result;

def_dh!(DeviceT, Device);

/// VkDeviceCreateInfo
#[derive(Debug)]
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
#[derive(Debug)]
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

/// PFN_vkCreateDevice
pub(crate) type CreateDevice = unsafe extern "C" fn(
    phys_dev: PhysicalDevice,
    info: *const DeviceCreateInfo,
    allocator: *const AllocationCallbacks,
    device: *mut Device,
) -> Result;

/// PFN_vkDestroyDevice
pub(crate) type DestroyDevice =
    unsafe extern "C" fn(device: Device, allocator: *const AllocationCallbacks);

def_dh!(QueueT, Queue);

/// VkSubmitInfo
#[derive(Debug)]
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

/// VkSubmitInfo2 (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct SubmitInfo2 {
    pub s_stype: StructureType,
    pub next: *const c_void,
    pub flags: SubmitFlags,
    pub wait_semaphore_info_count: u32,
    pub wait_semaphore_infos: *const SemaphoreSubmitInfo,
    pub command_buffer_info_count: u32,
    pub command_buffer_infos: *const CommandBufferSubmitInfo,
    pub signal_semaphore_info_count: u32,
    pub signal_semaphore_infos: *const SemaphoreSubmitInfo,
}

def_flags!(
    SubmitFlags,
    SubmitFlagBits,
    SUBMIT_PROTECTED_BIT = 0x00000001,
    SUBMIT_PROTECTED_BIT_KHR = SUBMIT_PROTECTED_BIT
);

/// VkSemaphoreSubmitInfo (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct SemaphoreSubmitInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
    pub stage_mask: PipelineStageFlags2,
    pub device_index: u32,
}

/// VkCommandBufferSubmitInfo (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct CommandBufferSubmitInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub command_buffer: CommandBuffer,
    pub device_mask: u32,
}

/// PFN_vkGetDeviceQueue
pub(crate) type GetDeviceQueue =
    unsafe extern "C" fn(device: Device, fam_idx: u32, queue_idx: u32, queue: *mut Queue);

/// PFN_vkQueueSubmit
pub(crate) type QueueSubmit = unsafe extern "C" fn(
    queue: Queue,
    submit_count: u32,
    submits: *const SubmitInfo,
    fence: Fence,
) -> Result;

/// PFN_vkQueueSubmit2
pub(crate) type QueueSubmit2 = unsafe extern "C" fn(
    queue: Queue,
    submit_count: u32,
    submits: *const SubmitInfo2,
    fence: Fence,
) -> Result;

def_dh!(CommandBufferT, CommandBuffer);

/// VkCommandBufferBeginInfo
#[derive(Debug)]
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
#[derive(Debug)]
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

/// PFN_vkBeginCommandBuffer
pub(crate) type BeginCommandBuffer =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, info: *const CommandBufferBeginInfo) -> Result;

/// PFN_vkCmdExecuteCommands
pub(crate) type CmdExecuteCommands =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, count: u32, cmd_bufs: *const CommandBuffer);

/// PFN_vkEndCommandBuffer
pub(crate) type EndCommandBuffer = unsafe extern "C" fn(cmd_buf: CommandBuffer) -> Result;

/// PFN_vkResetCommandBuffer
pub(crate) type ResetCommandBuffer =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, flags: CommandBufferResetFlags) -> Result;

def_ndh!(CommandPoolT, CommandPool);

/// VkCommandPoolCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

def_flags!(
    CommandPoolCreateFlags,
    CommandPoolCreateFlagBits,
    COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
    COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
    COMMAND_POOL_CREATE_PROTECTED_BIT = 0x00000004
);

def_flags!(CommandPoolTrimFlags, CommandPoolTrimFlagBits,);

def_flags!(
    CommandPoolResetFlags,
    CommandPoolResetFlagBits,
    COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001
);

/// VkCommandBufferAllocateInfo
#[derive(Debug)]
#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub command_pool: CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

def_ids!(
    CommandBufferLevel,
    COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    COMMAND_BUFFER_LEVEL_SECONDARY = 1
);

/// PFN_vkCreateCommandPool
pub(crate) type CreateCommandPool = unsafe extern "C" fn(
    device: Device,
    info: *const CommandPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    cmd_pool: *mut CommandPool,
) -> Result;

/// PFN_vkTrimCommandPool (v1.1)
pub(crate) type TrimCommandPool =
    unsafe extern "C" fn(device: Device, cmd_pool: CommandPool, flags: CommandPoolTrimFlags);

/// PFN_vkResetCommandPool
pub(crate) type ResetCommandPool = unsafe extern "C" fn(
    device: Device,
    cmd_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;

/// PFN_vkDestroyCommandPool
pub(crate) type DestroyCommandPool = unsafe extern "C" fn(
    device: Device,
    cmd_pool: CommandPool,
    allocator: *const AllocationCallbacks,
);

/// PFN_vkAllocateCommandBuffers
pub(crate) type AllocateCommandBuffers = unsafe extern "C" fn(
    device: Device,
    info: *const CommandBufferAllocateInfo,
    cmd_bufs: *mut CommandBuffer,
) -> Result;

/// PFN_vkFreeCommandBuffers
pub(crate) type FreeCommandBuffers = unsafe extern "C" fn(
    device: Device,
    cmd_pool: CommandPool,
    count: u32,
    cmd_bufs: *const CommandBuffer,
);

def_ndh!(FenceT, Fence);

/// VkFenceCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct FenceCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: FenceCreateFlags,
}

def_flags!(
    FenceCreateFlags,
    FenceCreateFlagBits,
    FENCE_CREATE_SIGNALED_BIT = 0x00000001
);

/// PFN_vkCreateFence
pub(crate) type CreateFence = unsafe extern "C" fn(
    device: Device,
    info: *const FenceCreateInfo,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> Result;

/// PFN_vkGetfenceStatus
pub(crate) type GetFenceStatus = unsafe extern "C" fn(device: Device, fence: Fence) -> Result;

/// PFN_vkResetFences
pub(crate) type ResetFences =
    unsafe extern "C" fn(device: Device, count: u32, fences: *const Fence) -> Result;

/// PFN_vkWaitForFences
pub(crate) type WaitForFences = unsafe extern "C" fn(
    device: Device,
    count: u32,
    fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;

/// PFN_vkDestroyFence
pub(crate) type DestroyFence =
    unsafe extern "C" fn(device: Device, fence: Fence, allocator: *const AllocationCallbacks);

def_ndh!(SemaphoreT, Semaphore);

/// VkSemaphoreCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

def_flags!(SemaphoreCreateFlags, SemaphoreCreateFlagBits,);

/// VkSemaphoreWaitInfo (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct SemaphoreWaitInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: SemaphoreWaitFlags,
    pub semaphore_count: u32,
    pub semaphore: *const Semaphore,
    pub values: *const u64,
}

def_flags!(
    SemaphoreWaitFlags,
    SemaphoreWaitFlagBits,
    SEMAPHORE_WAIT_ANY_BIT = 0x00000001,
    SEMAPHORE_WAIT_ANY_BIT_KHR = SEMAPHORE_WAIT_ANY_BIT
);

/// VkSemaphoreSignalInfo (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct SemaphoreSignalInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub semaphore: Semaphore,
    pub value: u64,
}

/// PFN_vkCreateSemaphore
pub(crate) type CreateSemaphore = unsafe extern "C" fn(
    device: Device,
    info: *const SemaphoreCreateInfo,
    allocator: *const AllocationCallbacks,
    semaphore: *mut Semaphore,
) -> Result;

/// PFN_vkGetSemaphoreCounterValue (v1.2)
pub(crate) type GetSemaphoreCounterValue =
    unsafe extern "C" fn(device: Device, semaphore: Semaphore, value: *mut u64) -> Result;

/// PFN_vkWaitSemaphores (v1.2)
pub(crate) type WaitSemaphores = unsafe extern "C" fn(
    device: Device,
    wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> Result;

/// PFN_vkSignalSemaphore (v1.2)
pub(crate) type SignalSemaphore =
    unsafe extern "C" fn(device: Device, signal_info: *const SemaphoreSignalInfo) -> Result;

/// PFN_vkDestroySemaphore
pub(crate) type DestroySemaphore = unsafe extern "C" fn(
    device: Device,
    semaphore: Semaphore,
    allocator: *const AllocationCallbacks,
);

/// VkMemoryBarrier
#[derive(Debug)]
#[repr(C)]
pub struct MemoryBarrier {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

/// VkImageMemoryBarrier
#[derive(Debug)]
#[repr(C)]
pub struct ImageMemoryBarrier {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

/// VkBufferMemoryBarrier
#[derive(Debug)]
#[repr(C)]
pub struct BufferMemoryBarrier {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: u64,
    pub size: u64,
}

/// PFN_vkCmdPipelineBarrier
pub(crate) type CmdPipelineBarrier = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    depend_flags: DependencyFlags,
    mem_barrier_count: u32,
    mem_barriers: *const MemoryBarrier,
    buf_barrier_count: u32,
    buf_barriers: *const BufferMemoryBarrier,
    img_barrier_count: u32,
    img_barriers: *const ImageMemoryBarrier,
);

/// VkDependencyInfo (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct DependencyInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub dependency_flags: DependencyFlags,
    pub memory_barrier_count: u32,
    pub memory_barriers: *const MemoryBarrier2,
    pub buffer_memory_barrier_count: u32,
    pub buf_barriers: *const BufferMemoryBarrier2,
    pub image_memory_barrier_count: u32,
    pub image_memory_barriers: *const ImageMemoryBarrier2,
}

/// VkMemoryBarrier2 (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct MemoryBarrier2 {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
}

/// VkImageMemoryBarrier2 (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct ImageMemoryBarrier2 {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: Image,
    pub subresource_range: ImageSubresourceRange,
}

/// VkBufferMemoryBarrier2 (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct BufferMemoryBarrier2 {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_stage_mask: PipelineStageFlags2,
    pub src_access_mask: AccessFlags2,
    pub dst_stage_mask: PipelineStageFlags2,
    pub dst_access_mask: AccessFlags2,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub buffer: Buffer,
    pub offset: u64,
    pub size: u64,
}

/// PFN_vkCmdPipelineBarrier2 (v1.3)
pub(crate) type CmdPipelineBarrier2 =
    unsafe extern "C" fn(command_buffer: CommandBuffer, dependency_info: *const DependencyInfo);

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

def_flags64!(
    PipelineStageFlags2,
    PipelineStageFlagBits2,
    PIPELINE_STAGE_2_NONE = 0,
    PIPELINE_STAGE_2_NONE_KHR = 0,
    PIPELINE_STAGE_2_TOP_OF_PIPE_BIT = 0x00000001,
    PIPELINE_STAGE_2_TOP_OF_PIPE_BIT_KHR = 0x00000001,
    PIPELINE_STAGE_2_DRAW_INDIRECT_BIT = 0x00000002,
    PIPELINE_STAGE_2_DRAW_INDIRECT_BIT_KHR = 0x00000002,
    PIPELINE_STAGE_2_VERTEX_INPUT_BIT = 0x00000004,
    PIPELINE_STAGE_2_VERTEX_INPUT_BIT_KHR = 0x00000004,
    PIPELINE_STAGE_2_VERTEX_SHADER_BIT = 0x00000008,
    PIPELINE_STAGE_2_VERTEX_SHADER_BIT_KHR = 0x00000008,
    PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
    PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT_KHR = 0x00000010,
    PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
    PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT_KHR = 0x00000020,
    PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT = 0x00000040,
    PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT_KHR = 0x00000040,
    PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT = 0x00000080,
    PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT_KHR = 0x00000080,
    PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
    PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT_KHR = 0x00000100,
    PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
    PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT_KHR = 0x00000200,
    PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
    PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT_KHR = 0x00000400,
    PIPELINE_STAGE_2_COMPUTE_SHADER_BIT = 0x00000800,
    PIPELINE_STAGE_2_COMPUTE_SHADER_BIT_KHR = 0x00000800,
    PIPELINE_STAGE_2_ALL_TRANSFER_BIT = 0x00001000,
    PIPELINE_STAGE_2_ALL_TRANSFER_BIT_KHR = 0x00001000,
    PIPELINE_STAGE_2_TRANSFER_BIT = 0x00001000,
    PIPELINE_STAGE_2_TRANSFER_BIT_KHR = 0x00001000,
    PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT = 0x00002000,
    PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT_KHR = 0x00002000,
    PIPELINE_STAGE_2_HOST_BIT = 0x00004000,
    PIPELINE_STAGE_2_HOST_BIT_KHR = 0x00004000,
    PIPELINE_STAGE_2_ALL_GRAPHICS_BIT = 0x00008000,
    PIPELINE_STAGE_2_ALL_GRAPHICS_BIT_KHR = 0x00008000,
    PIPELINE_STAGE_2_ALL_COMMANDS_BIT = 0x00010000,
    PIPELINE_STAGE_2_ALL_COMMANDS_BIT_KHR = 0x00010000,
    PIPELINE_STAGE_2_COPY_BIT = 0x100000000,
    PIPELINE_STAGE_2_COPY_BIT_KHR = 0x100000000,
    PIPELINE_STAGE_2_RESOLVE_BIT = 0x200000000,
    PIPELINE_STAGE_2_RESOLVE_BIT_KHR = 0x200000000,
    PIPELINE_STAGE_2_BLIT_BIT = 0x400000000,
    PIPELINE_STAGE_2_BLIT_BIT_KHR = 0x400000000,
    PIPELINE_STAGE_2_CLEAR_BIT = 0x800000000,
    PIPELINE_STAGE_2_CLEAR_BIT_KHR = 0x800000000,
    PIPELINE_STAGE_2_INDEX_INPUT_BIT = 0x1000000000,
    PIPELINE_STAGE_2_INDEX_INPUT_BIT_KHR = 0x1000000000,
    PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT = 0x2000000000,
    PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT_KHR = 0x2000000000,
    PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT = 0x4000000000,
    PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT_KHR = 0x4000000000,
    PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT = 0x01000000,
    PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT = 0x00040000,
    PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV = 0x00020000,
    PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00400000,
    PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV = 0x00400000,
    PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR = 0x02000000,
    PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR = 0x00200000,
    PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV = 0x00200000,
    PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV = 0x02000000,
    PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT = 0x00800000,
    PIPELINE_STAGE_2_TASK_SHADER_BIT_NV = 0x00080000,
    PIPELINE_STAGE_2_MESH_SHADER_BIT_NV = 0x00100000,
    PIPELINE_STAGE_2_TASK_SHADER_BIT_EXT = 0x00080000,
    PIPELINE_STAGE_2_MESH_SHADER_BIT_EXT = 0x00100000,
    PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI = 0x8000000000,
    PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI = 0x10000000000,
    PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_COPY_BIT_KHR = 0x10000000,
    PIPELINE_STAGE_2_MICROMAP_BUILD_BIT_EXT = 0x40000000,
    PIPELINE_STAGE_2_OPTICAL_FLOW_BIT_NV = 0x20000000
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

def_flags64!(
    AccessFlags2,
    AccessFlagBits2,
    ACCESS_2_NONE = 0,
    ACCESS_2_NONE_KHR = 0,
    ACCESS_2_INDIRECT_COMMAND_READ_BIT = 0x00000001,
    ACCESS_2_INDIRECT_COMMAND_READ_BIT_KHR = 0x00000001,
    ACCESS_2_INDEX_READ_BIT = 0x00000002,
    ACCESS_2_INDEX_READ_BIT_KHR = 0x00000002,
    ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
    ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT_KHR = 0x00000004,
    ACCESS_2_UNIFORM_READ_BIT = 0x00000008,
    ACCESS_2_UNIFORM_READ_BIT_KHR = 0x00000008,
    ACCESS_2_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
    ACCESS_2_INPUT_ATTACHMENT_READ_BIT_KHR = 0x00000010,
    ACCESS_2_SHADER_READ_BIT = 0x00000020,
    ACCESS_2_SHADER_READ_BIT_KHR = 0x00000020,
    ACCESS_2_SHADER_WRITE_BIT = 0x00000040,
    ACCESS_2_SHADER_WRITE_BIT_KHR = 0x00000040,
    ACCESS_2_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
    ACCESS_2_COLOR_ATTACHMENT_READ_BIT_KHR = 0x00000080,
    ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
    ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT_KHR = 0x00000100,
    ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
    ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT_KHR = 0x00000200,
    ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
    ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT_KHR = 0x00000400,
    ACCESS_2_TRANSFER_READ_BIT = 0x00000800,
    ACCESS_2_TRANSFER_READ_BIT_KHR = 0x00000800,
    ACCESS_2_TRANSFER_WRITE_BIT = 0x00001000,
    ACCESS_2_TRANSFER_WRITE_BIT_KHR = 0x00001000,
    ACCESS_2_HOST_READ_BIT = 0x00002000,
    ACCESS_2_HOST_READ_BIT_KHR = 0x00002000,
    ACCESS_2_HOST_WRITE_BIT = 0x00004000,
    ACCESS_2_HOST_WRITE_BIT_KHR = 0x00004000,
    ACCESS_2_MEMORY_READ_BIT = 0x00008000,
    ACCESS_2_MEMORY_READ_BIT_KHR = 0x00008000,
    ACCESS_2_MEMORY_WRITE_BIT = 0x00010000,
    ACCESS_2_MEMORY_WRITE_BIT_KHR = 0x00010000,
    ACCESS_2_SHADER_SAMPLED_READ_BIT = 0x100000000,
    ACCESS_2_SHADER_SAMPLED_READ_BIT_KHR = 0x100000000,
    ACCESS_2_SHADER_STORAGE_READ_BIT = 0x200000000,
    ACCESS_2_SHADER_STORAGE_READ_BIT_KHR = 0x200000000,
    ACCESS_2_SHADER_STORAGE_WRITE_BIT = 0x400000000,
    ACCESS_2_SHADER_STORAGE_WRITE_BIT_KHR = 0x400000000,
    ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT = 0x02000000,
    ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT = 0x04000000,
    ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT = 0x08000000,
    ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT = 0x00100000,
    ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV = 0x00020000,
    ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV = 0x00040000,
    ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR = 0x00800000,
    ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV = 0x00800000,
    ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR = 0x00200000,
    ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR = 0x00400000,
    ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV = 0x00200000,
    ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV = 0x00400000,
    ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT = 0x01000000,
    ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT = 0x00080000,
    ACCESS_2_INVOCATION_MASK_READ_BIT_HUAWEI = 0x8000000000,
    ACCESS_2_SHADER_BINDING_TABLE_READ_BIT_KHR = 0x10000000000,
    ACCESS_2_MICROMAP_READ_BIT_EXT = 0x100000000000,
    ACCESS_2_MICROMAP_WRITE_BIT_EXT = 0x200000000000,
    ACCESS_2_OPTICAL_FLOW_READ_BIT_NV = 0x40000000000,
    ACCESS_2_OPTICAL_FLOW_WRITE_BIT_NV = 0x80000000000
);

def_flags!(
    DependencyFlags,
    DependencyFlagBits,
    DEPENDENCY_BY_REGION_BIT = 0x00000001,
    DEPENDENCY_DEVICE_GROUP_BIT = 0x00000004,
    DEPENDENCY_VIEW_LOCAL_BIT = 0x00000002,
    DEPENDENCY_FEEDBACK_LOOP_BIT_EXT = 0x00000008,
    DEPENDENCY_VIEW_LOCAL_BIT_KHR = DEPENDENCY_VIEW_LOCAL_BIT,
    DEPENDENCY_DEVICE_GROUP_BIT_KHR = DEPENDENCY_DEVICE_GROUP_BIT
);

def_ids!(
    SharingMode,
    SHARING_MODE_EXCLUSIVE = 0,
    SHARING_MODE_CONCURRENT = 1
);

/// PFN_vkDeviceWaitIdle
pub(crate) type DeviceWaitIdle = unsafe extern "C" fn(device: Device) -> Result;

/// PFN_vkQueueWaitIdle
pub(crate) type QueueWaitIdle = unsafe extern "C" fn(queue: Queue) -> Result;

def_ndh!(DeviceMemoryT, DeviceMemory);

/// VkMappedMemoryRange
#[derive(Debug)]
#[repr(C)]
pub struct MappedMemoryRange {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: u64,
    pub size: u64,
}

/// VkMemoryAllocateInfo
#[derive(Debug)]
#[repr(C)]
pub struct MemoryAllocateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub allocation_size: u64,
    pub memory_type_index: u32,
}

/// VkMemoryRequirements
#[derive(Debug)]
#[repr(C)]
pub struct MemoryRequirements {
    pub size: u64,
    pub alignment: u64,
    pub memory_type_bits: u32,
}

def_flags!(MemoryMapFlags, MemoryMapFlagBits,);

/// PFN_vkAllocateMemory
pub(crate) type AllocateMemory = unsafe extern "C" fn(
    device: Device,
    info: *const MemoryAllocateInfo,
    allocator: *const AllocationCallbacks,
    memory: *mut DeviceMemory,
) -> Result;

/// PFN_vkMapMemory
pub(crate) type MapMemory = unsafe extern "C" fn(
    device: Device,
    memory: DeviceMemory,
    offset: u64,
    size: u64,
    flags: MemoryMapFlags,
    data: *mut *mut c_void,
) -> Result;

/// PFN_vkFlushMappedMemoryRanges
pub(crate) type FlushMappedMemoryRanges = unsafe extern "C" fn(
    device: Device,
    mem_range_count: u32,
    mem_ranges: *const MappedMemoryRange,
) -> Result;

/// PFN_vkInvalidateMappedMemoryRanges
pub(crate) type InvalidateMappedMemoryRanges = unsafe extern "C" fn(
    device: Device,
    mem_range_count: u32,
    mem_ranges: *const MappedMemoryRange,
) -> Result;

/// PFN_vkUnmapMemory
pub(crate) type UnmapMemory = unsafe extern "C" fn(device: Device, memory: DeviceMemory);

/// PFN_vkFreeMemory
pub(crate) type FreeMemory = unsafe extern "C" fn(
    device: Device,
    memory: DeviceMemory,
    allocator: *const AllocationCallbacks,
);

def_ndh!(BufferT, Buffer);

/// VkBufferCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct BufferCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: u64,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
}

def_flags!(
    BufferCreateFlags,
    BufferCreateFlagBits,
    BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,
    BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
    BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
    BUFFER_CREATE_PROTECTED_BIT = 0x00000008,
    BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT = 0x00000010,
    BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT =
        BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT,
    BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR =
        BUFFER_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT
);

def_flags!(
    BufferUsageFlags,
    BufferUsageFlagBits,
    BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
    BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
    BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
    BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
    BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
    BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
    BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
    BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
    BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT = 0x00020000,
    BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT = 0x00000800,
    BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT = 0x00001000,
    BUFFER_USAGE_CONDITIONAL_RENDERING_BIT_EXT = 0x00000200,
    BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR = 0x00080000,
    BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR = 0x00100000,
    BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR = 0x00000400,
    BUFFER_USAGE_MICROMAP_BUILD_INPUT_READ_ONLY_BIT_EXT = 0x00800000,
    BUFFER_USAGE_MICROMAP_STORAGE_BIT_EXT = 0x01000000,
    BUFFER_USAGE_RAY_TRACING_BIT_NV = BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR,
    BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_EXT = BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT,
    BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT_KHR = BUFFER_USAGE_SHADER_DEVICE_ADDRESS_BIT
);

/// PFN_vkCreateBuffer
pub(crate) type CreateBuffer = unsafe extern "C" fn(
    device: Device,
    info: *const BufferCreateInfo,
    allocator: *const AllocationCallbacks,
    buffer: *mut Buffer,
) -> Result;

/// PFN_vkGetBufferMemoryRequirements
pub(crate) type GetBufferMemoryRequirements =
    unsafe extern "C" fn(device: Device, buffer: Buffer, mem_reqs: *mut MemoryRequirements);

/// PFN_vkBindBufferMemory
pub(crate) type BindBufferMemory = unsafe extern "C" fn(
    device: Device,
    buffer: Buffer,
    memory: DeviceMemory,
    mem_off: u64,
) -> Result;

/// PFN_vkDestroyBuffer
pub(crate) type DestroyBuffer =
    unsafe extern "C" fn(device: Device, buffer: Buffer, allocator: *const AllocationCallbacks);

/// VkBufferCopy
#[derive(Debug)]
#[repr(C)]
pub struct BufferCopy {
    pub src_offset: u64,
    pub dst_offset: u64,
    pub size: u64,
}

/// PFN_vkCmdCopyBuffer
pub(crate) type CmdCopyBuffer = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    src_buf: Buffer,
    dst_buf: Buffer,
    region_count: u32,
    regions: *const BufferCopy,
);

/// PFN_vkCmdFillBuffer
pub(crate) type CmdFillBuffer = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    buffer: Buffer,
    offset: u64,
    size: u64,
    value: u32,
);

/// PFN_vkCmdUpdateBuffer
pub(crate) type CmdUpdateBuffer = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    buffer: Buffer,
    offset: u64,
    size: u64,
    data: *const c_void,
);

def_ndh!(BufferViewT, BufferView);

/// VkBufferViewCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct BufferViewCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: Buffer,
    pub format: Format,
    pub offset: u64,
    pub range: u64,
}

def_flags!(BufferViewCreateFlags, BufferViewCreateFlagBits,);

/// PFN_vkCreateBufferView
pub(crate) type CreateBufferView = unsafe extern "C" fn(
    device: Device,
    info: *const BufferViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut BufferView,
) -> Result;

/// PFN_vkDestroyBufferView
pub(crate) type DestroyBufferView =
    unsafe extern "C" fn(device: Device, view: BufferView, allocator: *const AllocationCallbacks);

def_ndh!(ImageT, Image);

/// VkImageCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct ImageCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3d,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlagBits,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}

def_flags!(
    ImageCreateFlags,
    ImageCreateFlagBits,
    IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,
    IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
    IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
    IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,
    IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010,
    IMAGE_CREATE_ALIAS_BIT = 0x00000400,
    IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT = 0x00000040,
    IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT = 0x00000020,
    IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT = 0x00000080,
    IMAGE_CREATE_EXTENDED_USAGE_BIT = 0x00000100,
    IMAGE_CREATE_PROTECTED_BIT = 0x00000800,
    IMAGE_CREATE_DISJOINT_BIT = 0x00000200,
    IMAGE_CREATE_CORNER_SAMPLED_BIT_NV = 0x00002000,
    IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT = 0x00001000,
    IMAGE_CREATE_SUBSAMPLED_BIT_EXT = 0x00004000,
    IMAGE_CREATE_MULTISAMPLED_RENDER_TO_SINGLE_SAMPLED_BIT_EXT = 0x00040000,
    IMAGE_CREATE_2D_VIEW_COMPATIBLE_BIT_EXT = 0x00020000,
    IMAGE_CREATE_FRAGMENT_DENSITY_MAP_OFFSET_BIT_QCOM = 0x00008000,
    IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT_KHR = IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT,
    IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT_KHR = IMAGE_CREATE_2D_ARRAY_COMPATIBLE_BIT,
    IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT_KHR = IMAGE_CREATE_BLOCK_TEXEL_VIEW_COMPATIBLE_BIT,
    IMAGE_CREATE_EXTENDED_USAGE_BIT_KHR = IMAGE_CREATE_EXTENDED_USAGE_BIT,
    IMAGE_CREATE_DISJOINT_BIT_KHR = IMAGE_CREATE_DISJOINT_BIT,
    IMAGE_CREATE_ALIAS_BIT_KHR = IMAGE_CREATE_ALIAS_BIT
);

def_ids!(
    ImageType,
    IMAGE_TYPE_1D = 0,
    IMAGE_TYPE_2D = 1,
    IMAGE_TYPE_3D = 2
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

def_ids!(
    ImageTiling,
    IMAGE_TILING_OPTIMAL = 0,
    IMAGE_TILING_LINEAR = 1,
    IMAGE_TILING_DRM_FORMAT_MODIFIER_EXT = 1000158000
);

def_flags!(
    ImageUsageFlags,
    ImageUsageFlagBits,
    IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
    IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
    IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
    IMAGE_USAGE_STORAGE_BIT = 0x00000008,
    IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
    IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
    IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
    IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,
    IMAGE_USAGE_FRAGMENT_DENSITY_MAP_BIT_EXT = 0x00000200,
    IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00000100,
    IMAGE_USAGE_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 0x00080000,
    IMAGE_USAGE_INVOCATION_MASK_BIT_HUAWEI = 0x00040000,
    IMAGE_USAGE_SAMPLE_WEIGHT_BIT_QCOM = 0x00100000,
    IMAGE_USAGE_SAMPLE_BLOCK_MATCH_BIT_QCOM = 0x00200000,
    IMAGE_USAGE_SHADING_RATE_IMAGE_BIT_NV = IMAGE_USAGE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR
);

def_ids!(
    ImageLayout,
    IMAGE_LAYOUT_UNDEFINED = 0,
    IMAGE_LAYOUT_GENERAL = 1,
    IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
    IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
    IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
    IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
    IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
    IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
    IMAGE_LAYOUT_PREINITIALIZED = 8,
    IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL = 1000117000,
    IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL = 1000117001,
    IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL = 1000241000,
    IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL = 1000241001,
    IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL = 1000241002,
    IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL = 1000241003,
    IMAGE_LAYOUT_READ_ONLY_OPTIMAL = 1000314000,
    IMAGE_LAYOUT_ATTACHMENT_OPTIMAL = 1000314001,
    IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
    IMAGE_LAYOUT_SHARED_PRESENT_KHR = 1000111000,
    IMAGE_LAYOUT_FRAGMENT_DENSITY_MAP_OPTIMAL_EXT = 1000218000,
    IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR = 1000164003,
    IMAGE_LAYOUT_ATTACHMENT_FEEDBACK_LOOP_OPTIMAL_EXT = 1000339000,
    IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL_KHR =
        IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL,
    IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL_KHR =
        IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL,
    IMAGE_LAYOUT_SHADING_RATE_OPTIMAL_NV =
        IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR,
    IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL_KHR = IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL,
    IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR = IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL,
    IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL_KHR = IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL,
    IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL_KHR = IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL,
    IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR = IMAGE_LAYOUT_READ_ONLY_OPTIMAL,
    IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR = IMAGE_LAYOUT_ATTACHMENT_OPTIMAL
);

/// PFN_vkCreateImage
pub(crate) type CreateImage = unsafe extern "C" fn(
    device: Device,
    info: *const ImageCreateInfo,
    allocator: *const AllocationCallbacks,
    image: *mut Image,
) -> Result;

/// PFN_vkGetImageMemoryProperties
pub(crate) type GetImageMemoryRequirements =
    unsafe extern "C" fn(device: Device, image: Image, mem_reqs: *mut MemoryRequirements);

/// PFN_vkBindImageMemory
pub(crate) type BindImageMemory = unsafe extern "C" fn(
    device: Device,
    image: Image,
    memory: DeviceMemory,
    mem_off: u64,
) -> Result;

/// PFN_vkDestroyImage
pub(crate) type DestroyImage =
    unsafe extern "C" fn(device: Device, image: Image, allocator: *const AllocationCallbacks);

/// VkImageCopy
#[derive(Debug)]
#[repr(C)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3d,
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offset: Offset3d,
    pub extent: Extent3d,
}

/// VkImageBlit
#[derive(Debug)]
#[repr(C)]
pub struct ImageBlit {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offsets: [Offset3d; 2],
    pub dst_subresource: ImageSubresourceLayers,
    pub dst_offsets: [Offset3d; 2],
}

/// VkImageSubresourceLayers
#[derive(Debug)]
#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

/// PFN_vkCmdCopyImage
pub(crate) type CmdCopyImage = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    src_img: Image,
    src_layout: ImageLayout,
    dst_img: Image,
    dst_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageCopy,
);

/// PFN_vkCmdBlitImage
pub(crate) type CmdBlitImage = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    src_img: Image,
    src_layout: ImageLayout,
    dst_img: Image,
    dst_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageBlit,
    filter: Filter,
);

/// PFN_vkCmdClearColorImage
pub(crate) type CmdClearColorImage = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    image: Image,
    layout: ImageLayout,
    value: *const ClearColorValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
);

/// PFN_vkCmdClearDepthStencilImage
pub(crate) type CmdClearDepthStencilImage = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    image: Image,
    layout: ImageLayout,
    value: *const ClearDepthStencilValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
);

def_ndh!(ImageViewT, ImageView);

/// VkImageViewCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct ImageViewCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

def_flags!(
    ImageViewCreateFlags,
    ImageViewCreateFlagBits,
    IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DYNAMIC_BIT_EXT = 0x00000001,
    IMAGE_VIEW_CREATE_FRAGMENT_DENSITY_MAP_DEFERRED_BIT_EXT = 0x00000002
);

def_ids!(
    ImageViewType,
    IMAGE_VIEW_TYPE_1D = 0,
    IMAGE_VIEW_TYPE_2D = 1,
    IMAGE_VIEW_TYPE_3D = 2,
    IMAGE_VIEW_TYPE_CUBE = 3,
    IMAGE_VIEW_TYPE_1D_ARRAY = 4,
    IMAGE_VIEW_TYPE_2D_ARRAY = 5,
    IMAGE_VIEW_TYPE_CUBE_ARRAY = 6
);

/// VkComponentMapping
#[derive(Debug)]
#[repr(C)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

def_ids!(
    ComponentSwizzle,
    COMPONENT_SWIZZLE_IDENTITY = 0,
    COMPONENT_SWIZZLE_ZERO = 1,
    COMPONENT_SWIZZLE_ONE = 2,
    COMPONENT_SWIZZLE_R = 3,
    COMPONENT_SWIZZLE_G = 4,
    COMPONENT_SWIZZLE_B = 5,
    COMPONENT_SWIZZLE_A = 6
);

/// VkImageSubresourceRange
#[derive(Debug)]
#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub level_count: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

def_flags!(
    ImageAspectFlags,
    ImageAspectFlagBits,
    IMAGE_ASPECT_COLOR_BIT = 0x00000001,
    IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
    IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
    IMAGE_ASPECT_METADATA_BIT = 0x00000008,
    IMAGE_ASPECT_PLANE_0_BIT = 0x00000010,
    IMAGE_ASPECT_PLANE_1_BIT = 0x00000020,
    IMAGE_ASPECT_PLANE_2_BIT = 0x00000040,
    IMAGE_ASPECT_NONE = 0,
    IMAGE_ASPECT_MEMORY_PLANE_0_BIT_EXT = 0x00000080,
    IMAGE_ASPECT_MEMORY_PLANE_1_BIT_EXT = 0x00000100,
    IMAGE_ASPECT_MEMORY_PLANE_2_BIT_EXT = 0x00000200,
    IMAGE_ASPECT_MEMORY_PLANE_3_BIT_EXT = 0x00000400,
    IMAGE_ASPECT_PLANE_0_BIT_KHR = IMAGE_ASPECT_PLANE_0_BIT,
    IMAGE_ASPECT_PLANE_1_BIT_KHR = IMAGE_ASPECT_PLANE_1_BIT,
    IMAGE_ASPECT_PLANE_2_BIT_KHR = IMAGE_ASPECT_PLANE_2_BIT,
    IMAGE_ASPECT_NONE_KHR = IMAGE_ASPECT_NONE
);

/// PFN_vkCreateImageView
pub(crate) type CreateImageView = unsafe extern "C" fn(
    device: Device,
    info: *const ImageViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut ImageView,
) -> Result;

/// PFN_vkDestroyImageView
pub(crate) type DestroyImageView =
    unsafe extern "C" fn(device: Device, view: ImageView, allocator: *const AllocationCallbacks);

/// VkBufferImageCopy
#[derive(Debug)]
#[repr(C)]
pub struct BufferImageCopy {
    pub buffer_offset: u64,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3d,
    pub image_extent: Extent3d,
}

/// PFN_vkCmdCopyBufferToImage
pub(crate) type CmdCopyBufferToImage = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    src_buf: Buffer,
    dst_img: Image,
    dst_layout: ImageLayout,
    region_count: u32,
    regions: *const BufferImageCopy,
);

/// PFN_vkCmdCopyImageToBuffer
pub(crate) type CmdCopyImageToBuffer = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    src_img: Image,
    src_layout: ImageLayout,
    dst_buf: Buffer,
    region_count: u32,
    regions: *const BufferImageCopy,
);

def_ndh!(SamplerT, Sampler);

/// VkSamplerCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct SamplerCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}

def_flags!(
    SamplerCreateFlags,
    SamplerCreateFlagBits,
    SAMPLER_CREATE_SUBSAMPLED_BIT_EXT = 0x00000001,
    SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT = 0x00000002,
    SAMPLER_CREATE_NON_SEAMLESS_CUBE_MAP_BIT_EXT = 0x00000004,
    SAMPLER_CREATE_IMAGE_PROCESSING_BIT_QCOM = 0x00000010
);

def_ids!(
    Filter,
    FILTER_NEAREST = 0,
    FILTER_LINEAR = 1,
    FILTER_CUBIC_EXT = 1000015000,
    FILTER_CUBIC_IMG = FILTER_CUBIC_EXT
);

def_ids!(
    SamplerMipmapMode,
    SAMPLER_MIPMAP_MODE_NEAREST = 0,
    SAMPLER_MIPMAP_MODE_LINEAR = 1
);

def_ids!(
    SamplerAddressMode,
    SAMPLER_ADDRESS_MODE_REPEAT = 0,
    SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
    SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
    SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
    SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
    SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR = SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE
);

def_ids!(
    CompareOp,
    COMPARE_OP_NEVER = 0,
    COMPARE_OP_LESS = 1,
    COMPARE_OP_EQUAL = 2,
    COMPARE_OP_LESS_OR_EQUAL = 3,
    COMPARE_OP_GREATER = 4,
    COMPARE_OP_NOT_EQUAL = 5,
    COMPARE_OP_GREATER_OR_EQUAL = 6,
    COMPARE_OP_ALWAYS = 7
);

def_ids!(
    BorderColor,
    BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
    BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
    BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
    BORDER_COLOR_INT_OPAQUE_BLACK = 3,
    BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
    BORDER_COLOR_INT_OPAQUE_WHITE = 5,
    BORDER_COLOR_FLOAT_CUSTOM_EXT = 1000287003,
    BORDER_COLOR_INT_CUSTOM_EXT = 1000287004
);

/// PFN_vkCreateSampler
pub(crate) type CreateSampler = unsafe extern "C" fn(
    device: Device,
    info: *const SamplerCreateInfo,
    allocator: *const AllocationCallbacks,
    sampler: *mut Sampler,
) -> Result;

/// PFN_vkDestroySampler
pub(crate) type DestroySampler =
    unsafe extern "C" fn(device: Device, sampler: Sampler, allocator: *const AllocationCallbacks);

def_ndh!(RenderPassT, RenderPass);

/// VkRenderPassCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct RenderPassCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub dependencies: *const SubpassDependency,
}

def_flags!(
    RenderPassCreateFlags,
    RenderPassCreateFlagBits,
    RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM = 0x00000002
);

/// VkAttachmentDescription
#[derive(Debug)]
#[repr(C)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

def_flags!(
    AttachmentDescriptionFlags,
    AttachmentDescriptionFlagBits,
    ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001
);

def_ids!(
    AttachmentLoadOp,
    ATTACHMENT_LOAD_OP_LOAD = 0,
    ATTACHMENT_LOAD_OP_CLEAR = 1,
    ATTACHMENT_LOAD_OP_DONT_CARE = 2,
    ATTACHMENT_LOAD_OP_NONE_EXT = 1000400000
);

def_ids!(
    AttachmentStoreOp,
    ATTACHMENT_STORE_OP_STORE = 0,
    ATTACHMENT_STORE_OP_DONT_CARE = 1,
    ATTACHMENT_STORE_OP_NONE = 1000301000,
    ATTACHMENT_STORE_OP_NONE_KHR = ATTACHMENT_STORE_OP_NONE,
    ATTACHMENT_STORE_OP_NONE_QCOM = ATTACHMENT_STORE_OP_NONE,
    ATTACHMENT_STORE_OP_NONE_EXT = ATTACHMENT_STORE_OP_NONE
);

/// VkSubpassDescription
#[derive(Debug)]
#[repr(C)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub color_attachments: *const AttachmentReference,
    pub resolve_attachments: *const AttachmentReference,
    pub depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub preserve_attachments: *const u32,
}

def_flags!(
    SubpassDescriptionFlags,
    SubpassDescriptionFlagBits,
    SUBPASS_DESCRIPTION_PER_VIEW_ATTRIBUTES_BIT_NVX = 0x00000001,
    SUBPASS_DESCRIPTION_PER_VIEW_POSITION_X_ONLY_BIT_NVX = 0x00000002,
    SUBPASS_DESCRIPTION_FRAGMENT_REGION_BIT_QCOM = 0x00000004,
    SUBPASS_DESCRIPTION_SHADER_RESOLVE_BIT_QCOM = 0x00000008,
    SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT = 0x00000010,
    SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT = 0x00000020,
    SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT = 0x00000040,
    SUBPASS_DESCRIPTION_ENABLE_LEGACY_DITHERING_BIT_EXT = 0x00000080,
    SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_ARM =
        SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_COLOR_ACCESS_BIT_EXT,
    SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM =
        SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT,
    SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM =
        SUBPASS_DESCRIPTION_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT
);

/// VkAttachmentReference
#[derive(Debug)]
#[repr(C)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

/// VkSubpassDependency
#[derive(Debug)]
#[repr(C)]
pub struct SubpassDependency {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: PipelineStageFlags,
    pub dst_stage_mask: PipelineStageFlags,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}

/// PFN_vkCreateRenderPass
pub(crate) type CreateRenderPass = unsafe extern "C" fn(
    device: Device,
    info: *const RenderPassCreateInfo,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> Result;

/// PFN_vkDestroyRenderPass
pub(crate) type DestroyRenderPass = unsafe extern "C" fn(
    device: Device,
    render_pass: RenderPass,
    allocator: *const AllocationCallbacks,
);

def_ndh!(FramebufferT, Framebuffer);

/// VkFramebufferCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct FramebufferCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: RenderPass,
    pub attachment_count: u32,
    pub attachments: *const ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

def_flags!(
    FramebufferCreateFlags,
    FramebufferCreateFlagBits,
    FRAMEBUFFER_CREATE_IMAGELESS_BIT = 0x00000001,
    FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR = FRAMEBUFFER_CREATE_IMAGELESS_BIT
);

/// PFN_vkCreateFramebuffer
pub(crate) type CreateFramebuffer = unsafe extern "C" fn(
    device: Device,
    info: *const FramebufferCreateInfo,
    allocator: *const AllocationCallbacks,
    framebuffer: *mut Framebuffer,
) -> Result;

/// PFN_vkDestroyFramebuffer
pub(crate) type DestroyFramebuffer = unsafe extern "C" fn(
    device: Device,
    framebuffer: Framebuffer,
    allocator: *const AllocationCallbacks,
);

/// VkRenderPassBeginInfo
#[derive(Debug)]
#[repr(C)]
pub struct RenderPassBeginInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub render_pass: RenderPass,
    pub framebuffer: Framebuffer,
    pub render_area: Rect2d,
    pub clear_value_count: u32,
    pub clear_values: *const ClearValue,
}

/// VkClearColorValue
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClearColorValue {
    pub float32: [f32; 4],
    pub int32: [i32; 4],
    pub uint32: [u32; 4],
}

impl fmt::Debug for ClearColorValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ClearColorValue>")
    }
}

/// VkClearDepthStencilValue
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

/// VkClearValue
#[derive(Copy, Clone)]
#[repr(C)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}

impl fmt::Debug for ClearValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<ClearValue>")
    }
}

def_ids!(
    SubpassContents,
    SUBPASS_CONTENTS_INLINE = 0,
    SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1
);

/// PFN_vkCmdBeginRenderPass
pub(crate) type CmdBeginRenderPass = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    info: *const RenderPassBeginInfo,
    contents: SubpassContents,
);

/// PFN_vkCmdNextSubpass
pub(crate) type CmdNextSubpass =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, contents: SubpassContents);

/// PFN_vkCmdEndRenderPass
pub(crate) type CmdEndRenderPass = unsafe extern "C" fn(cmd_buf: CommandBuffer);

/// VkRenderingInfo (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct RenderingInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: RenderingFlags,
    pub render_area: Rect2d,
    pub layer_count: u32,
    pub view_mask: u32,
    pub color_attachment_count: u32,
    pub color_attachments: *const RenderingAttachmentInfo,
    pub depth_attachment: *const RenderingAttachmentInfo,
    pub stencil_attachment: *const RenderingAttachmentInfo,
}

def_flags!(
    RenderingFlags,
    RenderingFlagBits,
    RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT = 0x00000001,
    RENDERING_SUSPENDING_BIT = 0x00000002,
    RENDERING_RESUMING_BIT = 0x00000004,
    RENDERING_ENABLE_LEGACY_DITHERING_BIT_EXT = 0x00000008,
    RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR =
        RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT,
    RENDERING_SUSPENDING_BIT_KHR = RENDERING_SUSPENDING_BIT,
    RENDERING_RESUMING_BIT_KHR = RENDERING_RESUMING_BIT
);

/// VkRenderingAttachmentInfo (v1.3)
#[derive(Debug)]
#[repr(C)]
pub struct RenderingAttachmentInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
    pub resolve_mode: ResolveModeFlagBits,
    pub resolve_image_view: ImageView,
    pub resolve_image_layout: ImageLayout,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub clear_value: ClearValue,
}

def_flags!(
    ResolveModeFlags,
    ResolveModeFlagBits,
    RESOLVE_MODE_NONE = 0,
    RESOLVE_MODE_SAMPLE_ZERO_BIT = 0x00000001,
    RESOLVE_MODE_AVERAGE_BIT = 0x00000002,
    RESOLVE_MODE_MIN_BIT = 0x00000004,
    RESOLVE_MODE_MAX_BIT = 0x00000008,
    RESOLVE_MODE_NONE_KHR = RESOLVE_MODE_NONE,
    RESOLVE_MODE_SAMPLE_ZERO_BIT_KHR = RESOLVE_MODE_SAMPLE_ZERO_BIT,
    RESOLVE_MODE_AVERAGE_BIT_KHR = RESOLVE_MODE_AVERAGE_BIT,
    RESOLVE_MODE_MIN_BIT_KHR = RESOLVE_MODE_MIN_BIT,
    RESOLVE_MODE_MAX_BIT_KHR = RESOLVE_MODE_MAX_BIT
);

/// PFN_vkCmdBeginRendering (v1.3)
pub(crate) type CmdBeginRendering =
    unsafe extern "C" fn(command_buffer: CommandBuffer, rendering_info: *const RenderingInfo);

/// PFN_vkCmdEndRendering (v1.3)
pub(crate) type CmdEndRendering = unsafe extern "C" fn(command_buffer: CommandBuffer);

def_ndh!(DescriptorSetT, DescriptorSet);

def_ids!(
    DescriptorType,
    DESCRIPTOR_TYPE_SAMPLER = 0,
    DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
    DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
    DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
    DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
    DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
    DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
    DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
    DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
    DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
    DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
    DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK = 1000138000,
    DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR = 1000150000,
    DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_NV = 1000165000,
    DESCRIPTOR_TYPE_SAMPLE_WEIGHT_IMAGE_QCOM = 1000440000,
    DESCRIPTOR_TYPE_BLOCK_MATCH_IMAGE_QCOM = 1000440001,
    DESCRIPTOR_TYPE_MUTABLE_EXT = 1000351000,
    DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK_EXT = DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK,
    DESCRIPTOR_TYPE_MUTABLE_VALVE = DESCRIPTOR_TYPE_MUTABLE_EXT
);

/// PFN_vkCmdBindDescriptorSets
pub(crate) type CmdBindDescriptorSets = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    set_count: u32,
    desc_sets: *const DescriptorSet,
    dyn_off_count: u32,
    dyn_offs: *const u32,
);

/// PFN_vkCmdPushConstants
pub(crate) type CmdPushConstants = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    values: *const c_void,
);

/// VkWriteDescriptorSet
#[derive(Debug)]
#[repr(C)]
pub struct WriteDescriptorSet {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub image_infos: *const DescriptorImageInfo,
    pub buffer_infos: *const DescriptorBufferInfo,
    pub texel_buffer_views: *const BufferView,
}

/// VkCopyDescriptorSet
#[derive(Debug)]
#[repr(C)]
pub struct CopyDescriptorSet {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub src_set: DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

/// VkDescriptorImageInfo
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: Sampler,
    pub image_view: ImageView,
    pub image_layout: ImageLayout,
}

/// VkDescriptorBufferInfo
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: Buffer,
    pub offset: u64,
    pub range: u64,
}

/// PFN_vkUpdateDescriptorSets
pub(crate) type UpdateDescriptorSets = unsafe extern "C" fn(
    device: Device,
    write_count: u32,
    writes: *const WriteDescriptorSet,
    copy_count: u32,
    copies: *const CopyDescriptorSet,
);

def_ndh!(DescriptorSetLayoutT, DescriptorSetLayout);

/// VkDescriptorSetLayoutCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub bindings: *const DescriptorSetLayoutBinding,
}

def_flags!(
    DescriptorSetLayoutCreateFlags,
    DescriptorSetLayoutCreateFlagBits,
    DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT = 0x00000002,
    DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR = 0x00000001,
    DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT = 0x00000004,
    DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT_EXT =
        DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT,
    DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_VALVE =
        DESCRIPTOR_SET_LAYOUT_CREATE_HOST_ONLY_POOL_BIT_EXT
);

/// VkDescriptorSetLayoutBinding
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub immutable_samplers: *const Sampler,
}

/// VkDescriptorSetLayoutSupport (v1.1)
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorSetLayoutSupport {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub supported: *mut Bool32,
}

/// VkDescriptorSetVariableDescriptorCountLayoutSupport (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorSetVariableDescriptorCountLayoutSupport {
    pub s_type: StructureType,
    pub next: *mut c_void,
    pub max_variable_descriptor_count: u32,
}

/// PFN_vkCreateDescriptorSetLayout
pub(crate) type CreateDescriptorSetLayout = unsafe extern "C" fn(
    device: Device,
    info: *const DescriptorSetLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    set_layout: *mut DescriptorSetLayout,
) -> Result;

/// PFN_vkDestroyDescriptorSetLayout
pub(crate) type DestroyDescriptorSetLayout = unsafe extern "C" fn(
    device: Device,
    set_layout: DescriptorSetLayout,
    allocator: *const AllocationCallbacks,
);

/// PFN_vkGetDescriptorSetLayoutSupport (v1.1)
pub(crate) type GetDescriptorSetLayoutSupport = unsafe extern "C" fn(
    device: Device,
    create_info: *const DescriptorSetLayoutCreateInfo,
    support: *mut DescriptorSetLayoutSupport,
);

def_ndh!(DescriptorPoolT, DescriptorPool);

/// VkDescriptorPoolCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub pool_sizes: *const DescriptorPoolSize,
}

def_flags!(
    DescriptorPoolCreateFlags,
    DescriptorPoolCreateFlagBits,
    DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001,
    DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT = 0x00000002,
    DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT = 0x00000004,
    DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT_EXT = DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT,
    DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE = DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_EXT
);

/// VkDescriptorPoolSize
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorPoolSize {
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
}

def_flags!(DescriptorPoolResetFlags, DescriptorPoolResetFlagBits,);

/// VkDescriptorSetAllocateInfo
#[derive(Debug)]
#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set_count: u32,
    pub set_layouts: *const DescriptorSetLayout,
}

/// PFN_vkCreateDescriptorPool
pub(crate) type CreateDescriptorPool = unsafe extern "C" fn(
    device: Device,
    info: *const DescriptorPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    desc_pool: *mut DescriptorPool,
) -> Result;

/// PFN_vkResetDescriptorPool
pub(crate) type ResetDescriptorPool = unsafe extern "C" fn(
    device: Device,
    desc_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> Result;

/// PFN_vkDestroyDescriptorPool
pub(crate) type DestroyDescriptorPool = unsafe extern "C" fn(
    device: Device,
    desc_pool: DescriptorPool,
    allocator: *const AllocationCallbacks,
);

/// PFN_vkAllocateDescriptorSets
pub(crate) type AllocateDescriptorSets = unsafe extern "C" fn(
    device: Device,
    info: *const DescriptorSetAllocateInfo,
    desc_sets: *mut DescriptorSet,
) -> Result;

/// PFN_vkFreeDescriptorSets
pub(crate) type FreeDescriptorSets = unsafe extern "C" fn(
    device: Device,
    desc_pool: DescriptorPool,
    count: u32,
    desc_sets: *const DescriptorSet,
) -> Result;

/// VkBufferDeviceAddressInfo (v1.2)
#[derive(Debug)]
#[repr(C)]
pub struct BufferDeviceAddressInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub buffer: Buffer,
}

/// VkGetBufferDeviceAddress (v1.2)
pub(crate) type GetBufferDeviceAddress =
    unsafe extern "C" fn(device: Device, info: *const BufferDeviceAddressInfo) -> u64;

/// VkGetBufferOpaqueCaptureAddress (v1.2)
pub(crate) type GetBufferOpaqueCaptureAddress =
    unsafe extern "C" fn(device: Device, info: *const BufferDeviceAddressInfo) -> u64;

def_ndh!(ShaderModuleT, ShaderModule);

/// VkShaderModuleCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct ShaderModuleCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: c_size_t,
    pub code: *const u32,
}

def_flags!(ShaderModuleCreateFlags, ShaderModuleCreateFlagBits,);

/// PFN_vkCreateShaderModule
pub(crate) type CreateShaderModule = unsafe extern "C" fn(
    device: Device,
    info: *const ShaderModuleCreateInfo,
    allocator: *const AllocationCallbacks,
    shader_mod: *mut ShaderModule,
) -> Result;

/// PFN_vkDestroyShaderModule
pub(crate) type DestroyShaderModule = unsafe extern "C" fn(
    device: Device,
    shader_mod: ShaderModule,
    allocator: *const AllocationCallbacks,
);

def_ndh!(PipelineT, Pipeline);

def_ids!(
    PipelineBindPoint,
    PIPELINE_BIND_POINT_GRAPHICS = 0,
    PIPELINE_BIND_POINT_COMPUTE = 1,
    PIPELINE_BIND_POINT_RAY_TRACING_KHR = 1000165000,
    PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI = 1000369003,
    PIPELINE_BIND_POINT_RAY_TRACING_NV = PIPELINE_BIND_POINT_RAY_TRACING_KHR
);

/// PFN_vkCmdBindPipeline
pub(crate) type CmdBindPipeline =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, bind_point: PipelineBindPoint, pipeline: Pipeline);

def_flags!(
    PipelineCreateFlags,
    PipelineCreateFlagBits,
    PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,
    PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,
    PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004,
    PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 0x00000008,
    PIPELINE_CREATE_DISPATCH_BASE_BIT = 0x00000010,
    PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 0x00000100,
    PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT = 0x00000200,
    PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR = 0x00200000,
    PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT = 0x00400000,
    PIPELINE_CREATE_RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_BIT_KHR = 0x00004000,
    PIPELINE_CREATE_RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_BIT_KHR = 0x00008000,
    PIPELINE_CREATE_RAY_TRACING_NO_NULL_MISS_SHADERS_BIT_KHR = 0x00010000,
    PIPELINE_CREATE_RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_BIT_KHR = 0x00020000,
    PIPELINE_CREATE_RAY_TRACING_SKIP_TRIANGLES_BIT_KHR = 0x00001000,
    PIPELINE_CREATE_RAY_TRACING_SKIP_AABBS_BIT_KHR = 0x00002000,
    PIPELINE_CREATE_RAY_TRACING_SHADER_GROUP_HANDLE_CAPTURE_REPLAY_BIT_KHR = 0x00080000,
    PIPELINE_CREATE_DEFER_COMPILE_BIT_NV = 0x00000020,
    PIPELINE_CREATE_CAPTURE_STATISTICS_BIT_KHR = 0x00000040,
    PIPELINE_CREATE_CAPTURE_INTERNAL_REPRESENTATIONS_BIT_KHR = 0x00000080,
    PIPELINE_CREATE_INDIRECT_BINDABLE_BIT_NV = 0x00040000,
    PIPELINE_CREATE_LIBRARY_BIT_KHR = 0x00000800,
    PIPELINE_CREATE_RETAIN_LINK_TIME_OPTIMIZATION_INFO_BIT_EXT = 0x00800000,
    PIPELINE_CREATE_LINK_TIME_OPTIMIZATION_BIT_EXT = 0x00000400,
    PIPELINE_CREATE_RAY_TRACING_ALLOW_MOTION_BIT_NV = 0x00100000,
    PIPELINE_CREATE_COLOR_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 0x02000000,
    PIPELINE_CREATE_DEPTH_STENCIL_ATTACHMENT_FEEDBACK_LOOP_BIT_EXT = 0x04000000,
    PIPELINE_CREATE_RAY_TRACING_OPACITY_MICROMAP_BIT_EXT = 0x01000000,
    PIPELINE_CREATE_NO_PROTECTED_ACCESS_BIT_EXT = 0x08000000,
    PIPELINE_CREATE_PROTECTED_ACCESS_ONLY_BIT_EXT = 0x40000000,
    PIPELINE_CREATE_DISPATCH_BASE = PIPELINE_CREATE_DISPATCH_BASE_BIT,
    PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR =
        PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR,
    PIPELINE_RASTERIZATION_STATE_CREATE_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT =
        PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT,
    PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT_KHR =
        PIPELINE_CREATE_VIEW_INDEX_FROM_DEVICE_INDEX_BIT,
    PIPELINE_CREATE_DISPATCH_BASE_KHR = PIPELINE_CREATE_DISPATCH_BASE,
    PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT_EXT =
        PIPELINE_CREATE_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT,
    PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT_EXT = PIPELINE_CREATE_EARLY_RETURN_ON_FAILURE_BIT
);

/// PFN_vkCreateGraphicsPipelines
pub(crate) type CreateGraphicsPipelines = unsafe extern "C" fn(
    device: Device,
    pl_cache: PipelineCache,
    info_count: u32,
    infos: *const GraphicsPipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;

/// PFN_vkCreateComputePipelines
pub(crate) type CreateComputePipelines = unsafe extern "C" fn(
    device: Device,
    pl_cache: PipelineCache,
    info_count: u32,
    infos: *const ComputePipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> Result;

/// PFN_vkDestroyPipeline
pub(crate) type DestroyPipeline =
    unsafe extern "C" fn(device: Device, pipeline: Pipeline, allocator: *const AllocationCallbacks);

def_flags!(
    ShaderStageFlags,
    ShaderStageFlagBits,
    SHADER_STAGE_VERTEX_BIT = 0x00000001,
    SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
    SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
    SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
    SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
    SHADER_STAGE_COMPUTE_BIT = 0x00000020,
    SHADER_STAGE_ALL_GRAPHICS = 0x0000001F,
    SHADER_STAGE_ALL = 0x7FFFFFFF,
    SHADER_STAGE_RAYGEN_BIT_KHR = 0x00000100,
    SHADER_STAGE_ANY_HIT_BIT_KHR = 0x00000200,
    SHADER_STAGE_CLOSEST_HIT_BIT_KHR = 0x00000400,
    SHADER_STAGE_MISS_BIT_KHR = 0x00000800,
    SHADER_STAGE_INTERSECTION_BIT_KHR = 0x00001000,
    SHADER_STAGE_CALLABLE_BIT_KHR = 0x00002000,
    SHADER_STAGE_TASK_BIT_EXT = 0x00000040,
    SHADER_STAGE_MESH_BIT_EXT = 0x00000080,
    SHADER_STAGE_SUBPASS_SHADING_BIT_HUAWEI = 0x00004000,
    SHADER_STAGE_RAYGEN_BIT_NV = SHADER_STAGE_RAYGEN_BIT_KHR,
    SHADER_STAGE_ANY_HIT_BIT_NV = SHADER_STAGE_ANY_HIT_BIT_KHR,
    SHADER_STAGE_CLOSEST_HIT_BIT_NV = SHADER_STAGE_CLOSEST_HIT_BIT_KHR,
    SHADER_STAGE_MISS_BIT_NV = SHADER_STAGE_MISS_BIT_KHR,
    SHADER_STAGE_INTERSECTION_BIT_NV = SHADER_STAGE_INTERSECTION_BIT_KHR,
    SHADER_STAGE_CALLABLE_BIT_NV = SHADER_STAGE_CALLABLE_BIT_KHR,
    SHADER_STAGE_TASK_BIT_NV = SHADER_STAGE_TASK_BIT_EXT,
    SHADER_STAGE_MESH_BIT_NV = SHADER_STAGE_MESH_BIT_EXT
);

/// VkPipelineShaderStageCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlags,
    pub module: ShaderModule,
    pub name: *const c_char,
    pub specialization_info: *const SpecializationInfo,
}

def_flags!(
    PipelineShaderStageCreateFlags,
    PipelineShaderStageCreateFlagBits,
    PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT = 0x00000001,
    PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT = 0x00000002,
    PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT_EXT =
        PIPELINE_SHADER_STAGE_CREATE_ALLOW_VARYING_SUBGROUP_SIZE_BIT,
    PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT_EXT =
        PIPELINE_SHADER_STAGE_CREATE_REQUIRE_FULL_SUBGROUPS_BIT
);

/// VkSpecializationInfo
#[derive(Debug)]
#[repr(C)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub map_entries: *const SpecializationMapEntry,
    pub data_size: c_size_t,
    pub data: *const c_void,
}

/// VkSpecializationMapEntry
#[derive(Debug)]
#[repr(C)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: c_size_t,
}

/// VkGraphicsPipelineCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub stages: *const PipelineShaderStageCreateInfo,
    pub vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub tessellation_state: *const PipelineTessellationStateCreateInfo,
    pub viewport_state: *const PipelineViewportStateCreateInfo,
    pub rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: PipelineLayout,
    pub render_pass: RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

def_flags!(
    PipelineVertexInputStateCreateFlags,
    PipelineVertexInputStateCreateFlagBits,
);

def_flags!(
    PipelineInputAssemblyStateCreateFlags,
    PipelineInputAssemblyStateCreateFlagBits,
);

def_flags!(
    PipelineTessellationStateCreateFlags,
    PipelineTessellationStateCreateFlagBits,
);

def_flags!(
    PipelineViewportStateCreateFlags,
    PipelineViewportStateCreateFlagBits,
);

def_flags!(
    PipelineRasterizationStateCreateFlags,
    PipelineRasterizationStateCreateFlagBits,
);

def_flags!(
    PipelineMultisampleStateCreateFlags,
    PipelineMultisampleStateCreateFlagBits,
);

def_flags!(
    PipelineDepthStencilStateCreateFlags,
    PipelineDepthStencilStateCreateFlagBits,
    PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT =
        0x00000001,
    PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT =
        0x00000002,
    PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM =
        PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_EXT,
    PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM =
        PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_EXT
);

def_flags!(
    PipelineColorBlendStateCreateFlags,
    PipelineColorBlendStateCreateFlagBits,
    PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT = 0x00000001,
    PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM =
        PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_EXT
);

def_flags!(
    PipelineDynamicStateCreateFlags,
    PipelineDynamicStateCreateFlagBits,
);

/// VkPipelineVertexInputStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

/// VkVertexInputBindingDescription
#[derive(Debug)]
#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

/// VkVertexInputAttributeDescription
#[derive(Debug)]
#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

def_ids!(
    VertexInputRate,
    VERTEX_INPUT_RATE_VERTEX = 0,
    VERTEX_INPUT_RATE_INSTANCE = 1
);

/// VkPipelineInputAssemblyStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enable: Bool32,
}

def_ids!(
    PrimitiveTopology,
    PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
    PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
    PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
    PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
    PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
    PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
    PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
    PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
    PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
    PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
    PRIMITIVE_TOPOLOGY_PATCH_LIST = 10
);

/// PFN_vkCmdSetPrimitiveToppology
pub(crate) type CmdSetPrimitiveTopology =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, topology: PrimitiveTopology);

/// PFN_vkCmdSetPrimitiveRestartEnable (v1.3)
pub(crate) type CmdSetPrimitiveRestartEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

def_ids!(
    IndexType,
    INDEX_TYPE_UINT16 = 0,
    INDEX_TYPE_UINT32 = 1,
    INDEX_TYPE_NONE_KHR = 1000165000,
    INDEX_TYPE_UINT8_EXT = 1000265000,
    INDEX_TYPE_NONE_NV = INDEX_TYPE_NONE_KHR
);

/// VkDrawIndirectCommand
#[derive(Debug)]
#[repr(C)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

/// VkDrawIndexedIndirectCommand
#[derive(Debug)]
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: i32,
    pub first_instance: u32,
}

/// PFN_vkBindIndexBuffers
pub(crate) type CmdBindIndexBuffer =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, buffer: Buffer, offset: u64, idx_type: IndexType);

/// PFN_vkCmdBindVertexBuffers
pub(crate) type CmdBindVertexBuffers = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const u64,
);

/// PFN_vkCmdDraw
pub(crate) type CmdDraw = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    vert_count: u32,
    inst_count: u32,
    first_vert: u32,
    first_inst: u32,
);

/// PFN_vkCmdDrawIndexed
pub(crate) type CmdDrawIndexed = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    idx_count: u32,
    inst_count: u32,
    first_idx: u32,
    vert_off: i32,
    first_inst: u32,
);

/// PFN_vkCmdDrawIndirect
pub(crate) type CmdDrawIndirect = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    buffer: Buffer,
    offset: u64,
    draw_count: u32,
    stride: u32,
);

/// PFN_vkCmdDrawIndexedIndirect
pub(crate) type CmdDrawIndexedIndirect = CmdDrawIndirect;

/// VkPipelineTessellationStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineTessellationStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineTessellationStateCreateFlags,
    pub patch_control_points: u32,
}

/// VkPipelineViewportStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub viewports: *const Viewport,
    pub scissor_count: u32,
    pub scissors: *const Rect2d,
}

/// VkViewport
#[derive(Debug)]
#[repr(C)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

/// PFN_vkCmdSetViewport
pub(crate) type CmdSetViewport = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    first_vport: u32,
    vport_count: u32,
    viewports: *const Viewport,
);

/// PFN_vkCmdSetViewportWithCount (v1.3)
pub(crate) type CmdSetViewportWithCount =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, vport_count: u32, viewports: *const Viewport);

/// PFN_vkCmdSetScissor
pub(crate) type CmdSetScissor = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    first_sciss: u32,
    sciss_count: u32,
    scissors: *const Rect2d,
);

/// PFN_vkCmdSetScissorWithCount (v1.3)
pub(crate) type CmdSetScissorWithCount =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, sciss_count: u32, scissors: *const Rect2d);

/// VkPipelineRasterizationStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: Bool32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

def_ids!(
    PolygonMode,
    POLYGON_MODE_FILL = 0,
    POLYGON_MODE_LINE = 1,
    POLYGON_MODE_POINT = 2,
    POLYGON_MODE_FILL_RECTANGLE_NV = 1000153000
);

def_flags!(
    CullModeFlags,
    CullModeFlagBits,
    CULL_MODE_NONE = 0,
    CULL_MODE_FRONT_BIT = 0x00000001,
    CULL_MODE_BACK_BIT = 0x00000002,
    CULL_MODE_FRONT_AND_BACK = 0x00000003
);

def_ids!(
    FrontFace,
    FRONT_FACE_COUNTER_CLOCKWISE = 0,
    FRONT_FACE_CLOCKWISE = 1
);

/// PFN_vkCmdSetCullMode (v1.3)
pub(crate) type CmdSetCullMode =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, cull_mode: CullModeFlags);

/// PFN_vkCmdSetFrontFace (v1.3)
pub(crate) type CmdSetFrontFace =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, front_face: FrontFace);

/// PFN_vkCmdSetRasterizerDiscarEnable (v1.3)
pub(crate) type CmdSetRasterizerDiscardEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

/// PFN_vkCmdSetLineWidth
pub(crate) type CmdSetLineWidth = unsafe extern "C" fn(cmd_buf: CommandBuffer, line_width: f32);

/// PFN_vkCmdSetSetDepthBiasEnable (v1.3)
pub(crate) type CmdSetDepthBiasEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

/// PFN_vkCmdSetDepthBias
pub(crate) type CmdSetDepthBias = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    const_factor: f32,
    clamp_value: f32,
    slope_factor: f32,
);

/// VkPipelineMultisampleStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enable: Bool32,
    pub min_sample_shading: f32,
    pub sample_mask: *const u32,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}

/// VkPipelineDepthStencilStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub depth_compare_op: CompareOp,
    pub depth_bounds_test_enable: Bool32,
    pub stencil_test_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

/// VkStencilOpState
#[derive(Debug)]
#[repr(C)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

def_ids!(
    StencilOp,
    STENCIL_OP_KEEP = 0,
    STENCIL_OP_ZERO = 1,
    STENCIL_OP_REPLACE = 2,
    STENCIL_OP_INCREMENT_AND_CLAMP = 3,
    STENCIL_OP_DECREMENT_AND_CLAMP = 4,
    STENCIL_OP_INVERT = 5,
    STENCIL_OP_INCREMENT_AND_WRAP = 6,
    STENCIL_OP_DECREMENT_AND_WRAP = 7
);

def_flags!(
    StencilFaceFlags,
    StencilFaceFlagBits,
    STENCIL_FACE_FRONT_BIT = 0x00000001,
    STENCIL_FACE_BACK_BIT = 0x00000002,
    STENCIL_FACE_FRONT_AND_BACK = 0x00000003,
    STENCIL_FRONT_AND_BACK = STENCIL_FACE_FRONT_AND_BACK
);

/// PFN_vkCmdSetDepthTestEnable (v1.3)
pub(crate) type CmdSetDepthTestEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

/// PFN_vkCmdSetDepthWriteEnable (v1.3)
pub(crate) type CmdSetDepthWriteEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

/// PFN_vkCmdSetDepthCompareOp (v1.3)
pub(crate) type CmdSetDepthCompareOp =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, cmp_op: CompareOp);

/// PFN_vkCmdSetDepthBoundsTestEnable (v1.3)
pub(crate) type CmdSetDepthBoundsTestEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

/// PFN_vkCmdSetDepthBounds
pub(crate) type CmdSetDepthBounds =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, min: f32, max: f32);

/// PFN_vkCmdSetStencilTestEnable (v1.3)
pub(crate) type CmdSetStencilTestEnable =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, enable: Bool32);

/// PFN_vkCmdSetStencilop
pub(crate) type CmdSetStencilOp = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    cmp_op: CompareOp,
);

/// PFN_vkCmdSetStencilCompareMask
pub(crate) type CmdSetStencilCompareMask =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, face_mask: StencilFaceFlags, cmp_mask: u32);

/// PFN_vkCmdSetStencilWriteMask
pub(crate) type CmdSetStencilWriteMask =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, face_mask: StencilFaceFlags, write_mask: u32);

/// PFN_vkCmdSetStencilReference
pub(crate) type CmdSetStencilReference =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, face_mask: StencilFaceFlags, reference: u32);

/// VkPipelineColorBlendStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

/// VkPipelineColorBlendAttachmentState
#[derive(Debug)]
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

def_ids!(
    LogicOp,
    LOGIC_OP_CLEAR = 0,
    LOGIC_OP_AND = 1,
    LOGIC_OP_AND_REVERSE = 2,
    LOGIC_OP_COPY = 3,
    LOGIC_OP_AND_INVERTED = 4,
    LOGIC_OP_NO_OP = 5,
    LOGIC_OP_XOR = 6,
    LOGIC_OP_OR = 7,
    LOGIC_OP_NOR = 8,
    LOGIC_OP_EQUIVALENT = 9,
    LOGIC_OP_INVERT = 10,
    LOGIC_OP_OR_REVERSE = 11,
    LOGIC_OP_COPY_INVERTED = 12,
    LOGIC_OP_OR_INVERTED = 13,
    LOGIC_OP_NAND = 14,
    LOGIC_OP_SET = 15
);

def_ids!(
    BlendFactor,
    BLEND_FACTOR_ZERO = 0,
    BLEND_FACTOR_ONE = 1,
    BLEND_FACTOR_SRC_COLOR = 2,
    BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
    BLEND_FACTOR_DST_COLOR = 4,
    BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
    BLEND_FACTOR_SRC_ALPHA = 6,
    BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
    BLEND_FACTOR_DST_ALPHA = 8,
    BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
    BLEND_FACTOR_CONSTANT_COLOR = 10,
    BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
    BLEND_FACTOR_CONSTANT_ALPHA = 12,
    BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
    BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
    BLEND_FACTOR_SRC1_COLOR = 15,
    BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
    BLEND_FACTOR_SRC1_ALPHA = 17,
    BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18
);

def_ids!(
    BlendOp,
    BLEND_OP_ADD = 0,
    BLEND_OP_SUBTRACT = 1,
    BLEND_OP_REVERSE_SUBTRACT = 2,
    BLEND_OP_MIN = 3,
    BLEND_OP_MAX = 4,
    BLEND_OP_ZERO_EXT = 1000148000,
    BLEND_OP_SRC_EXT = 1000148001,
    BLEND_OP_DST_EXT = 1000148002,
    BLEND_OP_SRC_OVER_EXT = 1000148003,
    BLEND_OP_DST_OVER_EXT = 1000148004,
    BLEND_OP_SRC_IN_EXT = 1000148005,
    BLEND_OP_DST_IN_EXT = 1000148006,
    BLEND_OP_SRC_OUT_EXT = 1000148007,
    BLEND_OP_DST_OUT_EXT = 1000148008,
    BLEND_OP_SRC_ATOP_EXT = 1000148009,
    BLEND_OP_DST_ATOP_EXT = 1000148010,
    BLEND_OP_XOR_EXT = 1000148011,
    BLEND_OP_MULTIPLY_EXT = 1000148012,
    BLEND_OP_SCREEN_EXT = 1000148013,
    BLEND_OP_OVERLAY_EXT = 1000148014,
    BLEND_OP_DARKEN_EXT = 1000148015,
    BLEND_OP_LIGHTEN_EXT = 1000148016,
    BLEND_OP_COLORDODGE_EXT = 1000148017,
    BLEND_OP_COLORBURN_EXT = 1000148018,
    BLEND_OP_HARDLIGHT_EXT = 1000148019,
    BLEND_OP_SOFTLIGHT_EXT = 1000148020,
    BLEND_OP_DIFFERENCE_EXT = 1000148021,
    BLEND_OP_EXCLUSION_EXT = 1000148022,
    BLEND_OP_INVERT_EXT = 1000148023,
    BLEND_OP_INVERT_RGB_EXT = 1000148024,
    BLEND_OP_LINEARDODGE_EXT = 1000148025,
    BLEND_OP_LINEARBURN_EXT = 1000148026,
    BLEND_OP_VIVIDLIGHT_EXT = 1000148027,
    BLEND_OP_LINEARLIGHT_EXT = 1000148028,
    BLEND_OP_PINLIGHT_EXT = 1000148029,
    BLEND_OP_HARDMIX_EXT = 1000148030,
    BLEND_OP_HSL_HUE_EXT = 1000148031,
    BLEND_OP_HSL_SATURATION_EXT = 1000148032,
    BLEND_OP_HSL_COLOR_EXT = 1000148033,
    BLEND_OP_HSL_LUMINOSITY_EXT = 1000148034,
    BLEND_OP_PLUS_EXT = 1000148035,
    BLEND_OP_PLUS_CLAMPED_EXT = 1000148036,
    BLEND_OP_PLUS_CLAMPED_ALPHA_EXT = 1000148037,
    BLEND_OP_PLUS_DARKER_EXT = 1000148038,
    BLEND_OP_MINUS_EXT = 1000148039,
    BLEND_OP_MINUS_CLAMPED_EXT = 1000148040,
    BLEND_OP_CONTRAST_EXT = 1000148041,
    BLEND_OP_INVERT_OVG_EXT = 1000148042,
    BLEND_OP_RED_EXT = 1000148043,
    BLEND_OP_GREEN_EXT = 1000148044,
    BLEND_OP_BLUE_EXT = 1000148045
);

def_flags!(
    ColorComponentFlags,
    ColorComponentFlagBits,
    COLOR_COMPONENT_R_BIT = 0x00000001,
    COLOR_COMPONENT_G_BIT = 0x00000002,
    COLOR_COMPONENT_B_BIT = 0x00000004,
    COLOR_COMPONENT_A_BIT = 0x00000008
);

/// PFN_vkCmdSetBlendConstants
pub(crate) type CmdSetBlendConstants =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, blend_consts: *const f32);

/// VkPipelineDynamicStateCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub dynamic_states: *const DynamicState,
}

def_ids!(
    DynamicState,
    DYNAMIC_STATE_VIEWPORT = 0,
    DYNAMIC_STATE_SCISSOR = 1,
    DYNAMIC_STATE_LINE_WIDTH = 2,
    DYNAMIC_STATE_DEPTH_BIAS = 3,
    DYNAMIC_STATE_BLEND_CONSTANTS = 4,
    DYNAMIC_STATE_DEPTH_BOUNDS = 5,
    DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
    DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
    DYNAMIC_STATE_STENCIL_REFERENCE = 8,
    DYNAMIC_STATE_CULL_MODE = 1000267000,
    DYNAMIC_STATE_FRONT_FACE = 1000267001,
    DYNAMIC_STATE_PRIMITIVE_TOPOLOGY = 1000267002,
    DYNAMIC_STATE_VIEWPORT_WITH_COUNT = 1000267003,
    DYNAMIC_STATE_SCISSOR_WITH_COUNT = 1000267004,
    DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE = 1000267005,
    DYNAMIC_STATE_DEPTH_TEST_ENABLE = 1000267006,
    DYNAMIC_STATE_DEPTH_WRITE_ENABLE = 1000267007,
    DYNAMIC_STATE_DEPTH_COMPARE_OP = 1000267008,
    DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE = 1000267009,
    DYNAMIC_STATE_STENCIL_TEST_ENABLE = 1000267010,
    DYNAMIC_STATE_STENCIL_OP = 1000267011,
    DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE = 1000377001,
    DYNAMIC_STATE_DEPTH_BIAS_ENABLE = 1000377002,
    DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE = 1000377004,
    DYNAMIC_STATE_VIEWPORT_W_SCALING_NV = 1000087000,
    DYNAMIC_STATE_DISCARD_RECTANGLE_EXT = 1000099000,
    DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT = 1000143000,
    DYNAMIC_STATE_RAY_TRACING_PIPELINE_STACK_SIZE_KHR = 1000347000,
    DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV = 1000164004,
    DYNAMIC_STATE_VIEWPORT_COARSE_SAMPLE_ORDER_NV = 1000164006,
    DYNAMIC_STATE_EXCLUSIVE_SCISSOR_NV = 1000205001,
    DYNAMIC_STATE_FRAGMENT_SHADING_RATE_KHR = 1000226000,
    DYNAMIC_STATE_LINE_STIPPLE_EXT = 1000259000,
    DYNAMIC_STATE_VERTEX_INPUT_EXT = 1000352000,
    DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT = 1000377000,
    DYNAMIC_STATE_LOGIC_OP_EXT = 1000377003,
    DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT = 1000381000,
    DYNAMIC_STATE_TESSELLATION_DOMAIN_ORIGIN_EXT = 1000455002,
    DYNAMIC_STATE_DEPTH_CLAMP_ENABLE_EXT = 1000455003,
    DYNAMIC_STATE_POLYGON_MODE_EXT = 1000455004,
    DYNAMIC_STATE_RASTERIZATION_SAMPLES_EXT = 1000455005,
    DYNAMIC_STATE_SAMPLE_MASK_EXT = 1000455006,
    DYNAMIC_STATE_ALPHA_TO_COVERAGE_ENABLE_EXT = 1000455007,
    DYNAMIC_STATE_ALPHA_TO_ONE_ENABLE_EXT = 1000455008,
    DYNAMIC_STATE_LOGIC_OP_ENABLE_EXT = 1000455009,
    DYNAMIC_STATE_COLOR_BLEND_ENABLE_EXT = 1000455010,
    DYNAMIC_STATE_COLOR_BLEND_EQUATION_EXT = 1000455011,
    DYNAMIC_STATE_COLOR_WRITE_MASK_EXT = 1000455012,
    DYNAMIC_STATE_RASTERIZATION_STREAM_EXT = 1000455013,
    DYNAMIC_STATE_CONSERVATIVE_RASTERIZATION_MODE_EXT = 1000455014,
    DYNAMIC_STATE_EXTRA_PRIMITIVE_OVERESTIMATION_SIZE_EXT = 1000455015,
    DYNAMIC_STATE_DEPTH_CLIP_ENABLE_EXT = 1000455016,
    DYNAMIC_STATE_SAMPLE_LOCATIONS_ENABLE_EXT = 1000455017,
    DYNAMIC_STATE_COLOR_BLEND_ADVANCED_EXT = 1000455018,
    DYNAMIC_STATE_PROVOKING_VERTEX_MODE_EXT = 1000455019,
    DYNAMIC_STATE_LINE_RASTERIZATION_MODE_EXT = 1000455020,
    DYNAMIC_STATE_LINE_STIPPLE_ENABLE_EXT = 1000455021,
    DYNAMIC_STATE_DEPTH_CLIP_NEGATIVE_ONE_TO_ONE_EXT = 1000455022,
    DYNAMIC_STATE_VIEWPORT_W_SCALING_ENABLE_NV = 1000455023,
    DYNAMIC_STATE_VIEWPORT_SWIZZLE_NV = 1000455024,
    DYNAMIC_STATE_COVERAGE_TO_COLOR_ENABLE_NV = 1000455025,
    DYNAMIC_STATE_COVERAGE_TO_COLOR_LOCATION_NV = 1000455026,
    DYNAMIC_STATE_COVERAGE_MODULATION_MODE_NV = 1000455027,
    DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_ENABLE_NV = 1000455028,
    DYNAMIC_STATE_COVERAGE_MODULATION_TABLE_NV = 1000455029,
    DYNAMIC_STATE_SHADING_RATE_IMAGE_ENABLE_NV = 1000455030,
    DYNAMIC_STATE_REPRESENTATIVE_FRAGMENT_TEST_ENABLE_NV = 1000455031,
    DYNAMIC_STATE_COVERAGE_REDUCTION_MODE_NV = 1000455032,
    DYNAMIC_STATE_CULL_MODE_EXT = DYNAMIC_STATE_CULL_MODE,
    DYNAMIC_STATE_FRONT_FACE_EXT = DYNAMIC_STATE_FRONT_FACE,
    DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT = DYNAMIC_STATE_PRIMITIVE_TOPOLOGY,
    DYNAMIC_STATE_VIEWPORT_WITH_COUNT_EXT = DYNAMIC_STATE_VIEWPORT_WITH_COUNT,
    DYNAMIC_STATE_SCISSOR_WITH_COUNT_EXT = DYNAMIC_STATE_SCISSOR_WITH_COUNT,
    DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT = DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE,
    DYNAMIC_STATE_DEPTH_TEST_ENABLE_EXT = DYNAMIC_STATE_DEPTH_TEST_ENABLE,
    DYNAMIC_STATE_DEPTH_WRITE_ENABLE_EXT = DYNAMIC_STATE_DEPTH_WRITE_ENABLE,
    DYNAMIC_STATE_DEPTH_COMPARE_OP_EXT = DYNAMIC_STATE_DEPTH_COMPARE_OP,
    DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE_EXT = DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE,
    DYNAMIC_STATE_STENCIL_TEST_ENABLE_EXT = DYNAMIC_STATE_STENCIL_TEST_ENABLE,
    DYNAMIC_STATE_STENCIL_OP_EXT = DYNAMIC_STATE_STENCIL_OP,
    DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE_EXT = DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE,
    DYNAMIC_STATE_DEPTH_BIAS_ENABLE_EXT = DYNAMIC_STATE_DEPTH_BIAS_ENABLE,
    DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT = DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE
);

/// VkComputePipelineCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct ComputePipelineCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: PipelineLayout,
    pub base_pipeline_handle: Pipeline,
    pub base_pipeline_index: i32,
}

/// VkDispatchIndirectCommand
#[derive(Debug)]
#[repr(C)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

/// PFN_vkCmdDispatch
pub(crate) type CmdDispatch = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    grp_count_x: u32,
    grp_count_y: u32,
    grp_count_z: u32,
);

/// PFN_vkCmdDispatchIndirect
pub(crate) type CmdDispatchIndirect =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, buffer: Buffer, offset: u64);

def_ndh!(PipelineLayoutT, PipelineLayout);

/// VkPipelineLayoutCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub set_layouts: *const DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub push_constant_ranges: *const PushConstantRange,
}

def_flags!(
    PipelineLayoutCreateFlags,
    PipelineLayoutCreateFlagBits,
    PIPELINE_LAYOUT_CREATE_INDEPENDENT_SETS_BIT_EXT = 0x00000002
);

/// VkPushConstantRange
#[derive(Debug)]
#[repr(C)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

/// PFN_vkCreatePipelineLayout
pub(crate) type CreatePipelineLayout = unsafe extern "C" fn(
    device: Device,
    info: *const PipelineLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    pl_layout: *mut PipelineLayout,
) -> Result;

/// PFN_vkDestroyPipelineLayout
pub(crate) type DestroyPipelineLayout = unsafe extern "C" fn(
    device: Device,
    pl_layout: PipelineLayout,
    allocator: *const AllocationCallbacks,
);

def_ndh!(PipelineCacheT, PipelineCache);

/// VkPipelineCacheCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct PipelineCacheCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: c_size_t,
    pub initial_data: *const c_void,
}

def_flags!(
    PipelineCacheCreateFlags,
    PipelineCacheCreateFlagBits,
    PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT = 0x00000001,
    PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT_EXT =
        PIPELINE_CACHE_CREATE_EXTERNALLY_SYNCHRONIZED_BIT
);

/// VkPipelineCacheHeaderVersionOne
#[derive(Debug)]
#[repr(C)]
pub struct PipelineCacheHeaderVersionOne {
    pub header_size: u32,
    pub header_version: PipelineCacheHeaderVersion,
    pub vendor_id: u32,
    pub device_id: u32,
    pub pipeline_cache_uuid: [u8; 16],
}

def_ids!(
    PipelineCacheHeaderVersion,
    PIPELINE_CACHE_HEADER_VERSION_ONE = 1
);

/// PFN_vkCreatePipelineCache
pub(crate) type CreatePipelineCache = unsafe extern "C" fn(
    device: Device,
    info: *const PipelineCacheCreateInfo,
    allocator: *const AllocationCallbacks,
    pl_cache: *mut PipelineCache,
) -> Result;

/// PFN_vkMergePipelineCaches
pub(crate) type MergePipelineCaches = unsafe extern "C" fn(
    device: Device,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    src_caches: *const PipelineCache,
) -> Result;

/// PFN_vkGetPipelineCacheData
pub(crate) type GetPipelineCacheData = unsafe extern "C" fn(
    device: Device,
    pl_cache: PipelineCache,
    data_size: *mut c_size_t,
    data: *mut c_void,
) -> Result;

/// PFN_vkDestroyPipelineCache
pub(crate) type DestroyPipelineCache = unsafe extern "C" fn(
    device: Device,
    pl_cache: PipelineCache,
    allocator: *const AllocationCallbacks,
);

def_ndh!(QueryPoolT, QueryPool);

/// VkQueryPoolCreateInfo
#[derive(Debug)]
#[repr(C)]
pub struct QueryPoolCreateInfo {
    pub s_type: StructureType,
    pub next: *const c_void,
    pub flags: QueryPoolCreateFlags,
    pub query_type: QueryType,
    pub query_count: u32,
    pub pipeline_statistics: QueryPipelineStatisticFlags,
}

def_flags!(QueryPoolCreateFlags, QueryPoolCreateFlagBits,);

def_ids!(
    QueryType,
    QUERY_TYPE_OCCLUSION = 0,
    QUERY_TYPE_PIPELINE_STATISTICS = 1,
    QUERY_TYPE_TIMESTAMP = 2,
    QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT = 1000028004,
    QUERY_TYPE_PERFORMANCE_QUERY_KHR = 1000116000,
    QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR = 1000150000,
    QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR = 1000150001,
    QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV = 1000165000,
    QUERY_TYPE_PERFORMANCE_QUERY_INTEL = 1000210000,
    QUERY_TYPE_MESH_PRIMITIVES_GENERATED_EXT = 1000328000,
    QUERY_TYPE_PRIMITIVES_GENERATED_EXT = 1000382000,
    QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_BOTTOM_LEVEL_POINTERS_KHR = 1000386000,
    QUERY_TYPE_ACCELERATION_STRUCTURE_SIZE_KHR = 1000386001,
    QUERY_TYPE_MICROMAP_SERIALIZATION_SIZE_EXT = 1000396000,
    QUERY_TYPE_MICROMAP_COMPACTED_SIZE_EXT = 1000396001
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

/// PFN_vkCreateQueryPool
pub(crate) type CreateQueryPool = unsafe extern "C" fn(
    device: Device,
    info: *const QueryPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    query_pool: *mut QueryPool,
) -> Result;

/// PFN_vkDestroyQueryPool
pub(crate) type DestroyQueryPool = unsafe extern "C" fn(
    device: Device,
    query_pool: QueryPool,
    allocator: *const AllocationCallbacks,
);

/// PFN_vkGetQueryPoolResults
pub(crate) type GetQueryPoolResults = unsafe extern "C" fn(
    device: Device,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: c_size_t,
    data: *mut c_void,
    stride: u64,
    flags: QueryResultFlags,
) -> Result;

/// PFN_vkCmdResetQueryPool
pub(crate) type CmdResetQueryPool = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);

/// PFN_vkCmdBeginQuery
pub(crate) type CmdBeginQuery = unsafe extern "C" fn(
    cmd_buf: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);

/// PFN_vkCmdEndQuery
pub(crate) type CmdEndQuery =
    unsafe extern "C" fn(cmd_buf: CommandBuffer, query_pool: QueryPool, query: u32);
