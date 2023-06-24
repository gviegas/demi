//! xgpu.

mod adapter;
pub use adapter::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter() {
        let adap = Adapter {};
        _ = adap.features();
        _ = adap.limits();
        _ = adap.info();
        _ = adap.request_device(/*...*/);
    }
}
