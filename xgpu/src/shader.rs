//! GPU shader.

use std::ops::BitOr;

pub struct ShaderModule {
    // TODO
}

pub struct ShaderModuleDescriptor<'a> {
    pub code: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ShaderStage {
    Vertex = 0x1,
    Fragment = 0x2,
    Compute = 0x4,
}

impl BitOr for ShaderStage {
    type Output = ShaderStageFlags;

    fn bitor(self, rhs: Self) -> Self::Output {
        ShaderStageFlags(self as u16 | rhs as u16)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ShaderStageFlags(u16);

impl ShaderStageFlags {
    pub fn is_set(self, stage: ShaderStage) -> bool {
        self.0 & stage as u16 != 0
    }
}

impl BitOr<ShaderStage> for ShaderStageFlags {
    type Output = Self;

    fn bitor(self, rhs: ShaderStage) -> Self::Output {
        Self(self.0 | rhs as u16)
    }
}

impl From<ShaderStage> for ShaderStageFlags {
    fn from(value: ShaderStage) -> Self {
        Self(value as u16)
    }
}

pub struct ProgrammableStage<'a> {
    pub module: &'a ShaderModule,
    pub entry_point: String,
    pub constants: Vec<PipelineConstant>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct PipelineConstant {
    // TODO: Should allow string IDs too.
    pub id: u32,
    pub value: PipelineConstantValue,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PipelineConstantValue {
    // TODO: `Float16`.
    Float32(f32),
    Sint32(i32),
    Uint32(u32),
    Bool(bool),
}
