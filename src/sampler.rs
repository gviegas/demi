// Copyright 2022 Gustavo C. Viegas. All rights reserved.

//! Texture sampler.

use std::io;

use crate::gpu::{self, SplrId, SplrOptions};

/// Sampler.
#[derive(Debug)]
pub struct Sampler {
    options: SplrOptions,
    gid: SplrId,
}

impl Drop for Sampler {
    fn drop(&mut self) {
        gpu::drop_sampler(&mut self.gid)
    }
}

/// Sampler wrapping modes.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Wrap {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

/// Sampler filters.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Filter {
    Nearest,
    Linear,
}

/// Comparison functions.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Compare {
    Never,
    Less,
    LessEqual,
    Equal,
    NotEqual,
    GreaterEqual,
    Greater,
    Always,
}

/// Sampler builder.
pub struct Builder {
    options: SplrOptions,
}

impl Builder {
    /// Creates a new sampler builder.
    pub fn new() -> Self {
        Self {
            options: SplrOptions {
                u_wrap: Wrap::Repeat,
                v_wrap: Wrap::Repeat,
                w_wrap: Wrap::Repeat,
                mag_filter: Filter::Nearest,
                min_filter: (Filter::Nearest, Some(Filter::Nearest)),
                compare: None,
            },
        }
    }

    /// Sets the wrapping mode of the `u` coordinates.
    ///
    /// This value need not be set. It defaults to [`Wrap::Repeat`].
    pub fn set_u_wrap(&mut self, wrap: Wrap) -> &mut Self {
        self.options.u_wrap = wrap;
        self
    }

    /// Sets the wrapping mode of the `v` coordinates.
    ///
    /// This value need not be set. It defaults to [`Wrap::Repeat`].
    pub fn set_v_wrap(&mut self, wrap: Wrap) -> &mut Self {
        self.options.v_wrap = wrap;
        self
    }

    /// Sets the wrapping mode of the `w` coordinates.
    ///
    /// This value need not be set. It defaults to [`Wrap::Repeat`].
    pub fn set_w_wrap(&mut self, wrap: Wrap) -> &mut Self {
        self.options.w_wrap = wrap;
        self
    }

    /// Sets the magnification filter.
    ///
    /// This value need not be set. It defaults to [`Filter::Nearest`].
    pub fn set_mag_filter(&mut self, filter: Filter) -> &mut Self {
        self.options.mag_filter = filter;
        self
    }

    /// Sets the minification filter.
    ///
    /// This value need not be set. It defaults to [`Filter::Nearest`].
    pub fn set_min_filter(&mut self, filter: Filter, mipmap: Option<Filter>) -> &mut Self {
        self.options.min_filter = (filter, mipmap);
        self
    }

    /// Creates a sampler.
    pub fn create(&mut self) -> io::Result<Sampler> {
        Ok(Sampler {
            options: self.options,
            gid: gpu::create_sampler(&self.options)?,
        })
    }

    /// Creates a shadow sampler.
    pub fn create_shadow(&mut self, compare: Compare) -> io::Result<Sampler> {
        let mut options = self.options;
        options.compare = Some(compare);
        Ok(Sampler {
            options,
            gid: gpu::create_sampler(&options)?,
        })
    }
}
