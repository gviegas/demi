#!cfg(unix)]

use std::mem;

#[test]
fn test_dl() {
    let so = match dl::Dl::new("libm.so.6", dl::NOW | dl::GLOBAL) {
        Ok(x) => x,
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    type Ceil = unsafe extern "C" fn(f64) -> f64;
    let ceil = match so.get("ceil") {
        Ok(x) => unsafe { mem::transmute::<_, Ceil>(x) },
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    // TODO: It would be nice if this resulted in a compilation error.
    //drop(so);

    let x = unsafe { ceil(1.01) };
    assert_eq!(x, 2.0);
}
