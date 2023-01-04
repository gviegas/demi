// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::io;
use std::ptr;

use vk_sys::{
    Sampler, SamplerCreateInfo, BORDER_COLOR_FLOAT_OPAQUE_BLACK, COMPARE_OP_NEVER,
    ERROR_OUT_OF_DEVICE_MEMORY, ERROR_OUT_OF_HOST_MEMORY, FALSE,
    STRUCTURE_TYPE_SAMPLER_CREATE_INFO, SUCCESS, TRUE,
};

use crate::gpu::vk::conv;
use crate::gpu::vk::Impl;
use crate::gpu::SplrOptions;

/// Sampler implementation.
#[derive(Debug)]
pub(super) struct SplrImpl {
    splr: Sampler,
}

impl SplrImpl {
    /// Creates a [`vk_sys::Sampler`].
    fn create_sampler(imp: &Impl, info: &SamplerCreateInfo) -> io::Result<Sampler> {
        let mut splr = vk_sys::null_handle();
        match unsafe {
            imp.dev_fp
                .create_sampler(imp.dev, info, ptr::null(), &mut splr)
        } {
            SUCCESS => Ok(splr),
            ERROR_OUT_OF_DEVICE_MEMORY | ERROR_OUT_OF_HOST_MEMORY => {
                Err(io::Error::from(io::ErrorKind::OutOfMemory))
            }
            _ => Err(io::Error::from(io::ErrorKind::Other)),
        }
    }

    /// Destroys a [`vk_sys::Sampler`].
    fn destroy_sampler(imp: &Impl, splr: Sampler) {
        unsafe {
            imp.dev_fp.destroy_sampler(imp.dev, splr, ptr::null());
        }
    }

    /// Creates a new `SplrImpl`.
    ///
    /// The value of `options.compare` is used to determine whether
    /// depth comparison is enabled or not.
    /// Image coordinates are assumed to be normalized. Integer formats
    /// are not supported at all.
    pub fn new(imp: &Impl, options: &SplrOptions) -> io::Result<Self> {
        let (min_filter, mipmap_mode, min_lod, max_lod) =
            conv::from_min_filter(options.min_filter.0, options.min_filter.1);
        let (compare_enable, compare_op) = if let Some(x) = options.compare {
            (TRUE, conv::from_compare_fn(x))
        } else {
            (FALSE, COMPARE_OP_NEVER)
        };
        // TODO: Consider exposing more sampler parameters
        // to crate users.
        let info = SamplerCreateInfo {
            s_type: STRUCTURE_TYPE_SAMPLER_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            mag_filter: conv::from_mag_filter(options.mag_filter),
            min_filter,
            mipmap_mode,
            address_mode_u: conv::from_wrap_mode(options.u_wrap),
            address_mode_v: conv::from_wrap_mode(options.v_wrap),
            address_mode_w: conv::from_wrap_mode(options.w_wrap),
            mip_lod_bias: 0.0,
            anisotropy_enable: FALSE,
            max_anisotropy: 1.0,
            compare_enable,
            compare_op,
            min_lod,
            max_lod,
            border_color: BORDER_COLOR_FLOAT_OPAQUE_BLACK,
            unnormalized_coordinates: FALSE,
        };
        Ok(Self {
            splr: Self::create_sampler(imp, &info)?,
        })
    }
}
