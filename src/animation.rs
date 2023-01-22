// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Key-frame animation.

use std::io::{self, Read};

/// Animation.
#[derive(Debug)]
pub struct Animation {
    // TODO
}

/// Key-frame input types.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum KfInput {
    SecondsF64,
    SecondsF32,
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
