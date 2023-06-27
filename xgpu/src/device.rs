//! GPU device.

use std::ops::RangeBounds;

use crate::{
    BindGroup, BindGroupDescriptor, BindGroupLayout, BindGroupLayoutDescriptor, Buffer,
    BufferDescriptor, ComputePipeline, ComputePipelineDescriptor, PipelineLayout,
    PipelineLayoutDescriptor, RenderPipeline, RenderPipelineDescriptor, Result, Sampler,
    SamplerDescriptor, ShaderModule, ShaderModuleDescriptor, SupportedFeatures, SupportedLimits,
    Texture, TextureDescriptor,
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

    pub fn create_sampler<T>(&mut self, _desc: &SamplerDescriptor<T>) -> Result<Sampler>
    where
        T: RangeBounds<f32>,
    {
        panic!("not yet implemented");
    }

    pub fn create_bind_group_layout(
        &mut self,
        _desc: &BindGroupLayoutDescriptor,
    ) -> Result<BindGroupLayout> {
        panic!("not yet implemented");
    }

    pub fn create_pipeline_layout(
        &mut self,
        _desc: &PipelineLayoutDescriptor,
    ) -> Result<PipelineLayout> {
        panic!("not yet implemented");
    }

    pub fn create_bind_group(&mut self, _desc: &BindGroupDescriptor) -> Result<BindGroup> {
        panic!("not yet implemented");
    }

    pub fn create_shader_module(&mut self, _desc: &ShaderModuleDescriptor) -> Result<ShaderModule> {
        panic!("not yet implemented");
    }

    // async
    pub fn create_compute_pipeline(
        &mut self,
        _desc: &ComputePipelineDescriptor,
    ) -> Result<ComputePipeline> {
        panic!("not yet implemented");
    }

    // async
    pub fn create_render_pipeline(
        &mut self,
        _desc: &RenderPipelineDescriptor,
    ) -> Result<RenderPipeline> {
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
