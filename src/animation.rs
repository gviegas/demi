// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Key-frame animation.

use std::alloc::{self, Layout};
use std::io::{self, Read};
use std::slice;

/// Animation.
#[derive(Debug)]
pub struct Animation {
    actions: Vec<Action>,
    inputs: Vec<KfData>,
    outputs: Vec<KfData>,
}

impl Animation {
    /// Returns a reference to the animation's [`Action`]s.
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }

    /// Searches for an action matching a given name.
    ///
    /// It returns a tuple containing a reference to the
    /// [`Action`] itself and its index in the animation,
    /// or [`None`] if no action has the given `name`.
    ///
    /// NOTE: Action names need not be unique. This method
    /// iterates over `actions()` in order and returns the
    /// first match.
    pub fn action(&self, name: &str) -> Option<(&Action, usize)> {
        self.actions
            .iter()
            .enumerate()
            .find(|(_, x)| x.name == name)
            .map(|(i, x)| (x, i))
    }
}

/// Key-frame i/o data.
// TODO: Decode normalized integers at build time
// and only keep the floating-point variants.
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
    ScaleF32x3(Box<[[f32; 3]]>),
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
    ScaleF32x3,
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
            KfOutput::TranslationF32x3 | KfOutput::ScaleF32x3 => Layout::new::<[f32; 3]>(),
            KfOutput::RotationF32x4 => Layout::new::<[f32; 4]>(),
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

/// Animation's action.
///
/// This type describes the key-frame animation of a single
/// property by means of an i/o pair and a specific
/// [`Interpolation`] method.
#[derive(Debug)]
pub struct Action {
    method: Interpolation,
    input_slot: usize,
    output_slot: usize,
    name: String,
}

impl Action {
    /// Returns the action's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets the action's name.
    pub fn set_name(&mut self, name: &str) {
        self.name.clear();
        self.name.push_str(name);
    }
}

/// Animation builder.
pub struct Builder {
    actions: Vec<Action>,
    inputs: Vec<KfData>,
    outputs: Vec<KfData>,
}

impl Builder {
    /// Creates a new animation builder.
    pub fn new() -> Self {
        Self {
            actions: vec![],
            inputs: vec![],
            outputs: vec![],
        }
    }

    /// Pushes an input source.
    ///
    /// Data is assumed to be tightly packed as defined by
    /// `input_type.layout()`.
    ///
    /// The order which this method is called defines the slot
    /// occupied by the pushed input source.
    /// The first pushed input occupies the slot `0`.
    pub fn push_input<T: Read>(
        &mut self,
        mut reader: T,
        input_type: KfInput,
        sample_count: usize,
    ) -> io::Result<&mut Self> {
        if sample_count == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let layout = {
            let layout = input_type.layout();
            let (size, align) = (layout.size(), layout.align());
            Layout::from_size_align(size * sample_count, align).unwrap()
        };
        // TODO: Do we really need to clear this memory?
        // (notice the `slice::from_raw_parts_mut` call below).
        let ptr = unsafe { alloc::alloc_zeroed(layout) };
        ptr.is_null().then(|| alloc::handle_alloc_error(layout));
        // Ensure that the mutable slice's lifetime ends here.
        let res = unsafe { reader.read_exact(slice::from_raw_parts_mut(ptr, layout.size())) };
        if let Err(e) = res {
            unsafe {
                alloc::dealloc(ptr, layout);
            }
            Err(e)
        } else {
            let data = match input_type {
                KfInput::SecondsF64 => KfData::SecondsF64(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfInput::SecondsF32 => KfData::SecondsF32(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
            };
            self.inputs.push(data);
            Ok(self)
        }
    }

    /// Pushes an output source.
    ///
    /// Data is assumed to be tightly packed as defined by
    /// `output_type.layout()`.
    ///
    /// The order which this method is called defines the slot
    /// occupied by the pushed output source.
    /// The first pushed output occupies the slot `0`.
    pub fn push_output<T: Read>(
        &mut self,
        mut reader: T,
        output_type: KfOutput,
        sample_count: usize,
    ) -> io::Result<&mut Self> {
        if sample_count == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        let layout = {
            let layout = output_type.layout();
            let (size, align) = (layout.size(), layout.align());
            Layout::from_size_align(size * sample_count, align).unwrap()
        };
        // TODO: Do we really need to clear this memory?
        // (notice the `slice::from_raw_parts_mut` call below).
        let ptr = unsafe { alloc::alloc_zeroed(layout) };
        ptr.is_null().then(|| alloc::handle_alloc_error(layout));
        // Ensure that the mutable slice's lifetime ends here.
        let res = unsafe { reader.read_exact(slice::from_raw_parts_mut(ptr, layout.size())) };
        if let Err(e) = res {
            unsafe {
                alloc::dealloc(ptr, layout);
            }
            Err(e)
        } else {
            let data = match output_type {
                KfOutput::TranslationF64x3 => KfData::TranslationF64x3(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::TranslationF32x3 => KfData::TranslationF32x3(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::RotationF32x4 => KfData::RotationF32x4(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::RotationI16x4 => KfData::RotationI16x4(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::RotationU16x4 => KfData::RotationU16x4(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::RotationI8x4 => KfData::RotationI8x4(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::RotationU8x4 => KfData::RotationU8x4(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::ScaleF32x3 => KfData::ScaleF32x3(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::WeightsF64 => KfData::WeightsF64(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::WeightsF32 => KfData::WeightsF32(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::WeightsI16 => KfData::WeightsI16(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::WeightsU16 => KfData::WeightsU16(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::WeightsI8 => KfData::WeightsI8(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
                KfOutput::WeightsU8 => KfData::WeightsU8(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(ptr.cast(), sample_count))
                }),
            };
            self.outputs.push(data);
            Ok(self)
        }
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
        self.actions.push(Action {
            method,
            input_slot,
            output_slot,
            name: name.to_string(),
        });
        self
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
