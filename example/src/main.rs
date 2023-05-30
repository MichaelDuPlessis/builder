use generic_builder::Builder;

#[derive(Builder)]
struct Test {
    param1: Option<String>,
    param2: String,
}

fn main() {
    println!("Hello, world!");
}
