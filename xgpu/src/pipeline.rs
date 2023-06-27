//! GPU shader/pipelines.

use std::ops::BitOr;

use crate::{BindGroupLayout, CompareFunction, PipelineLayout, Result, TextureFormat};

pub struct ShaderModule {
    // TODO
}

pub struct ShaderModuleDescriptor<'a> {
    pub code: &'a [u8],
}

pub struct ProgrammableStage<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: String,
    pub constants: Vec<PipelineConstant>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PipelineConstant {
    pub id: u32,
    pub value: PipelineConstantValue,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PipelineConstantValue {
    Uint32(u32),
    Sint32(i32),
    Float32(f32),
    // TODO
}

pub struct ComputePipeline {
    // TODO
}

impl ComputePipeline {
    // TODO: Maybe return a slice instead since it can't fail.
    pub fn bind_group_layout(&self, _index: u32) -> Result<&BindGroupLayout> {
        panic!("not yet implemented");
    }
}

// TODO: Will have to bind lifetimes to the created pipeline.
// It may be better to make fields copyable or reference-counted.
pub struct ComputePipelineDescriptor<'a, 'b> {
    pub layout: &'a PipelineLayout,
    pub compute: ProgrammableStage<'b>,
}

pub struct RenderPipeline {
    // TODO
}

impl RenderPipeline {
    // TODO: <see compute pipeline>
    pub fn bind_group_layout(&self, _index: u32) -> Result<&BindGroupLayout> {
        panic!("not yet implemented");
    }
}

// TODO: <see compute pipeline descriptor>
pub struct RenderPipelineDescriptor<'a, 'b, 'c> {
    pub layout: &'a PipelineLayout,
    pub vertex: VertexState<'b>,
    pub primitive: PrimitiveState,
    pub depth_stencil: DepthStencilState,
    pub multisample: MultisampleState,
    pub fragment: FragmentState<'c>,
}

pub struct VertexState<'a> {
    pub vertex: ProgrammableStage<'a>,
    pub buffers: Vec<VertexBufferLayout>,
}

pub struct VertexBufferLayout {
    pub array_stride: u64,
    pub step_mode: VertexStepMode,
    pub attributes: Vec<VertexAttribute>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VertexStepMode {
    Vertex,
    Instance,
}

pub struct VertexAttribute {
    pub format: VertexFormat,
    pub offset: u64,
    pub shader_location: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VertexFormat {
    Uint8x2,
    Uint8x4,
    Sint8x2,
    Sint8x4,

    Unorm8x2,
    Unorm8x4,
    Snorm8x2,
    Snorm8x4,

    Uint16x2,
    Uint16x4,
    Sint16x2,
    Sint16x4,

    Unorm16x2,
    Unorm16x4,
    Snorm16x2,
    Snorm16x4,

    Float16x2,
    Float16x4,
    Float32,
    Float32x2,
    Float32x3,
    Float32x4,

    Uint32,
    Uint32x2,
    Uint32x3,
    Uint32x4,
    Sint32,
    Sint32x2,
    Sint32x3,
    Sint32x4,
}

pub struct PrimitiveState {
    pub topology: PrimitiveTopology,
    pub strip_index_format: Option<IndexFormat>,
    pub front_face: FrontFace,
    pub cull_mode: CullMode,
    pub unclipped_depth: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IndexFormat {
    Uint16,
    Uint32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FrontFace {
    Ccw,
    Cw,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CullMode {
    None,
    Front,
    Back,
}

pub struct DepthStencilState {
    pub format: TextureFormat,
    pub depth_write_enabled: bool,
    pub depth_compare: CompareFunction,
    pub stencil_front: StencilFaceState,
    pub stencil_back: StencilFaceState,
    pub stencil_read_mask: u32,
    pub stencil_write_mask: u32,
    pub depth_bias: i32,
    pub depth_bias_slope_scale: f32,
    pub depth_bias_clamp: f32,
}

pub struct StencilFaceState {
    pub compare: CompareFunction,
    pub fail_op: StencilOperation,
    pub depth_fail_op: StencilOperation,
    pub pass_op: StencilOperation,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StencilOperation {
    Keep,
    Zero,
    Replace,
    Invert,
    IncrementClamp,
    DecrementClamp,
    IncrementWrap,
    DecrementWrap,
}

pub struct MultisampleState {
    pub count: u32,
    pub mask: u32,
    pub alpha_to_coverage_enabled: bool,
}

pub struct FragmentState<'a> {
    pub fragment: ProgrammableStage<'a>,
    // TODO: The length should be constant.
    pub targets: Vec<ColorTargetState>,
}

pub struct ColorTargetState {
    pub format: TextureFormat,
    pub blend: BlendState,
    pub write_mask: ColorWriteFlags,
}

pub struct BlendState {
    pub color: BlendComponent,
    pub alpha: BlendComponent,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ColorWrite {
    Red = 0x1,
    Green = 0x2,
    Blue = 0x4,
    Alpha = 0x8,
    All = 0xF,
}

impl BitOr for ColorWrite {
    type Output = ColorWriteFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        ColorWriteFlags(self as u16 | rhs as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ColorWriteFlags(u16);

impl ColorWriteFlags {
    pub fn is_set(self, color: ColorWrite) -> bool {
        self.0 & color as u16 != 0
    }
}

impl BitOr<ColorWrite> for ColorWriteFlags {
    type Output = Self;

    fn bitor(self, rhs: ColorWrite) -> Self::Output {
        Self(self.0 | rhs as u16)
    }
}

impl From<ColorWrite> for ColorWriteFlags {
    fn from(value: ColorWrite) -> Self {
        Self(value as u16)
    }
}

pub struct BlendComponent {
    pub operation: BlendOperation,
    pub src_factor: BlendFactor,
    pub dst_factor: BlendFactor,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlendOperation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlendFactor {
    Zero,
    One,
    Src,
    OneMinusSrc,
    SrcAlpha,
    OneMinusSrcAlpha,
    Dst,
    OneMinusDst,
    DstAlpha,
    OneMinusDstAlpha,
    SrcAlphaSaturated,
    Constant,
    OneMinusConstant,
}
