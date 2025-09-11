use nounwind::nounwind;

pub fn main() {
    x(String::from("Foo bar baz").chars());
    Foo::failure();
}

#[nounwind]
pub fn x<T: IntoIterator<Item = char>>(y: T) -> u32 {
    y.into_iter().map(u32::from).sum()
}

struct Foo;

impl Foo {
    #[nounwind]
    pub fn failure() -> String {
        panic!("failure (will trigger abort)")
    }
}
