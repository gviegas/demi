// TODO:
// - multiple frames
// - swapchain out of date
// - win32

use std::ffi::c_char;
use std::mem;
use std::ptr;
use std::time::{Duration, Instant};

use vk_sys::{
    ApplicationInfo, AttachmentDescription, AttachmentReference, Buffer, BufferCreateInfo,
    ClearColorValue, ClearValue, CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo,
    CommandPool, CommandPoolCreateInfo, ComponentMapping, CompositeAlphaFlagBitsKhr,
    DescriptorBufferInfo, DescriptorPool, DescriptorPoolCreateInfo, DescriptorPoolSize,
    DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding,
    DescriptorSetLayoutCreateInfo, Device, DeviceCreateInfo, DeviceFp, DeviceMemory,
    DeviceQueueCreateInfo, Extent2d, Fence, FenceCreateInfo, Format, Framebuffer,
    FramebufferCreateInfo, GraphicsPipelineCreateInfo, ImageSubresourceRange, ImageView,
    ImageViewCreateInfo, Instance, InstanceCreateInfo, InstanceFp, MemoryAllocateInfo,
    MemoryRequirements, Offset2d, PhysicalDevice, PhysicalDeviceProperties, Pipeline,
    PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo,
    PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo,
    PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo,
    PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo,
    PipelineViewportStateCreateInfo, PresentInfoKhr, Queue, Rect2d, RenderPass,
    RenderPassBeginInfo, RenderPassCreateInfo, Semaphore, SemaphoreCreateInfo, ShaderModule,
    ShaderModuleCreateInfo, SubmitInfo, SubpassDescription, SurfaceCapabilitiesKhr, SurfaceKhr,
    SwapchainCreateInfoKhr, SwapchainKhr, VertexInputAttributeDescription,
    VertexInputBindingDescription, Viewport, WriteDescriptorSet,
};

#[cfg(target_os = "linux")]
use vk_sys::WaylandSurfaceCreateInfoKhr;

#[cfg(windows)]
use vk_sys::Win32SurfaceCreateInfoKhr;

// Draws a triangle and presents the result.
#[test]
fn test_draw() {
    init_global();

    let mut state = State::new();

    const TIMEOUT: Duration = Duration::new(60, 0);
    let tm = Instant::now();

    while tm.elapsed() < TIMEOUT && !plat::quit() {
        plat::poll();
        state.render();
    }
}

// Initializes the library and get global procs.
fn init_global() {
    vk_sys::init().unwrap();
}

// Render state.
#[derive(Debug)]
struct State {
    inst: InstState,
    dev: DevState,
    cmd: CmdState,
    sc: ScState,
    pass: PassState,
    buf: BufState,
    desc: DescState,
    shd: ShdState,
    pl: PlState,
}

impl State {
    // Intializes the render state.
    fn new() -> Self {
        let inst = InstState::new();
        println!("{inst:#?}");

        let dev = DevState::new(&inst);
        println!("{dev:#?}");

        let cmd = CmdState::new(&dev);
        println!("{cmd:#?}");

        let sc = ScState::new(&inst, &dev);
        println!("{sc:#?}");

        let pass = PassState::new(&dev, &sc);
        println!("{pass:#?}");

        let buf = BufState::new(&inst, &dev, &sc);
        println!("{buf:#?}");

        let desc = DescState::new(&dev, &buf);
        println!("{desc:#?}");

        let shd = ShdState::new(&dev);
        println!("{shd:#?}");

        let pl = PlState::new(&dev, &sc, &pass, &desc, &shd);
        println!("{pl:#?}");

        Self {
            inst,
            dev,
            cmd,
            sc,
            pass,
            buf,
            desc,
            shd,
            pl,
        }
    }
}

impl State {
    // Renders a frame and presents it.
    fn render(&mut self) {
        unsafe {
            match self.dev.fp.wait_for_fences(
                self.dev.dev,
                1,
                &self.cmd.wait_fence,
                vk_sys::TRUE,
                u64::MAX,
            ) {
                vk_sys::SUCCESS => assert_eq!(
                    self.dev
                        .fp
                        .reset_fences(self.dev.dev, 1, &self.cmd.wait_fence),
                    vk_sys::SUCCESS
                ),
                other => panic!("wait_for_fences failed ({})", other),
            }

            let mut sc_idx = u32::MAX;
            match self.dev.fp.acquire_next_image_khr(
                self.dev.dev,
                self.sc.sc,
                1_000_000_000,
                self.cmd.wait_sem,
                vk_sys::null_handle(),
                &mut sc_idx,
            ) {
                vk_sys::SUCCESS => (),
                other => panic!("acquire_next_image_khr failed ({})", other),
            }

            let cmd_begin = CommandBufferBeginInfo {
                s_type: vk_sys::STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO,
                next: ptr::null(),
                flags: vk_sys::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
                inheritance_info: ptr::null(),
            };
            assert_eq!(
                self.dev
                    .fp
                    .begin_command_buffer(self.cmd.cmd_buf, &cmd_begin),
                vk_sys::SUCCESS
            );

            self.dev.fp.cmd_bind_pipeline(
                self.cmd.cmd_buf,
                vk_sys::PIPELINE_BIND_POINT_GRAPHICS,
                self.pl.pl,
            );

            self.dev.fp.cmd_bind_descriptor_sets(
                self.cmd.cmd_buf,
                vk_sys::PIPELINE_BIND_POINT_GRAPHICS,
                self.pl.layout,
                0,
                1,
                &self.desc.desc_set,
                0,
                ptr::null(),
            );

            let vert_bufs = [self.buf.buf, self.buf.buf];
            let vert_offs = [0, mem::size_of_val(&POSITIONS) as u64];
            self.dev.fp.cmd_bind_vertex_buffers(
                self.cmd.cmd_buf,
                0,
                2,
                vert_bufs.as_ptr(),
                vert_offs.as_ptr(),
            );

            let pass_begin = RenderPassBeginInfo {
                s_type: vk_sys::STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO,
                next: ptr::null(),
                render_pass: self.pass.pass,
                framebuffer: self.pass.fbs[sc_idx as usize],
                render_area: Rect2d {
                    offset: Offset2d { x: 0, y: 0 },
                    extent: self.sc.extent,
                },
                clear_value_count: 1,
                clear_values: &ClearValue {
                    color: ClearColorValue { float32: [0.01; 4] },
                },
            };
            self.dev.fp.cmd_begin_render_pass(
                self.cmd.cmd_buf,
                &pass_begin,
                vk_sys::SUBPASS_CONTENTS_INLINE,
            );

            self.dev.fp.cmd_draw(self.cmd.cmd_buf, 3, 1, 0, 0);

            self.dev.fp.cmd_end_render_pass(self.cmd.cmd_buf);

            assert_eq!(
                self.dev.fp.end_command_buffer(self.cmd.cmd_buf),
                vk_sys::SUCCESS
            );

            let submit = SubmitInfo {
                s_type: vk_sys::STRUCTURE_TYPE_SUBMIT_INFO,
                next: ptr::null(),
                wait_semaphore_count: 1,
                wait_semaphores: &self.cmd.wait_sem,
                wait_dst_stage_mask: &vk_sys::PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT,
                command_buffer_count: 1,
                command_buffers: &self.cmd.cmd_buf,
                signal_semaphore_count: 0,
                signal_semaphores: ptr::null(),
            };

            assert_eq!(
                self.dev.fp.queue_submit(
                    self.dev.queues[self.dev.rend_fam as usize],
                    1,
                    &submit,
                    self.cmd.wait_fence,
                ),
                vk_sys::SUCCESS
            );

            let present = PresentInfoKhr {
                s_type: vk_sys::STRUCTURE_TYPE_PRESENT_INFO_KHR,
                next: ptr::null(),
                wait_semaphore_count: 0,
                wait_semaphores: ptr::null(),
                swapchain_count: 1,
                swapchains: &self.sc.sc,
                image_indices: &sc_idx,
                results: ptr::null_mut(),
            };

            assert_eq!(
                self.dev
                    .fp
                    .queue_present_khr(self.dev.queues[self.sc.pres_fam as usize], &present),
                vk_sys::SUCCESS
            );
        }
    }
}

impl Drop for State {
    fn drop(&mut self) {
        // Ensure correct ordering of destruction.
        println!("device_wait_idle()");
        unsafe {
            self.dev.fp.device_wait_idle(self.dev.dev);
        }
        self.cmd.destroy(&self.dev);
        self.sc.destroy(&self.inst, &self.dev);
        self.pass.destroy(&self.dev);
        self.buf.destroy(&self.dev);
        self.desc.destroy(&self.dev);
        self.shd.destroy(&self.dev);
        self.pl.destroy(&self.dev);
        self.dev.destroy();
        self.inst.destroy();
        plat::fini();
        vk_sys::fini();
    }
}

// Instance state.
#[derive(Debug)]
struct InstState {
    inst: Instance,
    fp: InstanceFp,
}

impl InstState {
    // Creates the instance and gets instance-level procs.
    fn new() -> Self {
        let app_info = ApplicationInfo {
            s_type: vk_sys::STRUCTURE_TYPE_APPLICATION_INFO,
            next: ptr::null(),
            application_name: ptr::null(),
            application_version: 0,
            engine_name: ptr::null(),
            engine_version: 0,
            api_version: vk_sys::API_VERSION_1_3,
        };

        let exts: [*const c_char; 2] = if cfg!(target_os = "linux") {
            [
                c"VK_KHR_surface".as_ptr(),
                c"VK_KHR_wayland_surface".as_ptr(),
            ]
        } else if cfg!(windows) {
            [c"VK_KHR_surface".as_ptr(), c"VK_KHR_win32_surface".as_ptr()]
        } else {
            panic!("invalid OS");
        };

        let inst_info = InstanceCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            application_info: &app_info,
            enabled_layer_count: 0,
            enabled_layer_names: ptr::null(),
            enabled_extension_count: exts.len() as u32,
            enabled_extension_names: exts.as_ptr(),
        };

        let mut inst = ptr::null_mut();
        assert_eq!(
            unsafe { vk_sys::create_instance(&inst_info, ptr::null(), &mut inst) },
            vk_sys::SUCCESS
        );

        let fp = unsafe { InstanceFp::new(inst).unwrap() };

        Self { inst, fp }
    }

    fn destroy(&mut self) {
        unsafe {
            println!("destroy_instance()");
            self.fp.destroy_instance(self.inst, ptr::null());
        }
    }
}

// Device state.
#[derive(Debug)]
struct DevState {
    phys_dev: PhysicalDevice,
    dev: Device,
    fp: DeviceFp,
    rend_fam: u32,
    queues: Vec<Queue>,
}

impl DevState {
    // Selects a physical device, creates a logical device and
    // gets device-level procs.
    fn new(inst_state: &InstState) -> Self {
        let mut dev_count = 0u32;
        let mut phys_devs;
        unsafe {
            assert_eq!(
                inst_state.fp.enumerate_physical_devices(
                    inst_state.inst,
                    &mut dev_count,
                    ptr::null_mut()
                ),
                vk_sys::SUCCESS
            );
            assert!(dev_count > 0);
            phys_devs = Vec::with_capacity(dev_count as usize);
            assert_eq!(
                inst_state.fp.enumerate_physical_devices(
                    inst_state.inst,
                    &mut dev_count,
                    phys_devs.as_mut_ptr()
                ),
                vk_sys::SUCCESS
            );
            phys_devs.set_len(dev_count as usize);
        }

        let mut dev_props = Vec::with_capacity(dev_count as usize);
        let mut dev_idx = 0;
        for i in &phys_devs {
            let mut props: PhysicalDeviceProperties = unsafe { mem::zeroed() };
            unsafe {
                inst_state.fp.get_physical_device_properties(*i, &mut props);
            }
            match props.device_type {
                vk_sys::PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                | vk_sys::PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU => dev_idx = dev_props.len(),
                _ => (),
            }
            dev_props.push(props);
        }

        let mut fam_count = 032;
        let mut que_props;
        unsafe {
            inst_state.fp.get_physical_device_queue_family_properties(
                phys_devs[dev_idx],
                &mut fam_count,
                ptr::null_mut(),
            );
            assert!(fam_count > 0);
            que_props = Vec::with_capacity(fam_count as usize);
            inst_state.fp.get_physical_device_queue_family_properties(
                phys_devs[dev_idx],
                &mut fam_count,
                que_props.as_mut_ptr(),
            );
            que_props.set_len(fam_count as usize);
        }

        let mut que_info = Vec::with_capacity(fam_count as usize);
        for i in 0..fam_count {
            que_info.push(DeviceQueueCreateInfo {
                s_type: vk_sys::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
                next: ptr::null(),
                flags: 0,
                queue_family_index: i,
                queue_count: 1,
                queue_priorities: &1f32,
            });
        }

        const EXTS: [*const c_char; 1] = [c"VK_KHR_swapchain".as_ptr()];

        let dev_info = DeviceCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            queue_create_info_count: fam_count,
            queue_create_infos: que_info.as_ptr(),
            enabled_layer_count: 0,
            enabled_layer_names: ptr::null(),
            enabled_extension_count: EXTS.len() as u32,
            enabled_extension_names: EXTS.as_ptr(),
            enabled_features: ptr::null(),
        };

        let mut dev = ptr::null_mut();
        assert_eq!(
            unsafe {
                inst_state
                    .fp
                    .create_device(phys_devs[dev_idx], &dev_info, ptr::null(), &mut dev)
            },
            vk_sys::SUCCESS
        );

        let fp = unsafe { DeviceFp::new(dev, &inst_state.fp).unwrap() };

        let mut rend_fam = 0u32;
        let mut queues = Vec::with_capacity(fam_count as usize);
        for i in que_props.iter().enumerate() {
            let mut queue = ptr::null_mut();
            unsafe {
                fp.get_device_queue(dev, i.0 as u32, 0, &mut queue);
            }
            if i.1.queue_flags & vk_sys::QUEUE_GRAPHICS_BIT == 0 {
                rend_fam = i.0 as u32;
            }
            queues.push(queue);
        }

        Self {
            phys_dev: phys_devs[dev_idx],
            dev,
            fp,
            rend_fam,
            queues,
        }
    }

    fn destroy(&mut self) {
        unsafe {
            println!("destroy_device()");
            self.fp.destroy_device(self.dev, ptr::null());
        }
    }
}

// Command buffer state.
#[derive(Debug)]
struct CmdState {
    cmd_pool: CommandPool,
    cmd_buf: CommandBuffer,
    wait_sem: Semaphore,
    wait_fence: Fence,
}

impl CmdState {
    // Creates a command pool, the semaphore/fence to wait on,
    // and allocates a command buffer from the pool.
    fn new(dev_state: &DevState) -> Self {
        let pool_info = CommandPoolCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO,
            next: ptr::null(),
            flags: vk_sys::COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT,
            queue_family_index: dev_state.rend_fam,
        };
        let mut cmd_pool = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state.fp.create_command_pool(
                    dev_state.dev,
                    &pool_info,
                    ptr::null(),
                    &mut cmd_pool,
                )
            },
            vk_sys::SUCCESS
        );

        let buf_info = CommandBufferAllocateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO,
            next: ptr::null(),
            command_pool: cmd_pool,
            level: vk_sys::COMMAND_BUFFER_LEVEL_PRIMARY,
            command_buffer_count: 1,
        };
        let mut cmd_buf = ptr::null_mut();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .allocate_command_buffers(dev_state.dev, &buf_info, &mut cmd_buf)
            },
            vk_sys::SUCCESS
        );

        let sem_info = SemaphoreCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
        };
        let mut wait_sem = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_semaphore(dev_state.dev, &sem_info, ptr::null(), &mut wait_sem)
            },
            vk_sys::SUCCESS
        );

        let fence_info = FenceCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_FENCE_CREATE_INFO,
            next: ptr::null(),
            flags: vk_sys::FENCE_CREATE_SIGNALED_BIT,
        };
        let mut wait_fence = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_fence(dev_state.dev, &fence_info, ptr::null(), &mut wait_fence)
            },
            vk_sys::SUCCESS
        );

        Self {
            cmd_pool,
            cmd_buf,
            wait_sem,
            wait_fence,
        }
    }

    fn destroy(&mut self, dev_state: &DevState) {
        unsafe {
            println!("destroy_command_pool()");
            dev_state
                .fp
                .destroy_command_pool(dev_state.dev, self.cmd_pool, ptr::null());
            println!("destroy_semaphore()");
            dev_state
                .fp
                .destroy_semaphore(dev_state.dev, self.wait_sem, ptr::null());
            println!("destroy_fence()");
            dev_state
                .fp
                .destroy_fence(dev_state.dev, self.wait_fence, ptr::null());
        }
    }
}

// Swapchain state.
#[derive(Debug)]
struct ScState {
    pres_fam: u32,
    sf: SurfaceKhr,
    sc: SwapchainKhr,
    sc_fmt: Format,
    extent: Extent2d,
    views: Vec<ImageView>,
}

impl ScState {
    // Selects the presentation queue family and crates a surface,
    // swapchain and image views.
    fn new(inst_state: &InstState, dev_state: &DevState) -> Self {
        let sf = Self::new_sf(inst_state);

        let mut pres_fam = dev_state.rend_fam;
        loop {
            let mut supported = vk_sys::FALSE;
            assert_eq!(
                unsafe {
                    inst_state.fp.get_physical_device_surface_support_khr(
                        dev_state.phys_dev,
                        pres_fam,
                        sf,
                        &mut supported,
                    )
                },
                vk_sys::SUCCESS
            );
            if supported == vk_sys::TRUE {
                break;
            }
            pres_fam = (pres_fam + 1) % dev_state.queues.len() as u32;
            assert_ne!(pres_fam, dev_state.rend_fam);
        }

        let mut capab: SurfaceCapabilitiesKhr = unsafe { mem::zeroed() };
        assert_eq!(
            unsafe {
                inst_state.fp.get_physical_device_surface_capabilities_khr(
                    dev_state.phys_dev,
                    sf,
                    &mut capab,
                )
            },
            vk_sys::SUCCESS
        );

        let mut fmt_count = 0u32;
        assert_eq!(
            unsafe {
                inst_state.fp.get_physical_device_surface_formats_khr(
                    dev_state.phys_dev,
                    sf,
                    &mut fmt_count,
                    ptr::null_mut(),
                )
            },
            vk_sys::SUCCESS
        );
        let mut fmts = Vec::with_capacity(fmt_count as usize);
        assert_eq!(
            unsafe {
                inst_state.fp.get_physical_device_surface_formats_khr(
                    dev_state.phys_dev,
                    sf,
                    &mut fmt_count,
                    fmts.as_mut_ptr(),
                )
            },
            vk_sys::SUCCESS
        );
        unsafe {
            fmts.set_len(fmt_count as usize);
        }
        // TODO: Choose a format.
        assert!(fmts.len() > 0);
        let fmt_idx = 0;

        let extent = Extent2d {
            width: 640,
            height: 384,
        };

        let (shar_mode, queue_fams) = if pres_fam == dev_state.rend_fam {
            (vk_sys::SHARING_MODE_EXCLUSIVE, vec![])
        } else {
            (
                vk_sys::SHARING_MODE_CONCURRENT,
                vec![pres_fam, dev_state.rend_fam],
            )
        };

        let mut comp_alpha: CompositeAlphaFlagBitsKhr = 1;
        assert_ne!(capab.supported_composite_alpha, 0);
        while comp_alpha & capab.supported_composite_alpha == 0 {
            comp_alpha <<= 1;
        }

        let sc_info = SwapchainCreateInfoKhr {
            s_type: vk_sys::STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR,
            next: ptr::null(),
            flags: 0,
            surface: sf,
            min_image_count: capab.min_image_count,
            image_format: fmts[fmt_idx].format,
            image_color_space: fmts[fmt_idx].color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk_sys::IMAGE_USAGE_COLOR_ATTACHMENT_BIT,
            image_sharing_mode: shar_mode,
            queue_family_index_count: queue_fams.len() as u32,
            queue_family_indices: queue_fams.as_ptr(),
            pre_transform: vk_sys::SURFACE_TRANSFORM_IDENTITY_BIT_KHR,
            composite_alpha: comp_alpha,
            present_mode: vk_sys::PRESENT_MODE_FIFO_KHR,
            clipped: vk_sys::TRUE,
            old_swapchain: vk_sys::null_handle(),
        };

        let mut sc = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_swapchain_khr(dev_state.dev, &sc_info, ptr::null(), &mut sc)
            },
            vk_sys::SUCCESS
        );

        let mut count = 0u32;
        assert_eq!(
            unsafe {
                dev_state.fp.get_swapchain_images_khr(
                    dev_state.dev,
                    sc,
                    &mut count,
                    ptr::null_mut(),
                )
            },
            vk_sys::SUCCESS
        );
        let mut images = Vec::with_capacity(count as usize);
        assert_eq!(
            unsafe {
                dev_state.fp.get_swapchain_images_khr(
                    dev_state.dev,
                    sc,
                    &mut count,
                    images.as_mut_ptr(),
                )
            },
            vk_sys::SUCCESS
        );
        unsafe {
            images.set_len(count as usize);
        }

        let mut views = Vec::with_capacity(count as usize);
        for i in &images {
            let view_info = ImageViewCreateInfo {
                s_type: vk_sys::STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO,
                next: ptr::null(),
                flags: 0,
                image: *i,
                view_type: vk_sys::IMAGE_VIEW_TYPE_2D,
                format: sc_info.image_format,
                components: ComponentMapping {
                    r: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
                    g: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
                    b: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
                    a: vk_sys::COMPONENT_SWIZZLE_IDENTITY,
                },
                subresource_range: ImageSubresourceRange {
                    aspect_mask: vk_sys::IMAGE_ASPECT_COLOR_BIT,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
            };

            let mut view = vk_sys::null_handle();
            assert_eq!(
                unsafe {
                    dev_state.fp.create_image_view(
                        dev_state.dev,
                        &view_info,
                        ptr::null(),
                        &mut view,
                    )
                },
                vk_sys::SUCCESS
            );

            views.push(view);
        }

        Self {
            pres_fam,
            sf,
            sc,
            sc_fmt: sc_info.image_format,
            extent,
            views,
        }
    }

    #[cfg(target_os = "linux")]
    fn new_sf(inst_state: &InstState) -> SurfaceKhr {
        plat::init();

        let sf_info = WaylandSurfaceCreateInfoKhr {
            s_type: vk_sys::STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR,
            next: ptr::null(),
            flags: 0,
            display: unsafe { plat::DISPLAY },
            surface: unsafe { plat::SURFACE },
        };
        let mut sf = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                inst_state.fp.create_wayland_surface_khr(
                    inst_state.inst,
                    &sf_info,
                    ptr::null(),
                    &mut sf,
                )
            },
            vk_sys::SUCCESS
        );

        sf
    }

    #[cfg(windows)]
    fn new_sf(inst_state: &InstState) -> SurfaceKhr {
        todo!();
    }

    fn destroy(&mut self, inst_state: &InstState, dev_state: &DevState) {
        unsafe {
            for i in self.views.iter().enumerate() {
                println!("destroy_image_view() [{}]", i.0);
                dev_state
                    .fp
                    .destroy_image_view(dev_state.dev, *i.1, ptr::null());
            }
            println!("destroy_swapchain_khr()");
            dev_state
                .fp
                .destroy_swapchain_khr(dev_state.dev, self.sc, ptr::null());
            println!("destroy_surface_khr()");
            inst_state
                .fp
                .destroy_surface_khr(inst_state.inst, self.sf, ptr::null());
        }
    }
}

// Render pass state.
#[derive(Debug)]
struct PassState {
    pass: RenderPass,
    fbs: Vec<Framebuffer>,
}

impl PassState {
    // Creates a render pass and one framebuffer for each
    // swapchain image.
    fn new(dev_state: &DevState, sc_state: &ScState) -> Self {
        let attach = AttachmentDescription {
            flags: 0,
            format: sc_state.sc_fmt,
            samples: vk_sys::SAMPLE_COUNT_1_BIT,
            load_op: vk_sys::ATTACHMENT_LOAD_OP_CLEAR,
            store_op: vk_sys::ATTACHMENT_STORE_OP_STORE,
            stencil_load_op: vk_sys::ATTACHMENT_LOAD_OP_DONT_CARE,
            stencil_store_op: vk_sys::ATTACHMENT_STORE_OP_DONT_CARE,
            initial_layout: vk_sys::IMAGE_LAYOUT_UNDEFINED,
            final_layout: vk_sys::IMAGE_LAYOUT_PRESENT_SRC_KHR,
        };

        let color_ref = AttachmentReference {
            attachment: 0,
            layout: vk_sys::IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL,
        };

        let subp = SubpassDescription {
            flags: 0,
            pipeline_bind_point: vk_sys::PIPELINE_BIND_POINT_GRAPHICS,
            input_attachment_count: 0,
            input_attachments: ptr::null(),
            color_attachment_count: 1,
            color_attachments: &color_ref,
            resolve_attachments: ptr::null(),
            depth_stencil_attachment: ptr::null(),
            preserve_attachment_count: 0,
            preserve_attachments: ptr::null(),
        };

        let pass_info = RenderPassCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            attachment_count: 1,
            attachments: &attach,
            subpass_count: 1,
            subpasses: &subp,
            dependency_count: 0,
            dependencies: ptr::null(),
        };

        let mut pass = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_render_pass(dev_state.dev, &pass_info, ptr::null(), &mut pass)
            },
            vk_sys::SUCCESS
        );

        let mut fbs = Vec::with_capacity(sc_state.views.len());
        for i in &sc_state.views {
            let fb_info = FramebufferCreateInfo {
                s_type: vk_sys::STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO,
                next: ptr::null(),
                flags: 0,
                render_pass: pass,
                attachment_count: 1,
                attachments: i,
                width: sc_state.extent.width,
                height: sc_state.extent.height,
                layers: 1,
            };

            let mut fb = vk_sys::null_handle();
            assert_eq!(
                unsafe {
                    dev_state
                        .fp
                        .create_framebuffer(dev_state.dev, &fb_info, ptr::null(), &mut fb)
                },
                vk_sys::SUCCESS
            );

            fbs.push(fb);
        }

        Self { pass, fbs }
    }

    fn destroy(&mut self, dev_state: &DevState) {
        unsafe {
            println!("destroy_render_pass()");
            dev_state
                .fp
                .destroy_render_pass(dev_state.dev, self.pass, ptr::null());
            for i in self.fbs.iter().enumerate() {
                println!("destroy_framebuffer() [{}]", i.0);
                dev_state
                    .fp
                    .destroy_framebuffer(dev_state.dev, *i.1, ptr::null());
            }
        }
    }
}

// Buffer state.
#[derive(Debug)]
struct BufState {
    buf: Buffer,
    mem: DeviceMemory,
}

impl BufState {
    // Creates a buffer, allocates memory, copies vertex/uniform data
    // and binds buffer memory.
    fn new(inst_state: &InstState, dev_state: &DevState, sc_state: &ScState) -> Self {
        let buf_info = BufferCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            size: 4096,
            usage: vk_sys::BUFFER_USAGE_UNIFORM_BUFFER_BIT | vk_sys::BUFFER_USAGE_VERTEX_BUFFER_BIT,
            sharing_mode: vk_sys::SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            queue_family_indices: ptr::null(),
        };

        let mut buf = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_buffer(dev_state.dev, &buf_info, ptr::null(), &mut buf)
            },
            vk_sys::SUCCESS
        );

        let mut mem_reqs: MemoryRequirements = unsafe { mem::zeroed() };
        unsafe {
            dev_state
                .fp
                .get_buffer_memory_requirements(dev_state.dev, buf, &mut mem_reqs);
        }

        let mut mem_props = unsafe { mem::zeroed() };
        unsafe {
            inst_state
                .fp
                .get_physical_device_memory_properties(dev_state.phys_dev, &mut mem_props);
        }

        const MEM_FLAGS: vk_sys::MemoryPropertyFlags =
            vk_sys::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk_sys::MEMORY_PROPERTY_HOST_COHERENT_BIT;
        let mut mem_type = u32::MAX;
        for i in 0..mem_props.memory_type_count {
            if 1 << i & mem_reqs.memory_type_bits == 0 {
                continue;
            }
            if mem_props.memory_types[i as usize].property_flags & MEM_FLAGS == MEM_FLAGS {
                mem_type = i;
                break;
            }
        }
        assert_ne!(mem_type, u32::MAX);

        let alloc_info = MemoryAllocateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO,
            next: ptr::null(),
            allocation_size: mem_reqs.size,
            memory_type_index: mem_type,
        };

        let mut mem = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .allocate_memory(dev_state.dev, &alloc_info, ptr::null(), &mut mem)
            },
            vk_sys::SUCCESS
        );

        unsafe {
            let mut data = ptr::null_mut();
            assert_eq!(
                dev_state
                    .fp
                    .map_memory(dev_state.dev, mem, 0, vk_sys::WHOLE_SIZE, 0, &mut data),
                vk_sys::SUCCESS
            );
            ptr::copy_nonoverlapping(
                POSITIONS.as_ptr().cast(),
                data,
                mem::size_of_val(&POSITIONS),
            );
            ptr::copy_nonoverlapping(
                COLORS.as_ptr().cast(),
                data.add(mem::size_of_val(&POSITIONS)),
                mem::size_of_val(&COLORS),
            );
            let mut transform = [0f32; 4 * 4];
            transform[0] = 0.75;
            transform[5] = 0.75;
            transform[10] = 1.0;
            transform[15] = 1.0;
            match sc_state.extent.width as f32 / sc_state.extent.height as f32 {
                x if x > 1.0 => transform[0] /= x,
                x => transform[5] *= x,
            }
            ptr::copy_nonoverlapping(
                transform.as_ptr().cast(),
                data.offset(1024),
                mem::size_of_val(&transform),
            );
            dev_state.fp.unmap_memory(dev_state.dev, mem);
        }

        assert_eq!(
            unsafe { dev_state.fp.bind_buffer_memory(dev_state.dev, buf, mem, 0) },
            vk_sys::SUCCESS,
        );

        Self { buf, mem }
    }

    fn destroy(&mut self, dev_state: &DevState) {
        unsafe {
            println!("destroy_buffer()");
            dev_state
                .fp
                .destroy_buffer(dev_state.dev, self.buf, ptr::null());
            println!("free_memory()");
            dev_state
                .fp
                .free_memory(dev_state.dev, self.mem, ptr::null());
        }
    }
}

// Descriptor set state.
#[derive(Debug)]
struct DescState {
    set_layout: DescriptorSetLayout,
    desc_pool: DescriptorPool,
    desc_set: DescriptorSet,
}

impl DescState {
    // Creates a descriptor set layout, a descriptor pool, allocates
    // a descriptor set and updates it with uniform buffer.
    fn new(dev_state: &DevState, buf_state: &BufState) -> Self {
        let layout_info = DescriptorSetLayoutCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            binding_count: 1,
            bindings: &DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk_sys::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptor_count: 1,
                stage_flags: vk_sys::SHADER_STAGE_VERTEX_BIT,
                immutable_samplers: ptr::null(),
            },
        };

        let mut set_layout = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state.fp.create_descriptor_set_layout(
                    dev_state.dev,
                    &layout_info,
                    ptr::null(),
                    &mut set_layout,
                )
            },
            vk_sys::SUCCESS
        );

        let pool_info = DescriptorPoolCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            max_sets: 1,
            pool_size_count: 1,
            pool_sizes: &DescriptorPoolSize {
                descriptor_type: vk_sys::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
                descriptor_count: 1,
            },
        };

        let mut desc_pool = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state.fp.create_descriptor_pool(
                    dev_state.dev,
                    &pool_info,
                    ptr::null(),
                    &mut desc_pool,
                )
            },
            vk_sys::SUCCESS
        );

        let alloc_info = DescriptorSetAllocateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO,
            next: ptr::null(),
            descriptor_pool: desc_pool,
            descriptor_set_count: 1,
            set_layouts: &set_layout,
        };

        let mut desc_set = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .allocate_descriptor_sets(dev_state.dev, &alloc_info, &mut desc_set)
            },
            vk_sys::SUCCESS
        );

        let write = WriteDescriptorSet {
            s_type: vk_sys::STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET,
            next: ptr::null(),
            dst_set: desc_set,
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk_sys::DESCRIPTOR_TYPE_UNIFORM_BUFFER,
            image_infos: ptr::null(),
            buffer_infos: &DescriptorBufferInfo {
                buffer: buf_state.buf,
                offset: 1024,
                range: mem::size_of::<[f32; 16]>() as u64,
            },
            texel_buffer_views: ptr::null(),
        };

        unsafe {
            dev_state
                .fp
                .update_descriptor_sets(dev_state.dev, 1, &write, 0, ptr::null());
        }

        Self {
            set_layout,
            desc_pool,
            desc_set,
        }
    }

    fn destroy(&mut self, dev_state: &DevState) {
        unsafe {
            println!("destroy_descriptor_set_layout()");
            dev_state
                .fp
                .destroy_descriptor_set_layout(dev_state.dev, self.set_layout, ptr::null());
            println!("destroy_descriptor_pool()");
            dev_state
                .fp
                .destroy_descriptor_pool(dev_state.dev, self.desc_pool, ptr::null());
        }
    }
}

// Shader module state.
#[derive(Debug)]
struct ShdState {
    vert: ShaderModule,
    frag: ShaderModule,
}

impl ShdState {
    // Creates vertex and fragment shader modules.
    fn new(dev_state: &DevState) -> Self {
        let mut info = ShaderModuleCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            code_size: 0,
            code: ptr::null(),
        };

        let mut vert = vk_sys::null_handle();
        info.code_size = VS.len() * 4;
        info.code = VS.as_ptr();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_shader_module(dev_state.dev, &info, ptr::null(), &mut vert)
            },
            vk_sys::SUCCESS
        );

        let mut frag = vk_sys::null_handle();
        info.code_size = FS.len() * 4;
        info.code = FS.as_ptr();
        assert_eq!(
            unsafe {
                dev_state
                    .fp
                    .create_shader_module(dev_state.dev, &info, ptr::null(), &mut frag)
            },
            vk_sys::SUCCESS
        );

        Self { vert, frag }
    }

    fn destroy(&mut self, dev_state: &DevState) {
        unsafe {
            // Note that these could have been destroyed
            // right after creating the pipeline.
            println!("destroy_shader_module() [vert]");
            dev_state
                .fp
                .destroy_shader_module(dev_state.dev, self.vert, ptr::null());
            println!("destroy_shader_module() [frag]");
            dev_state
                .fp
                .destroy_shader_module(dev_state.dev, self.frag, ptr::null());
        }
    }
}

// Pipeline state.
#[derive(Debug)]
struct PlState {
    layout: PipelineLayout,
    pl: Pipeline,
}

impl PlState {
    // Creates a graphics pipeline with a single color attachment.
    fn new(
        dev_state: &DevState,
        sc_state: &ScState,
        pass_state: &PassState,
        desc_state: &DescState,
        shd_state: &ShdState,
    ) -> Self {
        let layout_info = PipelineLayoutCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            set_layout_count: 1,
            set_layouts: &desc_state.set_layout,
            push_constant_range_count: 0,
            push_constant_ranges: ptr::null(),
        };

        let mut layout = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state.fp.create_pipeline_layout(
                    dev_state.dev,
                    &layout_info,
                    ptr::null(),
                    &mut layout,
                )
            },
            vk_sys::SUCCESS
        );

        let stages = [
            PipelineShaderStageCreateInfo {
                s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                next: ptr::null(),
                flags: 0,
                stage: vk_sys::SHADER_STAGE_VERTEX_BIT,
                module: shd_state.vert,
                name: c"main".as_ptr(),
                specialization_info: ptr::null(),
            },
            PipelineShaderStageCreateInfo {
                s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO,
                next: ptr::null(),
                flags: 0,
                stage: vk_sys::SHADER_STAGE_FRAGMENT_BIT,
                module: shd_state.frag,
                name: c"main".as_ptr(),
                specialization_info: ptr::null(),
            },
        ];

        let vertex_bindings = [
            VertexInputBindingDescription {
                binding: 0,
                stride: 12,
                input_rate: vk_sys::VERTEX_INPUT_RATE_VERTEX,
            },
            VertexInputBindingDescription {
                binding: 1,
                stride: 16,
                input_rate: vk_sys::VERTEX_INPUT_RATE_VERTEX,
            },
        ];
        let vertex_attributes = [
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: vk_sys::FORMAT_R32G32B32_SFLOAT,
                offset: 0,
            },
            VertexInputAttributeDescription {
                location: 1,
                binding: 1,
                format: vk_sys::FORMAT_R32G32B32A32_SFLOAT,
                offset: 0,
            },
        ];
        let vertex_input = PipelineVertexInputStateCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            vertex_binding_description_count: 2,
            vertex_binding_descriptions: vertex_bindings.as_ptr(),
            vertex_attribute_description_count: 2,
            vertex_attribute_descriptions: vertex_attributes.as_ptr(),
        };

        let input_assembly = PipelineInputAssemblyStateCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            topology: vk_sys::PRIMITIVE_TOPOLOGY_TRIANGLE_LIST,
            primitive_restart_enable: vk_sys::FALSE,
        };

        let viewport = PipelineViewportStateCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            viewport_count: 1,
            viewports: &Viewport {
                x: 0.0,
                y: 0.0,
                width: sc_state.extent.width as f32,
                height: sc_state.extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            },
            scissor_count: 1,
            scissors: &Rect2d {
                offset: Offset2d { x: 0, y: 0 },
                extent: sc_state.extent,
            },
        };

        let rasterization = PipelineRasterizationStateCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            depth_clamp_enable: vk_sys::FALSE,
            rasterizer_discard_enable: vk_sys::FALSE,
            polygon_mode: vk_sys::POLYGON_MODE_FILL,
            cull_mode: vk_sys::CULL_MODE_NONE,
            front_face: vk_sys::FRONT_FACE_COUNTER_CLOCKWISE,
            depth_bias_enable: vk_sys::FALSE,
            depth_bias_constant_factor: 0.0,
            depth_bias_clamp: 0.0,
            depth_bias_slope_factor: 0.0,
            line_width: 1.0,
        };

        let multisample = PipelineMultisampleStateCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            rasterization_samples: vk_sys::SAMPLE_COUNT_1_BIT,
            sample_shading_enable: vk_sys::FALSE,
            min_sample_shading: 0.0,
            sample_mask: ptr::null(),
            alpha_to_coverage_enable: vk_sys::FALSE,
            alpha_to_one_enable: vk_sys::FALSE,
        };

        let color_blend = PipelineColorBlendStateCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            logic_op_enable: vk_sys::FALSE,
            logic_op: vk_sys::LOGIC_OP_CLEAR,
            attachment_count: 1,
            attachments: &PipelineColorBlendAttachmentState {
                blend_enable: vk_sys::TRUE,
                src_color_blend_factor: vk_sys::BLEND_FACTOR_ONE,
                dst_color_blend_factor: vk_sys::BLEND_FACTOR_ZERO,
                color_blend_op: vk_sys::BLEND_OP_ADD,
                src_alpha_blend_factor: vk_sys::BLEND_FACTOR_ONE,
                dst_alpha_blend_factor: vk_sys::BLEND_FACTOR_ZERO,
                alpha_blend_op: vk_sys::BLEND_OP_ADD,
                color_write_mask: vk_sys::COLOR_COMPONENT_R_BIT
                    | vk_sys::COLOR_COMPONENT_G_BIT
                    | vk_sys::COLOR_COMPONENT_B_BIT
                    | vk_sys::COLOR_COMPONENT_A_BIT,
            },
            blend_constants: [1.0; 4],
        };

        let pl_info = GraphicsPipelineCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            stage_count: stages.len() as u32,
            stages: stages.as_ptr(),
            vertex_input_state: &vertex_input,
            input_assembly_state: &input_assembly,
            tessellation_state: ptr::null(),
            viewport_state: &viewport,
            rasterization_state: &rasterization,
            multisample_state: &multisample,
            depth_stencil_state: ptr::null(),
            color_blend_state: &color_blend,
            dynamic_state: ptr::null(),
            layout,
            render_pass: pass_state.pass,
            subpass: 0,
            base_pipeline_handle: vk_sys::null_handle(),
            base_pipeline_index: -1,
        };

        let mut pl = vk_sys::null_handle();
        assert_eq!(
            unsafe {
                dev_state.fp.create_graphics_pipelines(
                    dev_state.dev,
                    vk_sys::null_handle(),
                    1,
                    &pl_info,
                    ptr::null(),
                    &mut pl,
                )
            },
            vk_sys::SUCCESS
        );

        Self { layout, pl }
    }

    fn destroy(&mut self, dev_state: &DevState) {
        unsafe {
            println!("destroy_pipeline_layout()");
            dev_state
                .fp
                .destroy_pipeline_layout(dev_state.dev, self.layout, ptr::null());
            println!("destroy_pipeline()");
            dev_state
                .fp
                .destroy_pipeline(dev_state.dev, self.pl, ptr::null());
        }
    }
}

const POSITIONS: [f32; 3 * 3] = [-1.0, 1.0, 0.5, 1.0, 1.0, 0.5, 0.0, -1.0, 0.5];

const COLORS: [f32; 3 * 4] = [1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0];

// #version 450 core
//
// layout(set = 0, binding = 0) uniform Ubuffer {
//     mat4 transform;
// } ubuffer;
//
// layout(location = 0) in vec3 position;
// layout(location = 1) in vec4 color;
//
// layout(location = 0) out Vertex {
//     vec4 color;
// } vertex;
//
// void main() {
//     gl_Position = ubuffer.transform * vec4(position, 1.0);
//     vertex.color = color;
// }
const VS: [u32; 261] = [
    0x07230203, 0x00010000, 0x0008000a, 0x0000002a, 0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x0009000f, 0x00000000, 0x00000004, 0x6e69616d, 0x00000000, 0x0000000d, 0x00000019, 0x00000025,
    0x00000027, 0x00050048, 0x0000000b, 0x00000000, 0x0000000b, 0x00000000, 0x00050048, 0x0000000b,
    0x00000001, 0x0000000b, 0x00000001, 0x00050048, 0x0000000b, 0x00000002, 0x0000000b, 0x00000003,
    0x00050048, 0x0000000b, 0x00000003, 0x0000000b, 0x00000004, 0x00030047, 0x0000000b, 0x00000002,
    0x00040048, 0x00000011, 0x00000000, 0x00000005, 0x00050048, 0x00000011, 0x00000000, 0x00000023,
    0x00000000, 0x00050048, 0x00000011, 0x00000000, 0x00000007, 0x00000010, 0x00030047, 0x00000011,
    0x00000002, 0x00040047, 0x00000013, 0x00000022, 0x00000000, 0x00040047, 0x00000013, 0x00000021,
    0x00000000, 0x00040047, 0x00000019, 0x0000001e, 0x00000000, 0x00030047, 0x00000023, 0x00000002,
    0x00040047, 0x00000025, 0x0000001e, 0x00000000, 0x00040047, 0x00000027, 0x0000001e, 0x00000001,
    0x00020013, 0x00000002, 0x00030021, 0x00000003, 0x00000002, 0x00030016, 0x00000006, 0x00000020,
    0x00040017, 0x00000007, 0x00000006, 0x00000004, 0x00040015, 0x00000008, 0x00000020, 0x00000000,
    0x0004002b, 0x00000008, 0x00000009, 0x00000001, 0x0004001c, 0x0000000a, 0x00000006, 0x00000009,
    0x0006001e, 0x0000000b, 0x00000007, 0x00000006, 0x0000000a, 0x0000000a, 0x00040020, 0x0000000c,
    0x00000003, 0x0000000b, 0x0004003b, 0x0000000c, 0x0000000d, 0x00000003, 0x00040015, 0x0000000e,
    0x00000020, 0x00000001, 0x0004002b, 0x0000000e, 0x0000000f, 0x00000000, 0x00040018, 0x00000010,
    0x00000007, 0x00000004, 0x0003001e, 0x00000011, 0x00000010, 0x00040020, 0x00000012, 0x00000002,
    0x00000011, 0x0004003b, 0x00000012, 0x00000013, 0x00000002, 0x00040020, 0x00000014, 0x00000002,
    0x00000010, 0x00040017, 0x00000017, 0x00000006, 0x00000003, 0x00040020, 0x00000018, 0x00000001,
    0x00000017, 0x0004003b, 0x00000018, 0x00000019, 0x00000001, 0x0004002b, 0x00000006, 0x0000001b,
    0x3f800000, 0x00040020, 0x00000021, 0x00000003, 0x00000007, 0x0003001e, 0x00000023, 0x00000007,
    0x00040020, 0x00000024, 0x00000003, 0x00000023, 0x0004003b, 0x00000024, 0x00000025, 0x00000003,
    0x00040020, 0x00000026, 0x00000001, 0x00000007, 0x0004003b, 0x00000026, 0x00000027, 0x00000001,
    0x00050036, 0x00000002, 0x00000004, 0x00000000, 0x00000003, 0x000200f8, 0x00000005, 0x00050041,
    0x00000014, 0x00000015, 0x00000013, 0x0000000f, 0x0004003d, 0x00000010, 0x00000016, 0x00000015,
    0x0004003d, 0x00000017, 0x0000001a, 0x00000019, 0x00050051, 0x00000006, 0x0000001c, 0x0000001a,
    0x00000000, 0x00050051, 0x00000006, 0x0000001d, 0x0000001a, 0x00000001, 0x00050051, 0x00000006,
    0x0000001e, 0x0000001a, 0x00000002, 0x00070050, 0x00000007, 0x0000001f, 0x0000001c, 0x0000001d,
    0x0000001e, 0x0000001b, 0x00050091, 0x00000007, 0x00000020, 0x00000016, 0x0000001f, 0x00050041,
    0x00000021, 0x00000022, 0x0000000d, 0x0000000f, 0x0003003e, 0x00000022, 0x00000020, 0x0004003d,
    0x00000007, 0x00000028, 0x00000027, 0x00050041, 0x00000021, 0x00000029, 0x00000025, 0x0000000f,
    0x0003003e, 0x00000029, 0x00000028, 0x000100fd, 0x00010038,
];

// #version 450 core
//
// layout(location = 0) in Vertex {
//     vec4 color;
// } vertex;
//
// layout(location = 0) out vec4 color;
//
// void main() {
//     color = vertex.color;
// }
const FS: [u32; 101] = [
    0x07230203, 0x00010000, 0x0008000a, 0x00000012, 0x00000000, 0x00020011, 0x00000001, 0x0006000b,
    0x00000001, 0x4c534c47, 0x6474732e, 0x3035342e, 0x00000000, 0x0003000e, 0x00000000, 0x00000001,
    0x0007000f, 0x00000004, 0x00000004, 0x6e69616d, 0x00000000, 0x00000009, 0x0000000c, 0x00030010,
    0x00000004, 0x00000007, 0x00040047, 0x00000009, 0x0000001e, 0x00000000, 0x00030047, 0x0000000a,
    0x00000002, 0x00040047, 0x0000000c, 0x0000001e, 0x00000000, 0x00020013, 0x00000002, 0x00030021,
    0x00000003, 0x00000002, 0x00030016, 0x00000006, 0x00000020, 0x00040017, 0x00000007, 0x00000006,
    0x00000004, 0x00040020, 0x00000008, 0x00000003, 0x00000007, 0x0004003b, 0x00000008, 0x00000009,
    0x00000003, 0x0003001e, 0x0000000a, 0x00000007, 0x00040020, 0x0000000b, 0x00000001, 0x0000000a,
    0x0004003b, 0x0000000b, 0x0000000c, 0x00000001, 0x00040015, 0x0000000d, 0x00000020, 0x00000001,
    0x0004002b, 0x0000000d, 0x0000000e, 0x00000000, 0x00040020, 0x0000000f, 0x00000001, 0x00000007,
    0x00050036, 0x00000002, 0x00000004, 0x00000000, 0x00000003, 0x000200f8, 0x00000005, 0x00050041,
    0x0000000f, 0x00000010, 0x0000000c, 0x0000000e, 0x0004003d, 0x00000007, 0x00000011, 0x00000010,
    0x0003003e, 0x00000009, 0x00000011, 0x000100fd, 0x00010038,
];

#[cfg(target_os = "linux")]
mod plat {
    use std::ffi::{c_char, c_void, CStr};
    use std::pin::Pin;
    use std::ptr;

    use wl_sys::{
        self, Compositor, Display, Registry, RegistryListener, Surface, Toplevel, ToplevelListener,
        WmBase, WmBaseListener, XdgSurface, XdgSurfaceListener,
    };

    pub static mut DISPLAY: *mut Display = ptr::null_mut();
    pub static mut SURFACE: *mut Surface = ptr::null_mut();
    pub static mut XDG_SURFACE: *mut XdgSurface = ptr::null_mut();
    pub static mut TOPLEVEL: *mut Toplevel = ptr::null_mut();
    static mut QUIT: bool = false;

    pub fn init() {
        unsafe {
            wl_sys::init().unwrap();
            DISPLAY = wl_sys::display_connect(ptr::null());
            assert!(!DISPLAY.is_null());
            let global = bind(DISPLAY);
            SURFACE = global.create_surface();
            assert!(!SURFACE.is_null());
            global.set_wm();
            XDG_SURFACE = global.get_xdg_surface(SURFACE);
            assert!(!XDG_SURFACE.is_null());
            TOPLEVEL = get_toplevel(XDG_SURFACE);
            assert!(!TOPLEVEL.is_null());
            wl_sys::display_flush(DISPLAY);
        }
    }

    pub fn fini() {
        unsafe {
            wl_sys::display_disconnect(DISPLAY);
        }
        wl_sys::fini();
    }

    pub fn poll() {
        unsafe {
            wl_sys::display_dispatch_pending(DISPLAY);
        }
    }

    pub fn quit() -> bool {
        unsafe { QUIT }
    }

    fn get_registry(display: *mut Display) -> *mut Registry {
        unsafe {
            let registry = wl_sys::display_get_registry(display);
            assert!(!registry.is_null());
            registry
        }
    }

    fn bind(display: *mut Display) -> Pin<Box<Global>> {
        unsafe {
            let registry = get_registry(display);
            let mut global = Box::pin(Global {
                compositor: (ptr::null_mut(), u32::MAX),
                wm_base: (ptr::null_mut(), u32::MAX),
            });
            assert_eq!(
                wl_sys::registry_add_listener(
                    registry,
                    &REGISTRY_LISTENER,
                    &mut *global as *mut _ as *mut _
                ),
                0
            );
            wl_sys::display_roundtrip(display);
            assert!(!global.compositor.0.is_null());
            assert!(!global.wm_base.0.is_null());
            global
        }
    }

    struct Global {
        compositor: (*mut Compositor, u32),
        wm_base: (*mut WmBase, u32),
    }

    impl Global {
        fn create_surface(&self) -> *mut Surface {
            unsafe {
                let surface = wl_sys::compositor_create_surface(self.compositor.0);
                assert!(!surface.is_null());
                surface
            }
        }

        fn set_wm(&self) {
            unsafe {
                assert_eq!(
                    wl_sys::wm_base_add_listener(
                        self.wm_base.0,
                        &WM_BASE_LISTENER,
                        ptr::null_mut(),
                    ),
                    0
                );
            }
        }

        fn get_xdg_surface(&self, surface: *mut Surface) -> *mut XdgSurface {
            unsafe {
                let xdg_surface = wl_sys::wm_base_get_xdg_surface(self.wm_base.0, surface);
                assert!(!xdg_surface.is_null());
                assert_eq!(
                    wl_sys::xdg_surface_add_listener(
                        xdg_surface,
                        &XDG_SURFACE_LISTENER,
                        ptr::null_mut(),
                    ),
                    0
                );
                xdg_surface
            }
        }
    }

    fn get_toplevel(xdg_surface: *mut XdgSurface) -> *mut Toplevel {
        unsafe {
            let toplevel = wl_sys::xdg_surface_get_toplevel(xdg_surface);
            assert!(!toplevel.is_null());
            assert_eq!(
                wl_sys::toplevel_add_listener(toplevel, &TOPLEVEL_LISTENER, ptr::null_mut()),
                0
            );
            toplevel
        }
    }

    static REGISTRY_LISTENER: RegistryListener = RegistryListener {
        global: rty_global,
        global_remove: rty_global_remove,
    };

    unsafe extern "C" fn rty_global(
        data: *mut c_void,
        registry: *mut Registry,
        name: u32,
        interface: *const c_char,
        version: u32,
    ) {
        let data: &mut Global = &mut *data.cast();
        match CStr::from_ptr(interface).to_str().unwrap() {
            "wl_compositor" => {
                let cpt =
                    wl_sys::registry_bind(registry, name, &wl_sys::COMPOSITOR_INTERFACE, version);
                assert!(!cpt.is_null());
                data.compositor = (cpt.cast(), name);
            }
            "xdg_wm_base" => {
                let wm = wl_sys::registry_bind(registry, name, &wl_sys::WM_BASE_INTERFACE, version);
                assert!(!wm.is_null());
                data.wm_base = (wm.cast(), name);
            }
            _ => (),
        }
    }

    unsafe extern "C" fn rty_global_remove(data: *mut c_void, _registry: *mut Registry, name: u32) {
        let data: &Global = &*data.cast();
        assert_ne!(name, data.compositor.1);
        assert_ne!(name, data.wm_base.1);
    }

    static WM_BASE_LISTENER: WmBaseListener = WmBaseListener { ping: wm_ping };

    unsafe extern "C" fn wm_ping(_data: *mut c_void, wm_base: *mut WmBase, serial: u32) {
        wl_sys::wm_base_pong(wm_base, serial);
    }

    static XDG_SURFACE_LISTENER: XdgSurfaceListener = XdgSurfaceListener {
        configure: xsf_configure,
    };

    unsafe extern "C" fn xsf_configure(
        _data: *mut c_void,
        xdg_surface: *mut XdgSurface,
        serial: u32,
    ) {
        wl_sys::xdg_surface_ack_configure(xdg_surface, serial);
    }

    static TOPLEVEL_LISTENER: ToplevelListener = ToplevelListener {
        configure: top_configure,
        close: top_close,
        configure_bounds: top_configure_bounds,
    };

    unsafe extern "C" fn top_configure(
        _data: *mut c_void,
        _toplevel: *mut Toplevel,
        _width: i32,
        _height: i32,
        _states: *mut c_void,
    ) {
    }

    unsafe extern "C" fn top_close(_data: *mut c_void, _toplevel: *mut Toplevel) {
        QUIT = true;
    }

    unsafe extern "C" fn top_configure_bounds(
        _data: *mut c_void,
        _toplevel: *mut Toplevel,
        _width: i32,
        _height: i32,
    ) {
    }
}

#[cfg(windows)]
mod plat {
    compile_error!("not implemented");
}
