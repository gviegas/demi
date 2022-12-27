// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::gpu::vk::*;

#[test]
fn new() {
    let imp = Impl::_new().unwrap();
    assert!(!imp.inst.is_null());
    assert_eq!(0, vk_sys::api_version_variant(imp.inst_vers));
    assert!(!imp.phys_dev.is_null());
    assert!(!imp.dev.is_null());
    assert_eq!(0, vk_sys::api_version_variant(imp.dev_prop.api_version));
    assert!(!imp.queue.0.is_null());
    assert_eq!(TRUE, imp.feat.shader_uniform_buffer_array_dynamic_indexing);
    assert_eq!(TRUE, imp.feat.shader_sampled_image_array_dynamic_indexing);
    assert_eq!(TRUE, imp.feat.shader_storage_buffer_array_dynamic_indexing);
    assert_eq!(TRUE, imp.feat.shader_storage_image_array_dynamic_indexing);
    println!("{imp}");
    println!("{imp:#?}");
}
