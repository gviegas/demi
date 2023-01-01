// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::io;

/// Sampler.
pub struct Sampler {
    // TODO
}

/// Sampler wrapping modes.
#[derive(Copy, Clone, Debug)]
pub enum Wrap {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
}

/// Sampler filters.
#[derive(Copy, Clone, Debug)]
pub enum Filter {
    Nearest,
    Linear,
}

/// Comparison functions.
#[derive(Copy, Clone, Debug)]
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
    // TODO
}

#[allow(unused_variables)] // TODO
impl Builder {
    pub fn new() -> Self {
        todo!();
    }

    pub fn set_u_wrap(&mut self, wrap: Wrap) -> &mut Self {
        todo!();
    }

    pub fn set_v_wrap(&mut self, wrap: Wrap) -> &mut Self {
        todo!();
    }

    pub fn set_w_wrap(&mut self, wrap: Wrap) -> &mut Self {
        todo!();
    }

    pub fn set_mag_filter(&mut self, filter: Filter) -> &mut Self {
        todo!();
    }

    pub fn set_min_filter(&mut self, filter: Filter, mipmap: Option<Filter>) -> &mut Self {
        todo!();
    }

    pub fn create(&mut self) -> io::Result<Sampler> {
        todo!();
    }

    pub fn create_shadow(&mut self, compare: Compare) -> io::Result<Sampler> {
        todo!();
    }
}
