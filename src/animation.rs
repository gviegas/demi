// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Key-frame animation.

use std::alloc::Layout;
use std::io::{self, Read};

/// Animation.
#[derive(Debug)]
pub struct Animation {
    // TODO
}

/// Key-frame i/o data.
#[derive(Debug)]
enum KfData {
    SecondsF64(Box<[f64]>),
    SecondsF32(Box<[f32]>),
    TranslationF64x3(Box<[[f64; 3]]>),
    TranslationF32x3(Box<[[f32; 3]]>),
    RotationF32x4(Box<[[f32; 4]]>),
    RotationI16x4(Box<[[i16; 4]]>),
    RotationU16x4(Box<[[u16; 4]]>),
    RotationI8x4(Box<[[i8; 4]]>),
    RotationU8x4(Box<[[u8; 4]]>),
    ScaleF32x4(Box<[[f32; 4]]>),
    WeightsF64(Box<[f64]>),
    WeightsF32(Box<[f32]>),
    WeightsI16(Box<[i16]>),
    WeightsU16(Box<[u16]>),
    WeightsI8(Box<[i8]>),
    WeightsU8(Box<[u8]>),
}

/// Key-frame input types.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum KfInput {
    SecondsF64,
    SecondsF32,
}

impl KfInput {
    /// Returns the [`Layout`] of the [`KfInput`].
    pub const fn layout(&self) -> Layout {
        match self {
            KfInput::SecondsF64 => Layout::new::<f64>(),
            KfInput::SecondsF32 => Layout::new::<f32>(),
        }
    }
}

/// Key-frame output types.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum KfOutput {
    TranslationF64x3,
    TranslationF32x3,
    RotationF32x4,
    RotationI16x4,
    RotationU16x4,
    RotationI8x4,
    RotationU8x4,
    ScaleF32x4,
    WeightsF64,
    WeightsF32,
    WeightsI16,
    WeightsU16,
    WeightsI8,
    WeightsU8,
}

impl KfOutput {
    /// Returns the [`Layout`] of the [`KfOutput`].
    pub const fn layout(&self) -> Layout {
        match self {
            KfOutput::TranslationF64x3 => Layout::new::<[f64; 3]>(),
            KfOutput::TranslationF32x3 => Layout::new::<[f32; 3]>(),
            KfOutput::RotationF32x4 | KfOutput::ScaleF32x4 => Layout::new::<[f32; 4]>(),
            KfOutput::RotationI16x4 | KfOutput::RotationU16x4 => Layout::new::<[i16; 4]>(),
            KfOutput::RotationI8x4 | KfOutput::RotationU8x4 => Layout::new::<[i8; 4]>(),
            KfOutput::WeightsF64 => Layout::new::<f64>(),
            KfOutput::WeightsF32 => Layout::new::<f32>(),
            KfOutput::WeightsI16 | KfOutput::WeightsU16 => Layout::new::<i16>(),
            KfOutput::WeightsI8 | KfOutput::WeightsU8 => Layout::new::<i8>(),
        }
    }
}

/// Interpolation methods.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Interpolation {
    Step,
    Linear,
    CubicSpline,
}

//pub struct Action {
//    // TODO
//}

/// Animation builder.
pub struct Builder {
    // TODO
}

#[allow(unused_variables)] // TODO
#[allow(unused_mut)] // TODO
impl Builder {
    /// Creates a new animation builder.
    pub fn new() -> Self {
        todo!();
    }

    /// Pushes an input source.
    ///
    /// The order which this method is called defines the slot
    /// occupied by the pushed input source.
    /// The first pushed input occupies the slot `0`.
    pub fn push_input<T: Read>(
        &mut self,
        mut reader: T,
        input_type: KfInput,
        sample_count: usize,
        stride: usize,
    ) -> io::Result<&mut Self> {
        todo!();
    }

    /// Pushes an output source.
    ///
    /// The order which this method is called defines the slot
    /// occupied by the pushed output source.
    /// The first pushed output occupies the slot `0`.
    pub fn push_output<T: Read>(
        &mut self,
        mut reader: T,
        output_type: KfOutput,
        sample_count: usize,
        stride: usize,
    ) -> io::Result<&mut Self> {
        todo!();
    }

    /// Pushes an action.
    ///
    /// It is not necessary to fill the referred i/o slots before
    /// calling this method.
    pub fn push_action(
        &mut self,
        method: Interpolation,
        input_slot: usize,
        output_slot: usize,
        name: &str,
    ) -> &mut Self {
        todo!();
    }

    /// Creates the animation.
    ///
    /// This method consumes all i/o slots and actions.
    pub fn create(&mut self) -> io::Result<Animation> {
        todo!();
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
