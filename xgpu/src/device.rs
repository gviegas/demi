//! GPU device.

use crate::{
    Buffer, BufferDescriptor, Result, SupportedFeatures, SupportedLimits, Texture,
    TextureDescriptor,
};

pub struct Device {
    // TODO
}

impl Device {
    pub fn features(&self) -> &SupportedFeatures {
        panic!("not yet implemented");
    }

    pub fn limits(&self) -> &SupportedLimits {
        panic!("not yet implemented");
    }

    pub fn queue(&self) /* -> ?Queue */
    {
        panic!("not yet implemented");
    }

    // TODO: Maybe use interior mutability for the following

    pub fn create_buffer(&mut self, _desc: &BufferDescriptor) -> Result<Buffer> {
        panic!("not yet implemented");
    }

    pub fn create_texture(&mut self, _desc: &TextureDescriptor) -> Result<Texture> {
        panic!("not yet implemented");
    }

    pub fn create_sampler(&mut self /*, desc: SamplerDescriptor */) /* -> Sampler */
    {
        panic!("not yet implemented");
    }

    pub fn create_bind_group_layout(&mut self /*, desc: BindGroupLayoutDescriptor */)
    /* -> BindGroupLayout */
    {
        panic!("not yet implemented");
    }

    pub fn create_pipeline_layout(&mut self /*, desc: PipelineLayoutDescriptor */)
    /* -> PipelineLayout */
    {
        panic!("not yet implemented");
    }

    pub fn create_bind_group(&mut self /*, desc: BindGroupDescriptor */) /* -> BindGroup */
    {
        panic!("not yet implemented");
    }

    pub fn create_shader_module(&mut self /*, desc: ShaderModuleDescriptor */) /* -> ShaderModule */
    {
        panic!("not yet implemented");
    }

    // async
    pub fn create_compute_pipeline(&mut self /*, desc: ComputePipelineDescriptor */)
    /* -> ComputePipeline */
    {
        panic!("not yet implemented");
    }

    // async
    pub fn create_render_pipeline(&mut self /*, desc: RenderPipelineDescriptor */)
    /* -> RenderPipeline */
    {
        panic!("not yet implemented");
    }

    pub fn create_command_encoder(&mut self /*, desc: CommandEncoderDescriptor */)
    /* -> CommandEncoder */
    {
        panic!("not yet implemented");
    }

    pub fn create_render_bundle_encoder(&mut self /*, desc: RenderBundleEncoderDescriptor */)
    /* -> RenderBundleEncoder */
    {
        panic!("not yet implemented");
    }

    pub fn create_query_set(&mut self /*, desc: QuerySetDescriptor */) /* -> QuerySet */
    {
        panic!("not yet implemented");
    }
}

// TODO
pub struct DeviceDescriptor {
    // required_features: ...,
    // required_limits: ...,
    // queue_descriptor: ...,
}
