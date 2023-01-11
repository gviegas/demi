// Copyright 2022 Gustavo C. Viegas. All rights reserved.

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
    pub fn new() -> Self {
        todo!();
    }

    pub fn set_input_count(&mut self, count: usize) -> &mut Self {
        todo!();
    }

    pub fn set_input(
        &mut self,
        slot: usize,
        input_type: KfInput,
        sample_count: usize,
        offset: usize,
        stride: usize,
    ) -> &mut Self {
        todo!();
    }

    pub fn read_input<T: Read>(&mut self, slot: usize, mut reader: T) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn set_output_count(&mut self, count: usize) -> &mut Self {
        todo!();
    }

    pub fn set_output(
        &mut self,
        slot: usize,
        output_type: KfOutput,
        sample_count: usize,
        offset: usize,
        stride: usize,
    ) -> &mut Self {
        todo!();
    }

    pub fn read_output<T: Read>(&mut self, slot: usize, mut reader: T) -> io::Result<&mut Self> {
        todo!();
    }

    pub fn push_action(
        &mut self,
        method: Interpolation,
        input_slot: usize,
        output_slot: usize,
        name: &str,
    ) -> &mut Self {
        todo!();
    }

    pub fn create(&mut self) -> io::Result<Animation> {
        todo!();
    }
}
