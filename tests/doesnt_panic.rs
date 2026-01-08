//! Test nounwind with code that doesn't panic.
//!
//! Can't use integration test with panicking code,
//! at least until the test module supports `panic = "abort"`.

#[test]
fn nopanic_closure() {
    nounwind::abort_unwind(|| {
        println!("shouldn't panic");
        assert_eq!(3 + 4, 7);
    });
    assert_eq!(
        nounwind::abort_unwind(|| [1, 7, 2].into_iter().sum::<i32>()),
        10
    );
}

#[cfg(feature = "macros")]
#[nounwind::nounwind]
#[test]
fn nopanic_macro() {
    println!("shouldn't panic");
    let x = [1, 7, 2];
    let mut res = 7;
    for val in x {
        res += val;
    }
    println!("res {res}");
}
