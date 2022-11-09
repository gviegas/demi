// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_void;
use std::mem;
use std::result;

use crate::{
    c_size_t, AllocateCommandBuffers, AllocateDescriptorSets, AllocateMemory, AllocationCallbacks,
    BindBufferMemory, BindImageMemory, Bool32, Buffer, BufferCreateInfo, BufferView,
    BufferViewCreateInfo, CommandBuffer, CommandBufferAllocateInfo, CommandBufferResetFlags,
    CommandPool, CommandPoolCreateInfo, CommandPoolResetFlags, CommandPoolTrimFlags,
    ComputePipelineCreateInfo, CopyDescriptorSet, CreateBuffer, CreateBufferView,
    CreateCommandPool, CreateComputePipelines, CreateDescriptorPool, CreateDescriptorSetLayout,
    CreateFence, CreateFramebuffer, CreateGraphicsPipelines, CreateImage, CreateImageView,
    CreatePipelineCache, CreatePipelineLayout, CreateQueryPool, CreateRenderPass, CreateSampler,
    CreateSemaphore, CreateShaderModule, DescriptorPool, DescriptorPoolCreateInfo,
    DescriptorPoolResetFlags, DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout,
    DescriptorSetLayoutCreateInfo, DestroyBuffer, DestroyBufferView, DestroyCommandPool,
    DestroyDescriptorPool, DestroyDescriptorSetLayout, DestroyDevice, DestroyFence,
    DestroyFramebuffer, DestroyImage, DestroyImageView, DestroyPipeline, DestroyPipelineCache,
    DestroyPipelineLayout, DestroyQueryPool, DestroyRenderPass, DestroySampler, DestroySemaphore,
    DestroyShaderModule, Device, DeviceMemory, DeviceWaitIdle, Fence, FenceCreateInfo,
    FlushMappedMemoryRanges, Framebuffer, FramebufferCreateInfo, FreeCommandBuffers,
    FreeDescriptorSets, FreeMemory, GetBufferMemoryRequirements, GetDeviceQueue, GetFenceStatus,
    GetImageMemoryRequirements, GetPipelineCacheData, GetQueryPoolResults,
    GetSemaphoreCounterValue, GraphicsPipelineCreateInfo, Image, ImageCreateInfo, ImageView,
    ImageViewCreateInfo, InstanceFp, InvalidateMappedMemoryRanges, MapMemory, MappedMemoryRange,
    MemoryAllocateInfo, MemoryMapFlags, MemoryRequirements, MergePipelineCaches, Pipeline,
    PipelineCache, PipelineCacheCreateInfo, PipelineLayout, PipelineLayoutCreateInfo, QueryPool,
    QueryPoolCreateInfo, QueryResultFlags, Queue, QueueSubmit, QueueSubmit2, QueueWaitIdle,
    RenderPass, RenderPassCreateInfo, ResetCommandBuffer, ResetCommandPool, ResetDescriptorPool,
    ResetFences, Result, Sampler, SamplerCreateInfo, Semaphore, SemaphoreCreateInfo,
    SemaphoreSignalInfo, SemaphoreWaitInfo, ShaderModule, ShaderModuleCreateInfo, SignalSemaphore,
    SubmitInfo, SubmitInfo2, TrimCommandPool, UnmapMemory, UpdateDescriptorSets, WaitForFences,
    WaitSemaphores, WriteDescriptorSet,
};

/// Device-level commands.
pub struct DeviceFp {
    destroy_device: DestroyDevice,
    get_device_queue: GetDeviceQueue,
    create_command_pool: CreateCommandPool,
    reset_command_pool: ResetCommandPool,
    destroy_command_pool: DestroyCommandPool,
    allocate_command_buffers: AllocateCommandBuffers,
    free_command_buffers: FreeCommandBuffers,
    create_fence: CreateFence,
    get_fence_status: GetFenceStatus,
    reset_fences: ResetFences,
    wait_for_fences: WaitForFences,
    destroy_fence: DestroyFence,
    create_semaphore: CreateSemaphore,
    destroy_semaphore: DestroySemaphore,
    device_wait_idle: DeviceWaitIdle,
    create_render_pass: CreateRenderPass,
    destroy_render_pass: DestroyRenderPass,
    create_framebuffer: CreateFramebuffer,
    destroy_framebuffer: DestroyFramebuffer,
    create_shader_module: CreateShaderModule,
    destroy_shader_module: DestroyShaderModule,
    create_compute_pipelines: CreateComputePipelines,
    create_graphics_pipelines: CreateGraphicsPipelines,
    destroy_pipeline: DestroyPipeline,
    create_pipeline_cache: CreatePipelineCache,
    merge_pipeline_caches: MergePipelineCaches,
    get_pipeline_cache_data: GetPipelineCacheData,
    destroy_pipeline_cache: DestroyPipelineCache,
    allocate_memory: AllocateMemory,
    free_memory: FreeMemory,
    map_memory: MapMemory,
    unmap_memory: UnmapMemory,
    flush_mapped_memory_ranges: FlushMappedMemoryRanges,
    invalidate_mapped_memory_ranges: InvalidateMappedMemoryRanges,
    create_buffer: CreateBuffer,
    destroy_buffer: DestroyBuffer,
    get_buffer_memory_requirements: GetBufferMemoryRequirements,
    bind_buffer_memory: BindBufferMemory,
    create_image: CreateImage,
    destroy_image: DestroyImage,
    get_image_memory_requirements: GetImageMemoryRequirements,
    bind_image_memory: BindImageMemory,
    create_buffer_view: CreateBufferView,
    destroy_buffer_view: DestroyBufferView,
    create_image_view: CreateImageView,
    destroy_image_view: DestroyImageView,
    create_sampler: CreateSampler,
    destroy_sampler: DestroySampler,
    create_descriptor_set_layout: CreateDescriptorSetLayout,
    destroy_descriptor_set_layout: DestroyDescriptorSetLayout,
    create_pipeline_layout: CreatePipelineLayout,
    destroy_pipeline_layout: DestroyPipelineLayout,
    create_descriptor_pool: CreateDescriptorPool,
    reset_descriptor_pool: ResetDescriptorPool,
    destroy_descriptor_pool: DestroyDescriptorPool,
    allocate_descriptor_sets: AllocateDescriptorSets,
    update_descriptor_sets: UpdateDescriptorSets,
    free_descriptor_sets: FreeDescriptorSets,
    create_query_pool: CreateQueryPool,
    get_query_pool_results: GetQueryPoolResults,
    destroy_query_pool: DestroyQueryPool,

    queue_submit: QueueSubmit,
    queue_wait_idle: QueueWaitIdle,

    reset_command_buffer: ResetCommandBuffer,

    // v1.1
    trim_command_pool: Option<TrimCommandPool>,

    // v1.2
    get_semaphore_counter_value: Option<GetSemaphoreCounterValue>,
    wait_semaphores: Option<WaitSemaphores>,
    signal_semaphore: Option<SignalSemaphore>,

    // v1.3
    queue_submit_2: Option<QueueSubmit2>,
}

impl DeviceFp {
    /// Initializes the function pointers for a given `Device`.
    ///
    /// `device` must have been created from `instance_fp`.
    pub unsafe fn new(device: Device, instance_fp: &InstanceFp) -> result::Result<Self, String> {
        debug_assert!(!device.is_null());

        macro_rules! get {
            ($bs:expr) => {
                match instance_fp.get_device_proc_addr(device, $bs.as_ptr().cast()) {
                    Some(x) => Ok(mem::transmute(x)),
                    None => Err(format!(
                        "could not obtain FP: {}",
                        String::from_utf8_lossy(&$bs[..$bs.len() - 1])
                    )),
                }
            };
        }

        Ok(Self {
            destroy_device: get!(b"vkDestroyDevice\0")?,
            get_device_queue: get!(b"vkGetDeviceQueue\0")?,
            create_command_pool: get!(b"vkCreateCommandPool\0")?,
            reset_command_pool: get!(b"vkResetCommandPool\0")?,
            destroy_command_pool: get!(b"vkDestroyCommandPool\0")?,
            allocate_command_buffers: get!(b"vkAllocateCommandBuffers\0")?,
            free_command_buffers: get!(b"vkFreeCommandBuffers\0")?,
            create_fence: get!(b"vkCreateFence\0")?,
            get_fence_status: get!(b"vkGetFenceStatus\0")?,
            reset_fences: get!(b"vkResetFences\0")?,
            wait_for_fences: get!(b"vkWaitForFences\0")?,
            destroy_fence: get!(b"vkDestroyFence\0")?,
            create_semaphore: get!(b"vkCreateSemaphore\0")?,
            destroy_semaphore: get!(b"vkDestroySemaphore\0")?,
            device_wait_idle: get!(b"vkDeviceWaitIdle\0")?,
            create_render_pass: get!(b"vkCreateRenderPass\0")?,
            destroy_render_pass: get!(b"vkDestroyRenderPass\0")?,
            create_framebuffer: get!(b"vkCreateFramebuffer\0")?,
            destroy_framebuffer: get!(b"vkDestroyFramebuffer\0")?,
            create_shader_module: get!(b"vkCreateShaderModule\0")?,
            destroy_shader_module: get!(b"vkDestroyShaderModule\0")?,
            create_compute_pipelines: get!(b"vkCreateComputePipelines\0")?,
            create_graphics_pipelines: get!(b"vkCreateGraphicsPipelines\0")?,
            destroy_pipeline: get!(b"vkDestroyPipeline\0")?,
            create_pipeline_cache: get!(b"vkCreatePipelineCache\0")?,
            merge_pipeline_caches: get!(b"vkMergePipelineCaches\0")?,
            get_pipeline_cache_data: get!(b"vkGetPipelineCacheData\0")?,
            destroy_pipeline_cache: get!(b"vkDestroyPipelineCache\0")?,
            allocate_memory: get!(b"vkAllocateMemory\0")?,
            free_memory: get!(b"vkFreeMemory\0")?,
            map_memory: get!(b"vkMapMemory\0")?,
            unmap_memory: get!(b"vkUnmapMemory\0")?,
            flush_mapped_memory_ranges: get!(b"vkFlushMappedMemoryRanges\0")?,
            invalidate_mapped_memory_ranges: get!(b"vkInvalidateMappedMemoryRanges\0")?,
            create_buffer: get!(b"vkCreateBuffer\0")?,
            destroy_buffer: get!(b"vkDestroyBuffer\0")?,
            get_buffer_memory_requirements: get!(b"vkGetBufferMemoryRequirements\0")?,
            bind_buffer_memory: get!(b"vkBindBufferMemory\0")?,
            create_image: get!(b"vkCreateImage\0")?,
            destroy_image: get!(b"vkDestroyImage\0")?,
            get_image_memory_requirements: get!(b"vkGetImageMemoryRequirements\0")?,
            bind_image_memory: get!(b"vkBindImageMemory\0")?,
            create_buffer_view: get!(b"vkCreateBufferView\0")?,
            destroy_buffer_view: get!(b"vkDestroyBufferView\0")?,
            create_image_view: get!(b"vkCreateImageView\0")?,
            destroy_image_view: get!(b"vkDestroyImageView\0")?,
            create_sampler: get!(b"vkCreateSampler\0")?,
            destroy_sampler: get!(b"vkDestroySampler\0")?,
            create_descriptor_set_layout: get!(b"vkCreateDescriptorSetLayout\0")?,
            destroy_descriptor_set_layout: get!(b"vkDestroyDescriptorSetLayout\0")?,
            create_pipeline_layout: get!(b"vkCreatePipelineLayout\0")?,
            destroy_pipeline_layout: get!(b"vkDestroyPipelineLayout\0")?,
            create_descriptor_pool: get!(b"vkCreateDescriptorPool\0")?,
            reset_descriptor_pool: get!(b"vkResetDescriptorPool\0")?,
            destroy_descriptor_pool: get!(b"vkDestroyDescriptorPool\0")?,
            allocate_descriptor_sets: get!(b"vkAllocateDescriptorSets\0")?,
            update_descriptor_sets: get!(b"vkUpdateDescriptorSets\0")?,
            free_descriptor_sets: get!(b"vkFreeDescriptorSets\0")?,
            create_query_pool: get!(b"vkCreateQueryPool\0")?,
            get_query_pool_results: get!(b"vkGetQueryPoolResults\0")?,
            destroy_query_pool: get!(b"vkDestroyQueryPool\0")?,

            queue_submit: get!(b"vkQueueSubmit\0")?,
            queue_wait_idle: get!(b"vkQueueWaitIdle\0")?,

            reset_command_buffer: get!(b"vkResetCommandBuffer\0")?,

            trim_command_pool: get!(b"vkTrimCommandPool\0").ok(),

            get_semaphore_counter_value: get!(b"vkGetSemaphoreCounterValue\0").ok(),
            wait_semaphores: get!(b"vkWaitSemaphores\0").ok(),
            signal_semaphore: get!(b"vkSignalSemaphore\0").ok(),

            queue_submit_2: get!(b"vkQueueSubmit2\0").ok(),
        })
    }
}

impl DeviceFp {
    /// vkDestroyDevice
    ///
    /// The `DeviceFp` must not be used anymore.
    pub unsafe fn destroy_device(&mut self, device: Device, allocator: *const AllocationCallbacks) {
        (self.destroy_device)(device, allocator);
    }

    /// vkGetDeviceQueue
    pub unsafe fn get_device_queue(
        &self,
        device: Device,
        queue_family_index: u32,
        queue_index: u32,
        queue: *mut Queue,
    ) {
        (self.get_device_queue)(device, queue_family_index, queue_index, queue);
    }

    /// vkCreateCommandPool
    pub unsafe fn create_command_pool(
        &self,
        device: Device,
        create_info: *const CommandPoolCreateInfo,
        allocator: *const AllocationCallbacks,
        command_pool: *mut CommandPool,
    ) -> Result {
        (self.create_command_pool)(device, create_info, allocator, command_pool)
    }

    /// vkTrimCommandPool
    /// [v1.1]
    pub unsafe fn trim_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolTrimFlags,
    ) {
        debug_assert!(self.trim_command_pool.is_some());
        (self.trim_command_pool.unwrap_unchecked())(device, command_pool, flags);
    }

    /// vkResetCommandPool
    pub unsafe fn reset_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result {
        (self.reset_command_pool)(device, command_pool, flags)
    }

    /// vkDestroyCommandPool
    pub unsafe fn destroy_command_pool(
        &self,
        device: Device,
        command_pool: CommandPool,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_command_pool)(device, command_pool, allocator);
    }

    /// vkAllocateCommandBuffers
    pub unsafe fn allocate_command_buffers(
        &self,
        device: Device,
        allocate_info: *const CommandBufferAllocateInfo,
        command_buffers: *mut CommandBuffer,
    ) -> Result {
        (self.allocate_command_buffers)(device, allocate_info, command_buffers)
    }

    /// vkFreeCommandBuffers
    pub unsafe fn free_command_buffers(
        &self,
        device: Device,
        command_pool: CommandPool,
        command_buffer_count: u32,
        command_buffers: *const CommandBuffer,
    ) {
        (self.free_command_buffers)(device, command_pool, command_buffer_count, command_buffers);
    }

    /// vkCreateFence
    pub unsafe fn create_fence(
        &self,
        device: Device,
        create_info: *const FenceCreateInfo,
        allocator: *const AllocationCallbacks,
        fence: *mut Fence,
    ) -> Result {
        (self.create_fence)(device, create_info, allocator, fence)
    }

    /// vkGetFenceStatus
    pub unsafe fn get_fence_status(&self, device: Device, fence: Fence) -> Result {
        (self.get_fence_status)(device, fence)
    }

    /// vkResetFences
    pub unsafe fn reset_fences(
        &self,
        device: Device,
        fence_count: u32,
        fences: *const Fence,
    ) -> Result {
        (self.reset_fences)(device, fence_count, fences)
    }

    /// vkWaitForFences
    pub unsafe fn wait_for_fences(
        &self,
        device: Device,
        fence_count: u32,
        fences: *const Fence,
        wait_all: Bool32,
        timeout: u64,
    ) -> Result {
        (self.wait_for_fences)(device, fence_count, fences, wait_all, timeout)
    }

    /// vkDestroyFence
    pub unsafe fn destroy_fence(
        &self,
        device: Device,
        fence: Fence,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_fence)(device, fence, allocator);
    }

    /// vkCreateSemaphore
    pub unsafe fn create_semaphore(
        &self,
        device: Device,
        create_info: *const SemaphoreCreateInfo,
        allocator: *const AllocationCallbacks,
        semaphore: *mut Semaphore,
    ) -> Result {
        (self.create_semaphore)(device, create_info, allocator, semaphore)
    }

    /// vkGetSemaphoreCounterValue
    /// [v1.2]
    pub unsafe fn get_semaphore_counter_value(
        &self,
        device: Device,
        semaphore: Semaphore,
        value: *mut u64,
    ) -> Result {
        debug_assert!(self.get_semaphore_counter_value.is_some());
        (self.get_semaphore_counter_value.unwrap_unchecked())(device, semaphore, value)
    }

    /// vkWaitSemaphores
    /// [v1.2]
    pub unsafe fn wait_semaphores(
        &self,
        device: Device,
        wait_info: *const SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result {
        debug_assert!(self.wait_semaphores.is_some());
        (self.wait_semaphores.unwrap_unchecked())(device, wait_info, timeout)
    }

    /// vkSignalSemaphore
    /// [v1.2]
    pub unsafe fn signal_semaphore(
        &self,
        device: Device,
        signal_info: *const SemaphoreSignalInfo,
    ) -> Result {
        debug_assert!(self.signal_semaphore.is_some());
        (self.signal_semaphore.unwrap_unchecked())(device, signal_info)
    }

    /// vkDestroySemaphore
    pub unsafe fn destroy_semaphore(
        &self,
        device: Device,
        semaphore: Semaphore,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_semaphore)(device, semaphore, allocator);
    }

    /// vkDeviceWaitIdle
    pub unsafe fn device_wait_idle(&self, device: Device) -> Result {
        (self.device_wait_idle)(device)
    }

    /// vkCreateRenderPass
    pub unsafe fn create_render_pass(
        &self,
        device: Device,
        create_info: *const RenderPassCreateInfo,
        allocator: *const AllocationCallbacks,
        render_pass: *mut RenderPass,
    ) -> Result {
        (self.create_render_pass)(device, create_info, allocator, render_pass)
    }

    /// vkDestroyRenderPass
    pub unsafe fn destroy_render_pass(
        &self,
        device: Device,
        render_pass: RenderPass,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_render_pass)(device, render_pass, allocator);
    }

    /// vkCreateFramebuffer
    pub unsafe fn create_frambuffer(
        &self,
        device: Device,
        create_info: *const FramebufferCreateInfo,
        allocator: *const AllocationCallbacks,
        framebuffer: *mut Framebuffer,
    ) -> Result {
        (self.create_framebuffer)(device, create_info, allocator, framebuffer)
    }

    /// vkDestroyFramebuffer
    pub unsafe fn destroy_framebuffer(
        &self,
        device: Device,
        framebuffer: Framebuffer,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_framebuffer)(device, framebuffer, allocator);
    }

    /// vkCreateShaderModule
    pub unsafe fn create_shader_module(
        &self,
        device: Device,
        create_info: *const ShaderModuleCreateInfo,
        allocator: *const AllocationCallbacks,
        shader_module: *mut ShaderModule,
    ) -> Result {
        (self.create_shader_module)(device, create_info, allocator, shader_module)
    }

    /// vkDestroyShaderModule
    pub unsafe fn destroy_shader_module(
        &self,
        device: Device,
        shader_module: ShaderModule,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_shader_module)(device, shader_module, allocator);
    }

    /// vkCreateComputePipelines
    pub unsafe fn create_compute_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const ComputePipelineCreateInfo,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> Result {
        (self.create_compute_pipelines)(
            device,
            pipeline_cache,
            create_info_count,
            create_infos,
            allocator,
            pipelines,
        )
    }

    /// vkCreateGraphicsPipelines
    pub unsafe fn create_graphics_pipelines(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        create_info_count: u32,
        create_infos: *const GraphicsPipelineCreateInfo,
        allocator: *const AllocationCallbacks,
        pipelines: *mut Pipeline,
    ) -> Result {
        (self.create_graphics_pipelines)(
            device,
            pipeline_cache,
            create_info_count,
            create_infos,
            allocator,
            pipelines,
        )
    }

    /// vkDestroyPipeline
    pub unsafe fn destroy_pipeline(
        &self,
        device: Device,
        pipeline: Pipeline,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_pipeline)(device, pipeline, allocator);
    }

    /// vkCreatePipelineCache
    pub unsafe fn create_pipeline_cache(
        &self,
        device: Device,
        create_info: *const PipelineCacheCreateInfo,
        allocator: *const AllocationCallbacks,
        pipeline_cache: *mut PipelineCache,
    ) -> Result {
        (self.create_pipeline_cache)(device, create_info, allocator, pipeline_cache)
    }

    /// vkMergePipelineCaches
    pub unsafe fn merge_pipeline_caches(
        &self,
        device: Device,
        dst_cache: PipelineCache,
        src_cache_count: u32,
        src_caches: *const PipelineCache,
    ) -> Result {
        (self.merge_pipeline_caches)(device, dst_cache, src_cache_count, src_caches)
    }

    /// vkGetPipelineCacheData
    pub unsafe fn get_pipeline_cache_data(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        data_size: *mut c_size_t,
        data: *mut c_void,
    ) -> Result {
        (self.get_pipeline_cache_data)(device, pipeline_cache, data_size, data)
    }

    /// vkDestroyPipelineCache
    pub unsafe fn destroy_pipeline_cache(
        &self,
        device: Device,
        pipeline_cache: PipelineCache,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_pipeline_cache)(device, pipeline_cache, allocator);
    }

    /// vkAllocateMemory
    pub unsafe fn allocate_memory(
        &self,
        device: Device,
        allocate_info: *const MemoryAllocateInfo,
        allocator: *const AllocationCallbacks,
        memory: *mut DeviceMemory,
    ) -> Result {
        (self.allocate_memory)(device, allocate_info, allocator, memory)
    }

    /// vkFreeMemory
    pub unsafe fn free_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        allocator: *const AllocationCallbacks,
    ) {
        (self.free_memory)(device, memory, allocator);
    }

    /// vkMapMemory
    pub unsafe fn map_memory(
        &self,
        device: Device,
        memory: DeviceMemory,
        offset: u64,
        size: u64,
        flags: MemoryMapFlags,
        data: *mut *mut c_void,
    ) -> Result {
        (self.map_memory)(device, memory, offset, size, flags, data)
    }

    /// vkUnmapMemory
    pub unsafe fn unmap_memory(&self, device: Device, memory: DeviceMemory) {
        (self.unmap_memory)(device, memory);
    }

    /// vkFlushMappedMemoryRanges
    pub unsafe fn flush_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: u32,
        memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        (self.flush_mapped_memory_ranges)(device, memory_range_count, memory_ranges)
    }

    /// vkInvalidateMappedMemoryRanges
    pub unsafe fn invalidate_mapped_memory_ranges(
        &self,
        device: Device,
        memory_range_count: u32,
        memory_ranges: *const MappedMemoryRange,
    ) -> Result {
        (self.invalidate_mapped_memory_ranges)(device, memory_range_count, memory_ranges)
    }

    /// vkCreateBuffer
    pub unsafe fn create_buffer(
        &self,
        device: Device,
        create_info: *const BufferCreateInfo,
        allocator: *const AllocationCallbacks,
        buffer: *mut Buffer,
    ) -> Result {
        (self.create_buffer)(device, create_info, allocator, buffer)
    }

    /// vkDestroyBuffer
    pub unsafe fn destroy_buffer(
        &self,
        device: Device,
        buffer: Buffer,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_buffer)(device, buffer, allocator);
    }

    /// vkGetBufferMemoryRequirements
    pub unsafe fn get_buffer_memory_requirements(
        &self,
        device: Device,
        buffer: Buffer,
        memory_requirements: *mut MemoryRequirements,
    ) {
        (self.get_buffer_memory_requirements)(device, buffer, memory_requirements);
    }

    /// vkBindBufferMemory
    pub unsafe fn bind_buffer_memory(
        &self,
        device: Device,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: u64,
    ) -> Result {
        (self.bind_buffer_memory)(device, buffer, memory, memory_offset)
    }

    /// vkCreateImage
    pub unsafe fn create_image(
        &self,
        device: Device,
        create_info: *const ImageCreateInfo,
        allocator: *const AllocationCallbacks,
        image: *mut Image,
    ) -> Result {
        (self.create_image)(device, create_info, allocator, image)
    }

    /// vkDestroyImage
    pub unsafe fn destroy_image(
        &self,
        device: Device,
        image: Image,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_image)(device, image, allocator);
    }

    /// vkGetImageMemoryRequirements
    pub unsafe fn get_image_memory_requirements(
        &self,
        device: Device,
        image: Image,
        memory_requirements: *mut MemoryRequirements,
    ) {
        (self.get_image_memory_requirements)(device, image, memory_requirements);
    }

    /// vkBindImageMemory
    pub unsafe fn bind_image_memory(
        &self,
        device: Device,
        image: Image,
        memory: DeviceMemory,
        memory_offset: u64,
    ) -> Result {
        (self.bind_image_memory)(device, image, memory, memory_offset)
    }

    /// vkCreateBufferView
    pub unsafe fn create_buffer_view(
        &self,
        device: Device,
        create_info: *const BufferViewCreateInfo,
        allocator: *const AllocationCallbacks,
        view: *mut BufferView,
    ) -> Result {
        (self.create_buffer_view)(device, create_info, allocator, view)
    }

    /// vkDestroyBufferView
    pub unsafe fn destroy_buffer_view(
        &self,
        device: Device,
        view: BufferView,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_buffer_view)(device, view, allocator);
    }

    /// vkCreateImageView
    pub unsafe fn create_image_view(
        &self,
        device: Device,
        create_info: *const ImageViewCreateInfo,
        allocator: *const AllocationCallbacks,
        view: *mut ImageView,
    ) -> Result {
        (self.create_image_view)(device, create_info, allocator, view)
    }

    /// vkDestroyImageView
    pub unsafe fn destroy_image_view(
        &self,
        device: Device,
        view: ImageView,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_image_view)(device, view, allocator);
    }

    /// vkCreateSampler
    pub unsafe fn create_sampler(
        &self,
        device: Device,
        create_info: *const SamplerCreateInfo,
        allocator: *const AllocationCallbacks,
        sampler: *mut Sampler,
    ) -> Result {
        (self.create_sampler)(device, create_info, allocator, sampler)
    }

    /// vkDestroySampler
    pub unsafe fn destroy_sampler(
        &self,
        device: Device,
        sampler: Sampler,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_sampler)(device, sampler, allocator);
    }

    /// vkCreateDescriptorSetLayout
    pub unsafe fn create_descriptor_set_layout(
        &self,
        device: Device,
        create_info: *const DescriptorSetLayoutCreateInfo,
        allocator: *const AllocationCallbacks,
        set_layout: *mut DescriptorSetLayout,
    ) -> Result {
        (self.create_descriptor_set_layout)(device, create_info, allocator, set_layout)
    }

    /// vkDestroyDescriptorSetLayout
    pub unsafe fn destroy_descriptor_set_layout(
        &self,
        device: Device,
        set_layout: DescriptorSetLayout,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_descriptor_set_layout)(device, set_layout, allocator);
    }

    /// vkCreatePipelineLayout
    pub unsafe fn create_pipeline_layout(
        &self,
        device: Device,
        create_info: *const PipelineLayoutCreateInfo,
        allocator: *const AllocationCallbacks,
        pipeline_layout: *mut PipelineLayout,
    ) -> Result {
        (self.create_pipeline_layout)(device, create_info, allocator, pipeline_layout)
    }

    /// vkDestroyPipelineLayout
    pub unsafe fn destroy_pipeline_layout(
        &self,
        device: Device,
        pipeline_layout: PipelineLayout,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_pipeline_layout)(device, pipeline_layout, allocator);
    }

    /// vkCreateDescriptorPool
    pub unsafe fn create_descriptor_pool(
        &self,
        device: Device,
        create_info: *const DescriptorPoolCreateInfo,
        allocator: *const AllocationCallbacks,
        descriptor_pool: *mut DescriptorPool,
    ) -> Result {
        (self.create_descriptor_pool)(device, create_info, allocator, descriptor_pool)
    }

    /// vkResetDescriptorPool
    pub unsafe fn reset_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result {
        (self.reset_descriptor_pool)(device, descriptor_pool, flags)
    }

    /// vkDestroyDescriptorPool
    pub unsafe fn destroy_descriptor_pool(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_descriptor_pool)(device, descriptor_pool, allocator);
    }

    /// vkAllocateDescriptorSets
    pub unsafe fn allocate_descriptor_sets(
        &self,
        device: Device,
        allocate_info: *const DescriptorSetAllocateInfo,
        descriptor_sets: *mut DescriptorSet,
    ) -> Result {
        (self.allocate_descriptor_sets)(device, allocate_info, descriptor_sets)
    }

    /// vkUpdateDescriptorSets
    pub unsafe fn update_descriptor_sets(
        &self,
        device: Device,
        descriptor_write_count: u32,
        descriptor_writes: *const WriteDescriptorSet,
        descriptor_copy_count: u32,
        descriptor_copies: *const CopyDescriptorSet,
    ) {
        (self.update_descriptor_sets)(
            device,
            descriptor_write_count,
            descriptor_writes,
            descriptor_copy_count,
            descriptor_copies,
        );
    }

    /// vkFreeDescriptorSets
    pub unsafe fn free_descriptor_sets(
        &self,
        device: Device,
        descriptor_pool: DescriptorPool,
        descriptor_set_count: u32,
        descriptor_sets: *const DescriptorSet,
    ) -> Result {
        (self.free_descriptor_sets)(
            device,
            descriptor_pool,
            descriptor_set_count,
            descriptor_sets,
        )
    }

    /// vkCreateQueryPool
    pub unsafe fn create_query_pool(
        &self,
        device: Device,
        create_info: *const QueryPoolCreateInfo,
        allocator: *const AllocationCallbacks,
        query_pool: *mut QueryPool,
    ) -> Result {
        (self.create_query_pool)(device, create_info, allocator, query_pool)
    }

    /// vkGetQueryPoolResults
    pub unsafe fn query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data_size: c_size_t,
        data: *mut c_void,
        stride: u64,
        flags: QueryResultFlags,
    ) -> Result {
        (self.get_query_pool_results)(
            device,
            query_pool,
            first_query,
            query_count,
            data_size,
            data,
            stride,
            flags,
        )
    }

    /// vkDestroyQueryPool
    pub unsafe fn destroy_query_pool(
        &self,
        device: Device,
        query_pool: QueryPool,
        allocator: *const AllocationCallbacks,
    ) {
        (self.destroy_query_pool)(device, query_pool, allocator);
    }
}

impl DeviceFp {
    /// vkQueueSubmit
    pub unsafe fn queue_submit(
        &self,
        queue: Queue,
        submit_count: u32,
        submits: *const SubmitInfo,
        fence: Fence,
    ) -> Result {
        (self.queue_submit)(queue, submit_count, submits, fence)
    }

    /// vkQueueSubmit2
    /// [v1.3]
    pub unsafe fn queue_submit_2(
        &self,
        queue: Queue,
        submit_count: u32,
        submits: *const SubmitInfo2,
        fence: Fence,
    ) -> Result {
        debug_assert!(self.queue_submit_2.is_some());
        (self.queue_submit_2.unwrap_unchecked())(queue, submit_count, submits, fence)
    }

    /// vkQueueWaitIdle
    pub unsafe fn queue_wait_idle(&self, queue: Queue) -> Result {
        (self.queue_wait_idle)(queue)
    }
}

impl DeviceFp {
    /// vkResetCommandBuffer
    pub unsafe fn reset_command_buffer(
        &self,
        command_buffer: CommandBuffer,
        flags: CommandBufferResetFlags,
    ) -> Result {
        (self.reset_command_buffer)(command_buffer, flags)
    }
}
