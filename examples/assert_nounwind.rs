pub fn main() {
    let x = 7;
    nounwind::assert_nounwind!(x > 5, "should not trigger abort (x={x})");
    nounwind::assert_nounwind!(x > 6); // this should not trigger an abort
    nounwind::assert_nounwind!(x > 7, "this will trigger an abort");
    nounwind::unreachable_nounwind();
}
