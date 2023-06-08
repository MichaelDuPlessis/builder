use std::collections::HashMap;

use generic_builder::Builder;

struct MyStuct {
    something: u8.
}

#[derive(Builder, Debug)]
pub struct Command<T> {
    executable: String,
    #[auto(arg, push)]
    args: Vec<T>,
    env: Vec<String>,
    current_dir: Option<String>,
    #[manual(t, insert, u8, String)]
    test: HashMap<u8, String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = Command::<MyStuct>::builder();

    Ok(())
}
