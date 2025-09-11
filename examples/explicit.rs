pub fn main() {
    nounwind::abort_unwind(
        #[inline(always)]
        || panic!("foo"),
    )
}
