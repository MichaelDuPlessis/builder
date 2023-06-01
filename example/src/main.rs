use generic_builder::Builder;

#[derive(Builder)]
struct Test {
    param1: Option<String>,
    param2: String,
    #[single(param3)]
    param3: Vec<u8>,
}

fn main() {
    println!("Hello, world!");
}
