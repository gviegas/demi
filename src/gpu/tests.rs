use crate::gpu::{self, IMPL};

#[test]
#[ignore]
fn init_and_shutdown() {
    assert!(unsafe { IMPL.is_none() });
    gpu::init();
    assert!(unsafe { IMPL.is_some() });
    gpu::shutdown();
    assert!(unsafe { IMPL.is_none() });
}
