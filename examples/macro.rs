use nounwind::nounwind;

pub fn main() {
    x(String::from("Foo bar baz").chars());
    failure();
}

#[nounwind]
pub fn x<T: IntoIterator<Item = char>>(y: T) -> u32 {
    y.into_iter().map(u32::from).sum()
}

#[nounwind]
pub fn failure() -> String {
    panic!("failure (will trigger abort)")
}
